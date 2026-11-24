//! Генератор QR-кода (ISO/IEC 18004): байтовый режим, уровень коррекции M, версии 1–9.
//!
//! Этого хватает на адрес сайта длиной до 180 байт. Модуль — чистая математика без обращения
//! к браузеру, поэтому целиком покрыт обычными `cargo test`; эталонные матрицы в
//! `tests/fixtures` получены утилитой `qrencode`.

use std::fmt;

/// Старшая поддерживаемая версия. До неё длина данных кодируется одним байтом.
const MAX_VERSION: u8 = 9;

/// Кодовых слов коррекции на блок для уровня M, по версиям 1–9.
const ECC_PER_BLOCK: [usize; MAX_VERSION as usize] = [10, 16, 26, 18, 24, 16, 18, 22, 22];
/// Число блоков коррекции для уровня M, по версиям 1–9.
const BLOCK_COUNT: [usize; MAX_VERSION as usize] = [1, 1, 1, 2, 2, 4, 4, 4, 5];
/// Биты уровня коррекции M в служебной информации о формате.
const FORMAT_LEVEL_M: u32 = 0b00;

const MASK_COUNT: u8 = 8;

#[derive(Debug, PartialEq, Eq)]
pub enum QrError {
    /// Данные не помещаются в версию 9 уровня M.
    TooLong { bytes: usize, capacity: usize },
}

impl fmt::Display for QrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QrError::TooLong { bytes, capacity } => {
                write!(f, "текст занимает {bytes} байт, а в QR-код помещается не больше {capacity}")
            }
        }
    }
}

impl std::error::Error for QrError {}

/// Готовый QR-код: квадратная матрица модулей без «тихой зоны» вокруг.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QrCode {
    size: usize,
    modules: Vec<bool>,
    mask: u8,
}

impl QrCode {
    /// Кодирует текст, выбирая наименьшую версию и маску с лучшей читаемостью.
    pub fn encode(text: &str) -> Result<Self, QrError> {
        Self::encode_bytes(text.as_bytes(), None)
    }

    /// То же, но с принудительной маской `0..8` — нужно для сверки с эталоном в тестах.
    pub fn encode_with_mask(text: &str, mask: u8) -> Result<Self, QrError> {
        Self::encode_bytes(text.as_bytes(), Some(mask % MASK_COUNT))
    }

    fn encode_bytes(data: &[u8], forced_mask: Option<u8>) -> Result<Self, QrError> {
        let version = (1..=MAX_VERSION)
            .find(|&version| data.len() <= byte_capacity(version))
            .ok_or(QrError::TooLong { bytes: data.len(), capacity: byte_capacity(MAX_VERSION) })?;

        let codewords = interleave_with_ecc(&pack_data(data, version), version);
        let mut grid = Grid::new(version);
        grid.draw_function_patterns();
        grid.draw_codewords(&codewords);

        let mask = forced_mask.unwrap_or_else(|| grid.best_mask());
        grid.apply_mask(mask);
        grid.draw_format_bits(mask);

        Ok(QrCode { size: grid.size, modules: grid.modules, mask })
    }

    /// Сторона матрицы в модулях.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Номер применённой маски, `0..8`.
    pub fn mask(&self) -> u8 {
        self.mask
    }

    /// Тёмный ли модуль. За пределами матрицы — всегда светлый.
    pub fn is_dark(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size && self.modules[y * self.size + x]
    }

    /// Попадает ли модуль в один из трёх угловых «глаз» 7×7.
    pub fn is_finder(&self, x: usize, y: usize) -> bool {
        let near = |v: usize| v < 7;
        let far = |v: usize| v + 7 >= self.size;
        (near(x) && near(y)) || (far(x) && near(y)) || (near(x) && far(y))
    }
}

fn size_of(version: u8) -> usize {
    usize::from(version) * 4 + 17
}

/// Сколько модулей остаётся под данные и коррекцию после служебных узоров.
fn raw_data_modules(version: u8) -> usize {
    let v = usize::from(version);
    let mut modules = (16 * v + 128) * v + 64;
    if v >= 2 {
        let align = v / 7 + 2;
        modules -= (25 * align - 10) * align - 55;
        if v >= 7 {
            modules -= 36;
        }
    }
    modules
}

fn total_codewords(version: u8) -> usize {
    raw_data_modules(version) / 8
}

fn data_codewords(version: u8) -> usize {
    let index = usize::from(version) - 1;
    total_codewords(version) - ECC_PER_BLOCK[index] * BLOCK_COUNT[index]
}

/// Сколько байт текста помещается: 4 бита режима и 8 бит длины уходят на заголовок.
fn byte_capacity(version: u8) -> usize {
    (data_codewords(version) * 8 - 12) / 8
}

/// Заголовок, данные, терминатор и байты-заполнители — ровно `data_codewords(version)` байт.
fn pack_data(data: &[u8], version: u8) -> Vec<u8> {
    let capacity_bits = data_codewords(version) * 8;
    let mut bits = BitBuffer::default();
    bits.push(0b0100, 4);
    bits.push(data.len() as u32, 8);
    for &byte in data {
        bits.push(u32::from(byte), 8);
    }
    bits.push(0, (capacity_bits - bits.len()).min(4));
    bits.push(0, (8 - bits.len() % 8) % 8);

    let mut bytes = bits.into_bytes();
    for pad in [0xEC, 0x11].into_iter().cycle() {
        if bytes.len() == capacity_bits / 8 {
            break;
        }
        bytes.push(pad);
    }
    bytes
}

#[derive(Default)]
struct BitBuffer(Vec<bool>);

impl BitBuffer {
    fn push(&mut self, value: u32, count: usize) {
        self.0.extend((0..count).rev().map(|i| (value >> i) & 1 == 1));
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0
            .chunks(8)
            .map(|chunk| chunk.iter().fold(0u8, |byte, &bit| (byte << 1) | u8::from(bit)))
            .collect()
    }
}

/// Делит данные на блоки, дописывает каждому коды Рида — Соломона и перемежает побайтно.
fn interleave_with_ecc(data: &[u8], version: u8) -> Vec<u8> {
    let index = usize::from(version) - 1;
    let (block_count, ecc_len) = (BLOCK_COUNT[index], ECC_PER_BLOCK[index]);
    let total = total_codewords(version);
    let short_blocks = block_count - total % block_count;
    let short_data_len = total / block_count - ecc_len;
    let divisor = rs_divisor(ecc_len);

    let mut blocks: Vec<(&[u8], Vec<u8>)> = Vec::with_capacity(block_count);
    let mut offset = 0;
    for block in 0..block_count {
        let len = short_data_len + usize::from(block >= short_blocks);
        let chunk = &data[offset..offset + len];
        blocks.push((chunk, rs_remainder(chunk, &divisor)));
        offset += len;
    }

    let mut out = Vec::with_capacity(total);
    for i in 0..=short_data_len {
        out.extend(blocks.iter().filter_map(|(chunk, _)| chunk.get(i)));
    }
    for i in 0..ecc_len {
        out.extend(blocks.iter().map(|(_, ecc)| ecc[i]));
    }
    out
}

/// Умножение в поле GF(2⁸) по модулю x⁸+x⁴+x³+x²+1.
fn gf_mul(x: u8, y: u8) -> u8 {
    let mut z: u32 = 0;
    for i in (0..8).rev() {
        z = (z << 1) ^ ((z >> 7) * 0x11D);
        z ^= u32::from((y >> i) & 1) * u32::from(x);
    }
    z as u8
}

/// Порождающий многочлен степени `degree` (старший коэффициент 1 опущен).
fn rs_divisor(degree: usize) -> Vec<u8> {
    let mut divisor = vec![0u8; degree];
    divisor[degree - 1] = 1;
    let mut root = 1u8;
    for _ in 0..degree {
        for j in 0..degree {
            divisor[j] = gf_mul(divisor[j], root);
            if j + 1 < degree {
                divisor[j] ^= divisor[j + 1];
            }
        }
        root = gf_mul(root, 0x02);
    }
    divisor
}

fn rs_remainder(data: &[u8], divisor: &[u8]) -> Vec<u8> {
    let mut remainder = vec![0u8; divisor.len()];
    for &byte in data {
        let factor = byte ^ remainder.remove(0);
        remainder.push(0);
        for (r, &d) in remainder.iter_mut().zip(divisor) {
            *r ^= gf_mul(d, factor);
        }
    }
    remainder
}

/// Матрица в процессе сборки. `function` помечает служебные модули, которые не маскируются.
struct Grid {
    version: u8,
    size: usize,
    modules: Vec<bool>,
    function: Vec<bool>,
}

impl Grid {
    fn new(version: u8) -> Self {
        let size = size_of(version);
        Grid { version, size, modules: vec![false; size * size], function: vec![false; size * size] }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.modules[y * self.size + x]
    }

    fn set_function(&mut self, x: usize, y: usize, dark: bool) {
        self.modules[y * self.size + x] = dark;
        self.function[y * self.size + x] = true;
    }

    fn draw_function_patterns(&mut self) {
        let size = self.size;
        for i in 0..size {
            self.set_function(6, i, i % 2 == 0);
            self.set_function(i, 6, i % 2 == 0);
        }
        for (cx, cy) in [(3, 3), (size - 4, 3), (3, size - 4)] {
            self.draw_finder(cx as i32, cy as i32);
        }

        let positions = self.alignment_positions();
        let last = positions.len().saturating_sub(1);
        for (i, &x) in positions.iter().enumerate() {
            for (j, &y) in positions.iter().enumerate() {
                let on_finder = (i == 0 && j == 0) || (i == 0 && j == last) || (i == last && j == 0);
                if !on_finder {
                    self.draw_alignment(x, y);
                }
            }
        }

        // Резервируем место под биты формата; настоящие значения пишутся после выбора маски.
        self.draw_format_bits(0);
        self.draw_version_bits();
    }

    /// «Глаз» 7×7 со светлой каймой вокруг.
    fn draw_finder(&mut self, cx: i32, cy: i32) {
        for dy in -4..=4 {
            for dx in -4..=4 {
                let (x, y) = (cx + dx, cy + dy);
                if (0..self.size as i32).contains(&x) && (0..self.size as i32).contains(&y) {
                    let ring = dx.abs().max(dy.abs());
                    self.set_function(x as usize, y as usize, ring != 2 && ring != 4);
                }
            }
        }
    }

    fn draw_alignment(&mut self, cx: usize, cy: usize) {
        for dy in 0..5usize {
            for dx in 0..5usize {
                let ring = dx.abs_diff(2).max(dy.abs_diff(2));
                self.set_function(cx + dx - 2, cy + dy - 2, ring != 1);
            }
        }
    }

    fn alignment_positions(&self) -> Vec<usize> {
        let version = usize::from(self.version);
        if version == 1 {
            return Vec::new();
        }
        let count = version / 7 + 2;
        let step = (version * 4 + count * 2 + 1) / (count * 2 - 2) * 2;
        let mut positions: Vec<usize> = (0..count - 1).map(|i| self.size - 7 - i * step).collect();
        positions.push(6);
        positions.reverse();
        positions
    }

    /// 15 бит: уровень коррекции, маска и код BCH; пишутся дважды, вокруг «глаз».
    fn draw_format_bits(&mut self, mask: u8) {
        let data = (FORMAT_LEVEL_M << 3) | u32::from(mask);
        let mut remainder = data;
        for _ in 0..10 {
            remainder = (remainder << 1) ^ ((remainder >> 9) * 0x537);
        }
        let bits = ((data << 10) | remainder) ^ 0x5412;
        let bit = |i: usize| (bits >> i) & 1 == 1;
        let size = self.size;

        for i in 0..6 {
            self.set_function(8, i, bit(i));
        }
        self.set_function(8, 7, bit(6));
        self.set_function(8, 8, bit(7));
        self.set_function(7, 8, bit(8));
        for i in 9..15 {
            self.set_function(14 - i, 8, bit(i));
        }

        for i in 0..8 {
            self.set_function(size - 1 - i, 8, bit(i));
        }
        for i in 8..15 {
            self.set_function(8, size - 15 + i, bit(i));
        }
        self.set_function(8, size - 8, true);
    }

    /// Номер версии с кодом BCH — только для версий 7 и старше.
    fn draw_version_bits(&mut self) {
        if self.version < 7 {
            return;
        }
        let version = u32::from(self.version);
        let mut remainder = version;
        for _ in 0..12 {
            remainder = (remainder << 1) ^ ((remainder >> 11) * 0x1F25);
        }
        let bits = (version << 12) | remainder;
        for i in 0..18 {
            let dark = (bits >> i) & 1 == 1;
            let (a, b) = (self.size - 11 + i % 3, i / 3);
            self.set_function(a, b, dark);
            self.set_function(b, a, dark);
        }
    }

    /// Раскладывает биты змейкой снизу вверх парами столбцов, справа налево.
    fn draw_codewords(&mut self, codewords: &[u8]) {
        let size = self.size as i32;
        let total_bits = codewords.len() * 8;
        let mut index = 0;
        let mut right = size - 1;
        while right >= 1 {
            if right == 6 {
                right = 5;
            }
            for vertical in 0..size {
                for j in 0..2 {
                    let x = (right - j) as usize;
                    let upward = (right + 1) & 2 == 0;
                    let y = (if upward { size - 1 - vertical } else { vertical }) as usize;
                    let cell = y * self.size + x;
                    if !self.function[cell] && index < total_bits {
                        self.modules[cell] = (codewords[index / 8] >> (7 - index % 8)) & 1 == 1;
                        index += 1;
                    }
                }
            }
            right -= 2;
        }
    }

    /// Инвертирует модули данных по правилу маски. Повторный вызов снимает маску.
    fn apply_mask(&mut self, mask: u8) {
        for y in 0..self.size {
            for x in 0..self.size {
                let invert = match mask {
                    0 => (x + y) % 2 == 0,
                    1 => y % 2 == 0,
                    2 => x % 3 == 0,
                    3 => (x + y) % 3 == 0,
                    4 => (x / 3 + y / 2) % 2 == 0,
                    5 => x * y % 2 + x * y % 3 == 0,
                    6 => (x * y % 2 + x * y % 3) % 2 == 0,
                    _ => ((x + y) % 2 + x * y % 3) % 2 == 0,
                };
                let cell = y * self.size + x;
                self.modules[cell] ^= invert && !self.function[cell];
            }
        }
    }

    /// Перебирает все восемь масок и возвращает ту, у которой меньше штраф.
    fn best_mask(&mut self) -> u8 {
        let mut best = (0u8, u32::MAX);
        for mask in 0..MASK_COUNT {
            self.apply_mask(mask);
            self.draw_format_bits(mask);
            let penalty = self.penalty();
            if penalty < best.1 {
                best = (mask, penalty);
            }
            self.apply_mask(mask);
        }
        best.0
    }

    /// Штраф по четырём правилам стандарта: чем он меньше, тем легче сканеру.
    fn penalty(&self) -> u32 {
        let size = self.size;
        let mut penalty = 0;

        // Правила 1 и 3: длинные одноцветные полосы и узоры, похожие на «глаз», по строкам и столбцам.
        for line in 0..size {
            penalty += line_penalty(size, |i| self.get(i, line));
            penalty += line_penalty(size, |i| self.get(line, i));
        }

        // Правило 2: одноцветные квадраты 2×2.
        for y in 0..size - 1 {
            for x in 0..size - 1 {
                let color = self.get(x, y);
                if color == self.get(x + 1, y) && color == self.get(x, y + 1) && color == self.get(x + 1, y + 1) {
                    penalty += 3;
                }
            }
        }

        // Правило 4: перекос в сторону тёмных или светлых модулей.
        let dark = self.modules.iter().filter(|&&m| m).count();
        let total = size * size;
        let imbalance = (dark * 20).abs_diff(total * 10).div_ceil(total).saturating_sub(1);
        penalty + imbalance as u32 * 10
    }
}

/// Штраф одной строки или столбца: полосы от пяти модулей и узоры 1:1:3:1:1 со светлым полем.
fn line_penalty(size: usize, module: impl Fn(usize) -> bool) -> u32 {
    let mut penalty = 0;
    let mut history = RunHistory::new(size);
    let mut run_color = false;
    let mut run_len = 0;

    for i in 0..size {
        if module(i) == run_color {
            run_len += 1;
            if run_len == 5 {
                penalty += 3;
            } else if run_len > 5 {
                penalty += 1;
            }
        } else {
            history.add(run_len);
            if !run_color {
                penalty += history.finder_like() * 40;
            }
            run_color = module(i);
            run_len = 1;
        }
    }
    penalty + history.terminate(run_color, run_len) * 40
}

/// Длины последних семи полос в строке — по ним ищутся узоры, похожие на «глаз».
struct RunHistory {
    runs: [usize; 7],
    size: usize,
}

impl RunHistory {
    fn new(size: usize) -> Self {
        RunHistory { runs: [0; 7], size }
    }

    fn add(&mut self, mut run_len: usize) {
        if self.runs[0] == 0 {
            run_len += self.size; // Светлое поле за краем матрицы.
        }
        self.runs.rotate_right(1);
        self.runs[0] = run_len;
    }

    fn finder_like(&self) -> u32 {
        let r = &self.runs;
        let n = r[1];
        let core = n > 0 && r[2] == n && r[3] == n * 3 && r[4] == n && r[5] == n;
        u32::from(core && r[0] >= n * 4 && r[6] >= n) + u32::from(core && r[6] >= n * 4 && r[0] >= n)
    }

    fn terminate(&mut self, run_color: bool, mut run_len: usize) -> u32 {
        if run_color {
            self.add(run_len);
            run_len = 0;
        }
        self.add(run_len + self.size);
        self.finder_like()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Эталон от `qrencode -l M -8 -m 0 -t ASCII`: два символа на модуль, `#` — тёмный.
    fn parse_fixture(text: &str) -> Vec<Vec<bool>> {
        text.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.as_bytes().chunks(2).map(|pair| pair[0] == b'#').collect())
            .collect()
    }

    /// Читает номер маски из битов формата эталонной матрицы.
    fn mask_of(reference: &[Vec<bool>]) -> u8 {
        let at = |x: usize, y: usize| u32::from(reference[y][x]);
        let mut bits = 0;
        for i in 0..6 {
            bits |= at(8, i) << i;
        }
        bits |= at(8, 7) << 6 | at(8, 8) << 7 | at(7, 8) << 8;
        for i in 9..15 {
            bits |= at(14 - i, 8) << i;
        }
        (((bits ^ 0x5412) >> 10) & 0b111) as u8
    }

    fn assert_matches_reference(text: &str, fixture: &str) {
        let reference = parse_fixture(fixture);
        let qr = QrCode::encode_with_mask(text, mask_of(&reference)).expect("fits");
        assert_eq!(qr.size(), reference.len(), "версия отличается от эталона");
        for (y, row) in reference.iter().enumerate() {
            for (x, &dark) in row.iter().enumerate() {
                assert_eq!(qr.is_dark(x, y), dark, "модуль ({x}, {y}) отличается от эталона");
            }
        }
    }

    #[test]
    fn version_2_matches_qrencode() {
        assert_matches_reference("https://example.com/", include_str!("../tests/fixtures/qr-182cce.txt"));
    }

    #[test]
    fn version_4_with_two_blocks_matches_qrencode() {
        assert_matches_reference(
            "https://oficerova.example/price?utm=qr-card-2026",
            include_str!("../tests/fixtures/qr-750abb.txt"),
        );
    }

    /// Маску кодер выбирает сам. С `qrencode` выбор может расходиться (оценка штрафа у него
    /// чуть другая), но любая из восьми масок даёт корректный код, поэтому проверяем только,
    /// что автоматический результат совпадает с принудительным для той же маски.
    #[test]
    fn automatic_mask_is_one_of_the_valid_masks() {
        for text in ["https://example.com/", "https://oficerova.example/price?utm=qr-card-2026"] {
            let auto = QrCode::encode(text).expect("fits");
            assert!(auto.mask() < MASK_COUNT);
            assert_eq!(Ok(&auto), QrCode::encode_with_mask(text, auto.mask()).as_ref());
        }
    }

    #[test]
    fn every_supported_version_has_valid_structure() {
        for version in 1..=MAX_VERSION {
            let text = "a".repeat(byte_capacity(version));
            let qr = QrCode::encode(&text).expect("fits exactly");
            let size = size_of(version);
            assert_eq!(qr.size(), size);
            // Центры трёх «глаз» тёмные, кайма вокруг них светлая.
            for (x, y) in [(3, 3), (size - 4, 3), (3, size - 4)] {
                assert!(qr.is_dark(x, y));
                assert!(qr.is_finder(x, y));
            }
            assert!(!qr.is_dark(7, 7));
            // Обязательный тёмный модуль и чередование синхрополосы.
            assert!(qr.is_dark(8, size - 8));
            for i in 8..size - 8 {
                assert_eq!(qr.is_dark(i, 6), i % 2 == 0);
            }
        }
    }

    #[test]
    fn capacity_grows_with_version_and_boundaries_are_exact() {
        assert_eq!(byte_capacity(1), 14);
        assert_eq!(byte_capacity(MAX_VERSION), 180);
        assert!((1..MAX_VERSION).all(|v| byte_capacity(v) < byte_capacity(v + 1)));
        assert_eq!(QrCode::encode(&"a".repeat(14)).map(|qr| qr.size()), Ok(21));
        assert_eq!(QrCode::encode(&"a".repeat(15)).map(|qr| qr.size()), Ok(25));
    }

    #[test]
    fn empty_text_is_encoded() {
        assert_eq!(QrCode::encode("").map(|qr| qr.size()), Ok(21));
    }

    #[test]
    fn too_long_text_is_rejected_not_truncated() {
        let text = "я".repeat(91); // 182 байта в UTF-8.
        assert_eq!(QrCode::encode(&text), Err(QrError::TooLong { bytes: 182, capacity: 180 }));
    }

    #[test]
    fn out_of_range_modules_are_light() {
        let qr = QrCode::encode("x").expect("fits");
        assert!(!qr.is_dark(qr.size(), 0));
        assert!(!qr.is_dark(0, usize::MAX));
    }

    #[test]
    fn reed_solomon_remainder_of_zero_data_is_zero() {
        let divisor = rs_divisor(10);
        assert_eq!(rs_remainder(&[0; 16], &divisor), vec![0; 10]);
    }

    #[test]
    fn gf_multiplication_is_commutative_with_identity() {
        for x in 0..=255u8 {
            assert_eq!(gf_mul(x, 1), x);
            assert_eq!(gf_mul(x, 0), 0);
            assert_eq!(gf_mul(x, 0x53), gf_mul(0x53, x));
        }
    }
}

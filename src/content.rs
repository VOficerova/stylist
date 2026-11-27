//! Контент сайта. Чтобы поменять цену, подпись или текст — правьте этот файл и пересоберите.

use std::fmt;

pub const BRAND: &str = "V. Oficerova";
pub const SURNAME: &str = "Офицерова";
pub const FIRST_NAME: &str = "Василиса";
pub const ROLE: &str = "Парикмахер-колорист и визажист";
pub const MOTTO: &str = "Твой образ, твои правила, твоя красота.";
pub const INSTAGRAM_URL: &str = "https://www.instagram.com/_oficerova_vasilisa_";
pub const INSTAGRAM_HANDLE: &str = "@_oficerova_vasilisa_";
pub const PHONE_LABEL: &str = "+7 925 046-85-19";
pub const PHONE_URL: &str = "tel:+79250468519";

/// Разделы страницы, на которые ведёт навигация.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Section {
    Services,
    About,
    Price,
    Portfolio,
    Contacts,
}

impl Section {
    pub const ALL: [Section; 5] = [
        Section::Services,
        Section::About,
        Section::Price,
        Section::Portfolio,
        Section::Contacts,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Section::Services => "services",
            Section::About => "about",
            Section::Price => "price",
            Section::Portfolio => "portfolio",
            Section::Contacts => "contacts",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Section::Services => "Услуги",
            Section::About => "Обо мне",
            Section::Price => "Прайс",
            Section::Portfolio => "Работы",
            Section::Contacts => "Контакты",
        }
    }
}

/// Навык в секции «Что я умею».
pub struct Skill {
    pub name: &'static str,
    pub icon: &'static str,
    pub text: &'static str,
    pub tags: &'static [&'static str],
}

// TODO(content): описания навыков — черновик, сверить формулировки с Василисой.
pub const SKILLS: &[Skill] = &[
    Skill {
        name: "Окрашивание",
        icon: "assets/services/butterfly.webp",
        text: "Ровный цвет от корней до кончиков. Подбираю оттенок под тон кожи и глаз, закрашиваю седину и отросшие корни, освежаю цвет по длине. Могу работать и с вашей краской.",
        tags: &["тон в тон", "корни", "седина", "своя краска"],
    },
    Skill {
        name: "Сложное окрашивание",
        icon: "assets/services/lily.webp",
        text: "Многоступенчатая работа с цветом: осветление прядей, мягкие переходы и тонирование. Контуринг высветляет волосы у лица и подчёркивает его форму. Делаю так, чтобы цвет красиво отрастал без резкой границы.",
        tags: &["контуринг", "блики у лица", "растяжка цвета", "тонирование"],
    },
    Skill {
        name: "Стрижки",
        icon: "assets/services/scissors.webp",
        text: "Женские и мужские на любую длину. Форму подбираю под овал лица, густоту и то, как вы укладываете волосы каждый день: стрижка должна ложиться сама.",
        tags: &["женские", "мужские", "короткие", "длинные"],
    },
    Skill {
        name: "Укладка",
        icon: "assets/services/roses.webp",
        text: "Гладкие волосы, прикорневой объём, мягкие волны или локоны. Дневная укладка лёгкая и подвижная, вечерняя держится весь праздник.",
        tags: &["дневная", "вечерняя", "волны", "объём"],
    },
    Skill {
        name: "Мейк",
        icon: "assets/services/vanilla.webp",
        text: "От почти незаметного нюда до вечернего образа со стрелками и яркими губами. Работаю с тоном так, чтобы кожа оставалась живой, и учитываю свет, в котором вы будете: дневной, вечерний или съёмочный.",
        tags: &["нюд", "дневной", "вечерний"],
    },
    Skill {
        name: "Уходы",
        icon: "assets/services/watering-can.webp",
        text: "Восстанавливающие процедуры для волос после осветления, окрашивания и горячих укладок. Возвращают плотность, мягкость и блеск; состав подбираю по состоянию волос.",
        tags: &["восстановление", "блеск", "после окрашивания"],
    },
    Skill {
        name: "Брови и ресницы",
        icon: "assets/services/bow.webp",
        text: "Коррекция формы и окрашивание бровей, окрашивание ресниц. Взгляд становится выразительнее, а утром можно обходиться без карандаша и туши.",
        tags: &["коррекция", "окрашивание бровей", "окрашивание ресниц"],
    },
];

// TODO(content): заменить на текст Василисы — стаж, обучение, подход.
pub const ABOUT: &[&str] = &[
    "Меня зовут Василиса. Я работаю с волосами и лицом как с одной картиной: сначала слушаю, какой вы хотите себя видеть, потом подбираю цвет, форму и макияж, которые это покажут.",
    "Мне важно, чтобы результат жил дольше одного вечера. Окрашивание должно красиво отрастать, стрижка должна укладываться дома за десять минут, а мейк должен оставаться вашим лицом.",
];

/// Цена в рублях: одна сумма или вилка «от и до».
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Price {
    Fixed(u32),
    Range(u32, u32),
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Price::Fixed(sum) => f.write_str(&group_thousands(sum)),
            Price::Range(from, to) => {
                write!(f, "{}–{}", group_thousands(from), group_thousands(to))
            }
        }
    }
}

/// `10500` → `10 500` с неразрывным пробелом, чтобы сумма не переносилась посередине.
fn group_thousands(sum: u32) -> String {
    let digits = sum.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push('\u{a0}');
        }
        out.push(ch);
    }
    out
}

pub struct PriceItem {
    pub label: &'static str,
    pub price: Price,
}

const fn item(label: &'static str, sum: u32) -> PriceItem {
    PriceItem { label, price: Price::Fixed(sum) }
}

/// Как плитка прайса показывает свои позиции.
pub enum CardBody {
    /// Обычный список «название — цена».
    List(&'static [PriceItem]),
    /// Два списка подряд (женские и мужские стрижки).
    Split(&'static [PriceItem], &'static [PriceItem]),
    /// Крупный список для короткой плитки-акцента.
    Big(&'static [PriceItem]),
    /// Одна вилка цен с пояснением.
    Range { price: Price, note: &'static str },
}

pub struct PriceCard {
    /// Модификатор CSS-класса, он же место плитки в сетке.
    pub slot: &'static str,
    pub title: &'static str,
    pub body: CardBody,
}

pub const PRICE_CARDS: &[PriceCard] = &[
    PriceCard {
        slot: "color",
        title: "Окрашивание",
        body: CardBody::List(&[
            item("Своей краской", 3_500),
            item("Корни", 5_000),
            item("Тон в тон, короткие", 6_500),
            item("Тон в тон, средние", 7_000),
            item("Тон в тон, длинные", 7_500),
        ]),
    },
    PriceCard {
        slot: "complex",
        title: "Сложное окрашивание",
        body: CardBody::List(&[
            item("Контуринг, короткие", 3_500),
            item("Контуринг, средние", 5_500),
            item("Контуринг, длинные", 7_500),
            item("Средние волосы", 7_500),
            item("Длинные волосы", 10_500),
        ]),
    },
    PriceCard {
        slot: "cuts",
        title: "Стрижки",
        body: CardBody::Split(
            &[
                item("Женская короткая", 2_000),
                item("Женская средняя", 2_300),
                item("Женская длинная", 2_700),
            ],
            &[
                item("Мужская короткая", 1_100),
                item("Мужская средняя", 1_300),
                item("Мужская длинная", 2_000),
            ],
        ),
    },
    PriceCard {
        slot: "styling",
        title: "Укладка",
        body: CardBody::List(&[
            item("Короткие волосы", 1_900),
            item("Средние волосы", 2_200),
            item("Длинные, дневная", 2_700),
            item("Длинные, вечерняя", 2_900),
        ]),
    },
    PriceCard {
        slot: "makeup",
        title: "Мейк",
        body: CardBody::Big(&[
            item("Нюд", 3_000),
            item("Дневной", 4_000),
            item("Вечерний", 5_000),
        ]),
    },
    PriceCard {
        slot: "care",
        title: "Уходы",
        body: CardBody::Range {
            price: Price::Range(4_500, 5_500),
            note: "Подбираю уход по состоянию волос.",
        },
    },
    PriceCard {
        slot: "brows",
        title: "Брови и ресницы",
        body: CardBody::List(&[
            item("Окрашивание бровей", 700),
            item("Окрашивание и коррекция бровей", 900),
            item("Окрашивание ресниц", 700),
            item("Ресницы, окрашивание и коррекция бровей", 1_500),
        ]),
    },
];

/// Место фото в сетке портфолио.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShotLayout {
    /// Двойная ширина и высота.
    Big,
    /// Одна ячейка с центральным кадрированием.
    Standard,
    /// Одна ячейка, кадрируется по верхней части (лицо).
    Top,
}

/// Работа в портфолио. Файлы лежат в `assets/img/{slug}-700.webp` и `{slug}-1400.webp`.
pub struct Shot {
    pub slug: &'static str,
    pub alt: &'static str,
    pub caption: &'static str,
    pub small_width: u32,
    pub width: u32,
    pub height: u32,
    pub layout: ShotLayout,
}

pub const SHOTS: &[Shot] = &[
    Shot { slug: "beige-waves", alt: "Бежевый блонд, уложенный в мягкие волны", caption: "Сложное окрашивание", small_width: 525, width: 1050, height: 1400, layout: ShotLayout::Big },
    Shot { slug: "cherry", alt: "Длинные волосы глубокого вишнёвого цвета с чёлкой", caption: "Окрашивание тон в тон", small_width: 525, width: 1050, height: 1400, layout: ShotLayout::Standard },
    Shot { slug: "red-lips", alt: "Графичные стрелки и красные губы", caption: "Мейк", small_width: 468, width: 855, height: 1280, layout: ShotLayout::Top },
    Shot { slug: "mens-cut", alt: "Короткая мужская стрижка с плавным переходом по бокам", caption: "Мужская стрижка", small_width: 535, width: 1070, height: 1400, layout: ShotLayout::Top },
    Shot { slug: "womens-cut", alt: "Женская стрижка со светлыми прядями и слоями по длине", caption: "Женская стрижка", small_width: 460, width: 921, height: 1400, layout: ShotLayout::Standard },
];

/// Ссылка на мессенджер. `url: None` — ссылка ещё не получена, такой пункт не выводится.
pub struct Messenger {
    pub label: &'static str,
    pub url: Option<&'static str>,
    /// Содержимое SVG-иконки 48×48.
    pub icon: &'static str,
}

/// Блок мессенджеров выводится на страницу, только когда этот флаг включён.
pub const SHOW_MESSENGERS: bool = false;

pub const MESSENGERS: &[Messenger] = &[
    Messenger {
        label: "Telegram",
        url: None, // TODO(content): https://t.me/ЛОГИН
        icon: r#"<path d="M42 7 L5 22 L17 27 L21 40 L28 31 L37 38 Z"/><path d="M17 27 L36 13"/>"#,
    },
    Messenger {
        label: "Telegram-канал",
        url: Some("https://t.me/voficerova"),
        icon: r#"<path d="M8 20 V28 H15 L32 38 V10 L15 20 Z"/><path d="M37 19 C39.5 21.5 39.5 26.5 37 29"/><path d="M16 28 L19 40 H24 L22 31"/>"#,
    },
    Messenger {
        label: "WhatsApp",
        url: None, // TODO(content): https://wa.me/7XXXXXXXXXX
        icon: r#"<path d="M7 41 L10 31 C4 20 12 7 24 7 C34 7 41 14 41 24 C41 36 28 44 17 38 Z"/><path d="M18 17 C17 23 25 31 31 30 L32 26 L28 24 L26 26 C24 25 23 24 22 22 L24 20 L22 16 Z"/>"#,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prices_are_grouped_by_thousands_with_non_breaking_space() {
        assert_eq!(Price::Fixed(700).to_string(), "700");
        assert_eq!(Price::Fixed(3_500).to_string(), "3\u{a0}500");
        assert_eq!(Price::Fixed(10_500).to_string(), "10\u{a0}500");
        assert_eq!(Price::Fixed(1_000_000).to_string(), "1\u{a0}000\u{a0}000");
        assert_eq!(Price::Range(4_500, 5_500).to_string(), "4\u{a0}500–5\u{a0}500");
    }

    #[test]
    fn price_list_matches_the_source_document() {
        let count: usize = PRICE_CARDS
            .iter()
            .map(|card| match &card.body {
                CardBody::List(items) | CardBody::Big(items) => items.len(),
                CardBody::Split(a, b) => a.len() + b.len(),
                CardBody::Range { .. } => 1,
            })
            .sum();
        assert_eq!(count, 28, "в прайс-листе заказчика 28 позиций");
    }

    #[test]
    fn section_ids_are_unique() {
        for (i, a) in Section::ALL.iter().enumerate() {
            for b in &Section::ALL[i + 1..] {
                assert_ne!(a.id(), b.id());
            }
        }
    }
}

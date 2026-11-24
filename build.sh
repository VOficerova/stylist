#!/usr/bin/env bash
# Собирает сайт в dist/: готовый index.html, стили, картинки и WASM-модуль.
# Нужны: cargo с целью wasm32-unknown-unknown и wasm-bindgen-cli той же версии, что в Cargo.toml.
set -euo pipefail
cd "$(dirname "$0")"

target=wasm32-unknown-unknown
crate=vasilisa_site

rm -rf dist
mkdir -p dist
cp -R assets css dist/
cp web/boot.js dist/

# Страница рендерится тем же Rust-кодом, что работает в браузере.
cargo run --quiet --release --bin prerender > dist/index.html

cargo build --quiet --release --lib --target "$target"
wasm-bindgen --target web --no-typescript --out-dir dist/pkg "target/$target/release/$crate.wasm"

echo "Готово: dist/ ($(du -sh dist | cut -f1), WASM $(du -h "dist/pkg/${crate}_bg.wasm" | cut -f1))"

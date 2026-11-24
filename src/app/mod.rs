//! Поведение страницы в браузере. Разметка уже отрисована при сборке (`prerender`),
//! здесь к ней только подключаются обработчики.

mod depth;
mod dom;
mod lightbox;
mod nav;
mod share;

use wasm_bindgen::prelude::wasm_bindgen;

/// Точка входа: вызывается один раз, когда WASM-модуль загружен.
///
/// Каждая часть независима и возвращает `None`, если её элементов нет на странице, —
/// сбой одной не ломает остальные.
#[wasm_bindgen(start)]
pub fn start() {
    let _ = nav::init();
    let _ = depth::init();
    let _ = lightbox::init();
    let _ = share::init();
}

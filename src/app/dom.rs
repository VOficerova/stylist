//! Короткие обёртки над `web-sys`, чтобы остальной код читался как намерение, а не как API.

use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::JsCast;
use web_sys::{AddEventListenerOptions, Document, Element, Event, EventTarget, HtmlDialogElement, Window};

pub fn window() -> Option<Window> {
    web_sys::window()
}

pub fn document() -> Option<Document> {
    window()?.document()
}

/// Элемент по `id`, приведённый к нужному типу.
pub fn by_id<T: JsCast>(id: &str) -> Option<T> {
    document()?.get_element_by_id(id)?.dyn_into().ok()
}

pub fn query(selector: &str) -> Option<Element> {
    document()?.query_selector(selector).ok()?
}

pub fn query_all(selector: &str) -> Vec<Element> {
    let Some(list) = document().and_then(|doc| doc.query_selector_all(selector).ok()) else {
        return Vec::new();
    };
    (0..list.length()).filter_map(|i| list.item(i)?.dyn_into().ok()).collect()
}

/// Высота окна в CSS-пикселях.
pub fn viewport_height(window: &Window) -> f64 {
    window.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0)
}

pub fn viewport_width(window: &Window) -> f64 {
    window.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(0.0)
}

/// Подписывает обработчик на событие.
///
/// Обработчики живут столько же, сколько страница, поэтому замыкание намеренно
/// «забывается»: освобождать его некому и незачем.
pub fn on<E>(target: &EventTarget, kind: &str, handler: impl FnMut(E) + 'static)
where
    E: FromWasmAbi + 'static,
{
    let closure = Closure::<dyn FnMut(E)>::new(handler);
    let _ = target.add_event_listener_with_callback(kind, closure.as_ref().unchecked_ref());
    closure.forget();
}

/// То же, но пассивно: браузер не ждёт обработчик, чтобы начать прокрутку.
pub fn on_passive<E>(target: &EventTarget, kind: &str, handler: impl FnMut(E) + 'static)
where
    E: FromWasmAbi + 'static,
{
    let closure = Closure::<dyn FnMut(E)>::new(handler);
    let options = AddEventListenerOptions::new();
    options.set_passive(true);
    let _ = target.add_event_listener_with_callback_and_add_event_listener_options(
        kind,
        closure.as_ref().unchecked_ref(),
        &options,
    );
    closure.forget();
}

/// Закрытие модального окна по кнопке с `data-close` и по клику на затемнённый фон.
pub fn wire_dialog_closing(dialog: &HtmlDialogElement) {
    if let Ok(buttons) = dialog.query_selector_all("[data-close]") {
        for button in (0..buttons.length()).filter_map(|i| buttons.item(i)) {
            let dialog = dialog.clone();
            on(&button, "click", move |_: Event| dialog.close());
        }
    }

    let backdrop: EventTarget = dialog.clone().into();
    let dialog = dialog.clone();
    on(dialog.clone().as_ref(), "click", move |event: Event| {
        // Клик по содержимому приходит от вложенного элемента; от самого <dialog> — только с фона.
        if event.target().as_ref() == Some(&backdrop) {
            dialog.close();
        }
    });
}

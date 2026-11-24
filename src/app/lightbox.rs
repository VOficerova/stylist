//! Просмотр работы из портфолио в модальном окне.

use wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlDialogElement, HtmlImageElement};

use super::dom;

pub fn init() -> Option<()> {
    let dialog: HtmlDialogElement = dom::by_id("lightbox")?;
    let image: HtmlImageElement = dom::by_id("lightbox-img")?;
    let caption: Element = dom::by_id("lightbox-cap")?;
    let shots = dom::query(".shots")?;

    // Один обработчик на всю сетку: клик поднимается от фото к списку.
    {
        let (dialog, image) = (dialog.clone(), image.clone());
        dom::on(&shots, "click", move |event: Event| {
            let Some(button) = event
                .target()
                .and_then(|target| target.dyn_into::<Element>().ok())
                .and_then(|element| element.closest("button[data-full]").ok().flatten())
            else {
                return;
            };
            let Some(full) = button.get_attribute("data-full") else { return };
            let thumbnail_alt = button
                .query_selector("img")
                .ok()
                .flatten()
                .and_then(|img| img.get_attribute("alt"))
                .unwrap_or_default();
            let title = button.query_selector("strong").ok().flatten().and_then(|el| el.text_content());

            image.set_src(&full);
            image.set_alt(&thumbnail_alt);
            caption.set_text_content(title.as_deref());
            let _ = dialog.show_modal();
        });
    }

    dom::wire_dialog_closing(&dialog);
    // Убираем фото после закрытия, чтобы при следующем открытии не мелькало предыдущее.
    dom::on(&dialog, "close", move |_: Event| {
        let _ = image.remove_attribute("src");
    });

    Some(())
}

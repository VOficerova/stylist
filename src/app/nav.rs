//! Навигация: стеклянная полоса после начала прокрутки, полноэкранное меню на телефоне,
//! подсветка раздела, который сейчас на экране.

use std::rc::Rc;

use web_sys::{Element, Event, KeyboardEvent};

use super::dom;

/// После скольких пикселей прокрутки полоса становится стеклянной.
const SCROLLED_AFTER_PX: f64 = 24.0;
/// Раздел считается текущим, когда его верх поднялся выше этой доли высоты экрана.
const CURRENT_SECTION_LINE: f64 = 0.45;

pub fn init() -> Option<()> {
    let window = dom::window()?;
    let document = dom::document()?;
    let root = document.document_element()?;
    let nav: Element = dom::by_id("nav")?;
    let toggle = nav.query_selector(".nav__toggle").ok()??;

    let set_open: Rc<dyn Fn(bool)> = {
        let (nav, toggle) = (nav.clone(), toggle.clone());
        Rc::new(move |open| {
            let _ = nav.class_list().toggle_with_force("is-open", open);
            // Пока меню открыто, страница под ним не прокручивается.
            let _ = root.class_list().toggle_with_force("is-locked", open);
            let _ = toggle.set_attribute("aria-expanded", if open { "true" } else { "false" });
        })
    };

    {
        let (nav, set_open) = (nav.clone(), set_open.clone());
        dom::on(&toggle, "click", move |_: Event| set_open(!nav.class_list().contains("is-open")));
    }
    for link in dom::query_all(".nav__panel a") {
        let set_open = set_open.clone();
        dom::on(&link, "click", move |_: Event| set_open(false));
    }
    {
        let set_open = set_open.clone();
        dom::on(&document, "keydown", move |event: KeyboardEvent| {
            if event.key() == "Escape" {
                set_open(false);
            }
        });
    }

    // Пары «ссылка — раздел, на который она ведёт».
    let links: Vec<(Element, Element)> = dom::query_all(".nav__list a")
        .into_iter()
        .filter_map(|link| {
            let id = link.get_attribute("href")?.strip_prefix('#')?.to_owned();
            Some((link, dom::by_id(&id)?))
        })
        .collect();

    let update = {
        let window = window.clone();
        move || {
            let scrolled = window.scroll_y().unwrap_or(0.0) > SCROLLED_AFTER_PX;
            let _ = nav.class_list().toggle_with_force("is-scrolled", scrolled);

            let line = dom::viewport_height(&window) * CURRENT_SECTION_LINE;
            let current = links.iter().rposition(|(_, section)| section.get_bounding_client_rect().top() <= line);
            for (index, (link, _)) in links.iter().enumerate() {
                if Some(index) == current {
                    let _ = link.set_attribute("aria-current", "true");
                } else {
                    let _ = link.remove_attribute("aria-current");
                }
            }
        }
    };
    update();
    dom::on_passive(&window, "scroll", move |_: Event| update());

    Some(())
}

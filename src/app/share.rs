//! «Поделиться сайтом»: карточка с QR-кодом в стиле сайта, которую можно скачать картинкой.

use std::f64::consts::TAU;

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    CanvasRenderingContext2d, Element, Event, HtmlAnchorElement, HtmlCanvasElement, HtmlDialogElement,
    HtmlElement, HtmlImageElement,
};

use super::dom;
use crate::content::{FIRST_NAME, ROLE, SURNAME};
use crate::qr::QrCode;

const NIGHT: &str = "#131015";
const INK: &str = "#1A141C";
const IVORY: &str = "#EFE9DD";
const ROSE: &str = "#D7A9AF";
const EYE: &str = "#6B3F4B";

/// Светлая подложка под QR-код: сканерам нужен контраст и «тихая зона» вокруг.
const PANEL: Rect = Rect { x: 160.0, y: 450.0, size: 760.0 };
/// Ширина «тихой зоны» в модулях с каждой стороны.
const QUIET_MODULES: f64 = 2.0;
/// Радиус точки в долях модуля. Меньше 0,4 сканеры начинают ошибаться.
const DOT_RADIUS: f64 = 0.46;

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    size: f64,
}

pub fn init() -> Option<()> {
    let open: HtmlElement = dom::by_id("share-open")?;
    let dialog: HtmlDialogElement = dom::by_id("share")?;
    let canvas: HtmlCanvasElement = dom::by_id("share-canvas")?;
    let url_label: Element = dom::by_id("share-url")?;
    let download: HtmlAnchorElement = dom::by_id("share-download")?;

    let url = site_url()?;
    // Слишком длинный адрес в код не поместится — тогда кнопка просто остаётся скрытой.
    let qr = QrCode::encode(&url).ok()?;
    let shown_url = url.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/').to_owned();

    url_label.set_text_content(Some(&shown_url));
    dom::wire_dialog_closing(&dialog);
    wire_native_share(&url);

    dom::on(&open, "click", move |_: Event| {
        if draw_card(&canvas, &qr, &shown_url).is_some() {
            if let Ok(png) = canvas.to_data_url() {
                download.set_href(&png);
            }
        }
        let _ = dialog.show_modal();
    });
    let _ = open.remove_attribute("hidden");

    Some(())
}

/// Адрес, с которого открыт сайт, без якоря и параметров: его и кодируем.
fn site_url() -> Option<String> {
    let location = dom::window()?.location();
    let path = location.pathname().ok()?;
    Some(format!("{}{}", location.origin().ok()?, path.trim_end_matches("index.html")))
}

/// Системное меню «Поделиться» есть не во всех браузерах — кнопка появляется, только если оно есть.
fn wire_native_share(url: &str) -> Option<()> {
    let button: HtmlElement = dom::by_id("share-native")?;
    let navigator = dom::window()?.navigator();
    let share: Function = Reflect::get(&navigator, &JsValue::from_str("share")).ok()?.dyn_into().ok()?;

    let url = url.to_owned();
    dom::on(&button, "click", move |_: Event| {
        let data = Object::new();
        let _ = Reflect::set(&data, &JsValue::from_str("title"), &JsValue::from_str(&format!("{SURNAME} {FIRST_NAME}")));
        let _ = Reflect::set(&data, &JsValue::from_str("url"), &JsValue::from_str(&url));
        // Пользователь может закрыть меню — отклонённое обещание нас не интересует.
        let _ = share.call1(&navigator, &data);
    });
    let _ = button.remove_attribute("hidden");
    Some(())
}

/// Рисует карточку 1080×1440: фон с лилией, имя, QR на светлой подложке, подпись и адрес.
fn draw_card(canvas: &HtmlCanvasElement, qr: &QrCode, shown_url: &str) -> Option<()> {
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").ok()??.dyn_into().ok()?;
    let (width, height) = (f64::from(canvas.width()), f64::from(canvas.height()));

    let ground = ctx.create_linear_gradient(0.0, 0.0, 0.0, height);
    ground.add_color_stop(0.0, "#1C151D").ok()?;
    ground.add_color_stop(1.0, "#100E12").ok()?;
    ctx.set_fill_style_canvas_gradient(&ground);
    ctx.fill_rect(0.0, 0.0, width, height);

    draw_lily(&ctx, width);

    ctx.set_text_align("left");
    ctx.set_fill_style_str(IVORY);
    ctx.set_font("400 150px Oranienbaum, serif");
    ctx.fill_text(&SURNAME.to_uppercase(), 84.0, 250.0).ok()?;
    ctx.set_fill_style_str(ROSE);
    ctx.set_font("400 136px 'Great Vibes', cursive");
    ctx.fill_text(FIRST_NAME, 300.0, 362.0).ok()?;

    ctx.set_fill_style_str(IVORY);
    rounded_square(&ctx, PANEL, 44.0)?;
    ctx.fill();
    draw_code(&ctx, qr)?;

    ctx.set_text_align("center");
    ctx.set_fill_style_str(IVORY);
    ctx.set_font("300 40px Jost, sans-serif");
    ctx.fill_text(ROLE, width / 2.0, 1296.0).ok()?;
    ctx.set_fill_style_str(ROSE);
    ctx.set_font("400 46px Oranienbaum, serif");
    ctx.fill_text(shown_url, width / 2.0, 1368.0).ok()?;

    Some(())
}

/// Та же лилия, что на первом экране; она уже загружена, новых запросов не будет.
fn draw_lily(ctx: &CanvasRenderingContext2d, width: f64) {
    let Some(lily) = dom::query(".flower--hero-lily").and_then(|el| el.dyn_into::<HtmlImageElement>().ok()) else {
        return;
    };
    if !lily.complete() || lily.natural_width() == 0 {
        return;
    }
    ctx.set_global_alpha(0.6);
    let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(&lily, width - 760.0, -60.0, 930.0, 620.0);
    ctx.set_global_alpha(1.0);

    // Низ цветка уходит в фон, чтобы не спорить с кодом.
    let veil = ctx.create_linear_gradient(0.0, 300.0, 0.0, 560.0);
    let _ = veil.add_color_stop(0.0, "rgba(19, 16, 21, 0)");
    let _ = veil.add_color_stop(1.0, NIGHT);
    ctx.set_fill_style_canvas_gradient(&veil);
    ctx.fill_rect(0.0, 300.0, width, 260.0);
}

/// Модули — круглые точки, три «глаза» — скруглённые квадраты в цвете акцента.
fn draw_code(ctx: &CanvasRenderingContext2d, qr: &QrCode) -> Option<()> {
    let modules = qr.size();
    let cell = PANEL.size / (modules as f64 + QUIET_MODULES * 2.0);
    let origin = (PANEL.x + QUIET_MODULES * cell, PANEL.y + QUIET_MODULES * cell);

    ctx.set_fill_style_str(INK);
    for y in 0..modules {
        for x in 0..modules {
            if qr.is_dark(x, y) && !qr.is_finder(x, y) {
                ctx.begin_path();
                ctx.arc(origin.0 + (x as f64 + 0.5) * cell, origin.1 + (y as f64 + 0.5) * cell, cell * DOT_RADIUS, 0.0, TAU)
                    .ok()?;
                ctx.fill();
            }
        }
    }

    let far = (modules - 7) as f64;
    for (column, row) in [(0.0, 0.0), (far, 0.0), (0.0, far)] {
        let eye = Rect { x: origin.0 + column * cell, y: origin.1 + row * cell, size: cell * 7.0 };
        // Кольцо 7×7, светлый зазор 5×5 и зрачок 3×3 — пропорции задаёт стандарт.
        for (inset, radius, color) in [(0.0, 2.0, EYE), (1.0, 1.3, IVORY), (2.0, 0.9, INK)] {
            ctx.set_fill_style_str(color);
            let ring = Rect { x: eye.x + inset * cell, y: eye.y + inset * cell, size: eye.size - inset * cell * 2.0 };
            rounded_square(ctx, ring, radius * cell)?;
            ctx.fill();
        }
    }
    Some(())
}

/// Строит путь скруглённого квадрата; заливку вызывает вызывающий код.
fn rounded_square(ctx: &CanvasRenderingContext2d, rect: Rect, radius: f64) -> Option<()> {
    let (left, top, right, bottom) = (rect.x, rect.y, rect.x + rect.size, rect.y + rect.size);
    ctx.begin_path();
    ctx.move_to(left + radius, top);
    ctx.arc_to(right, top, right, bottom, radius).ok()?;
    ctx.arc_to(right, bottom, left, bottom, radius).ok()?;
    ctx.arc_to(left, bottom, left, top, radius).ok()?;
    ctx.arc_to(left, top, right, top, radius).ok()?;
    ctx.close_path();
    Some(())
}

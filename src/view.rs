//! Шаблоны страницы. Имена классов совпадают с `css/style.css`.

use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::content::{
    CardBody, Messenger, PriceCard, PriceItem, Section, Shot, ShotLayout, Skill, ABOUT, BRAND,
    FIRST_NAME, INSTAGRAM_HANDLE, INSTAGRAM_URL, MESSENGERS, MOTTO, PHONE_LABEL, PHONE_URL, PRICE_CARDS, ROLE,
    SHOTS, SHOW_MESSENGERS, SKILLS, SURNAME,
};

/// WASM исполняется только со своего origin; сторонних скриптов, стилей и запросов нет.
const CSP: &str = "default-src 'none'; img-src 'self' data:; style-src 'self'; font-src 'self'; \
                   script-src 'self' 'wasm-unsafe-eval'; connect-src 'self'; base-uri 'none'; form-action 'none'";

const ICON_STROKE: &str =
    r#"aria-hidden="true" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round""#;

/// Вся страница целиком, от `<!doctype html>`.
pub fn page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="ru" {
            (head())
            body {
                a class="skip" href={ "#" (Section::Services.id()) } { "К содержанию" }
                (nav())
                main class="canvas" id="top" {
                    (hero())
                    (services())
                    (about())
                    (price())
                    (portfolio())
                    (contacts())
                }
                (lightbox())
                (share_dialog())
            }
        }
    }
}

fn head() -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta http-equiv="Content-Security-Policy" content=(CSP);
            meta name="referrer" content="strict-origin-when-cross-origin";
            meta name="theme-color" content="#131015";
            title { (SURNAME) " " (FIRST_NAME) " — парикмахер-колорист и визажист" }
            meta name="description" content="Окрашивание, сложное окрашивание, стрижки, укладки, мейк, уходы, брови и ресницы. Прайс, работы и запись.";
            link rel="preload" href="assets/fonts/oranienbaum-cyrillic.woff2" as="font" type="font/woff2" crossorigin;
            link rel="preload" href="assets/fonts/great-vibes-cyrillic.woff2" as="font" type="font/woff2" crossorigin;
            link rel="stylesheet" href="css/fonts.css";
            link rel="stylesheet" href="css/style.css?v=20261128";
            script type="module" src="boot.js" {}
        }
    }
}

fn nav() -> Markup {
    html! {
        header class="nav" id="nav" {
            a class="nav__brand" href="#top" { (BRAND) }
            button class="nav__toggle" type="button" aria-expanded="false" aria-controls="nav-panel" {
                span class="nav__toggle-text" { "Меню" }
                (icon(24, 22, "1.4", r#"<path class="bar bar--a" d="M3 8 H21"/><path class="bar bar--b" d="M3 16 H21"/>"#))
            }
            nav class="nav__panel" id="nav-panel" aria-label="Разделы" {
                ul class="nav__list" {
                    @for section in Section::ALL {
                        li { a href={ "#" (section.id()) } { (section.label()) } }
                    }
                }
                a class="btn btn--solid nav__cta" href=(INSTAGRAM_URL) target="_blank" rel="noopener noreferrer" { "Записаться" }
            }
        }
    }
}

/// Декоративное фото цветка. `depth` задаёт, насколько сильно слой смещается при прокрутке.
fn flower(modifier: &str, src: &str, width: u32, height: u32, depth: &str, lazy: bool) -> Markup {
    html! {
        img class={ "flower flower--" (modifier) } src=(src) alt="" width=(width) height=(height)
            loading=[lazy.then_some("lazy")] data-depth=(depth);
    }
}

fn hero() -> Markup {
    html! {
        section class="hero" {
            (flower("hero-lily", "assets/flora/lily.webp", 1500, 1000, ".10", false))
            (flower("hero-bloodroot", "assets/flora/bloodroot.webp", 1500, 1385, ".08", false))
            div class="wrap hero__wrap" {
                div class="hero__text" {
                    h1 class="name" {
                        span class="name__first" { (SURNAME) }
                        span class="name__last" { (FIRST_NAME) }
                    }
                    p class="hero__lead" { (ROLE) ". " (MOTTO) }
                    div class="hero__actions" {
                        a class="btn btn--solid" href=(INSTAGRAM_URL) target="_blank" rel="noopener noreferrer" { "Записаться в Instagram" }
                        a class="btn" href={ "#" (Section::Portfolio.id()) } { "Посмотреть работы" }
                    }
                }
            }
        }
    }
}

fn services() -> Markup {
    html! {
        section class="services" id=(Section::Services.id()) {
            (flower("buttonbush", "assets/flora/buttonbush.webp", 1500, 1000, ".16", true))
            div class="wrap" {
                h2 class="title" { "Что я умею" }
                p class="intro" { "Цвет, форма и лицо складываются в один образ. Поэтому всё это можно сделать у одного мастера." }
                div class="stage" {
                    img class="stage__bg stage__bg--pressed" src="assets/decor/pressed.webp" alt="" width="675" height="1200" loading="lazy";
                    ul class="skills glass" {
                        @for skill in SKILLS { (skill_row(skill)) }
                    }
                }
            }
        }
    }
}

fn skill_row(skill: &Skill) -> Markup {
    html! {
        li class="skill" {
            img class="skill__icon" src=(format!("{}?v=transparent", skill.icon)) alt="" loading="lazy" decoding="async";
            h3 { (skill.name) }
            div class="skill__body" {
                p { (skill.text) }
                ul class="skill__tags" {
                    @for tag in skill.tags { li { (tag) } }
                }
            }
        }
    }
}

fn about() -> Markup {
    html! {
        section class="about" id=(Section::About.id()) {
            (flower("turtlehead", "assets/flora/turtlehead.webp", 1500, 1000, ".14", true))
            (flower("goldenrod", "assets/flora/goldenrod.webp", 1500, 1250, ".08", true))
            div class="wrap about__wrap" {
                div class="about__text glass" {
                    h2 class="title" { "Обо мне" }
                    @for paragraph in ABOUT { p { (paragraph) } }
                }
                div class="about__side" {
                    figure class="portrait" {
                        div class="portrait__pane glass" {}
                        img class="portrait__photo" src="assets/img/vasilisa-640.webp"
                            srcset="assets/img/vasilisa-640.webp 481w, assets/img/vasilisa-1200.webp 901w"
                            sizes="(min-width: 860px) 32vw, 80vw" alt={ (FIRST_NAME) " " (SURNAME) }
                            width="901" height="1200" loading="lazy";
                        img class="portrait__bloom" src="assets/flora/yucca.webp" alt="" width="1500" height="984"
                            loading="lazy" data-depth=".05";
                    }
                    p class="about__quote script" { "Утром без борьбы с зеркалом" }
                }
            }
        }
    }
}

fn price() -> Markup {
    html! {
        section class="price" id=(Section::Price.id()) {
            (flower("nettle", "assets/flora/nettle.webp", 1500, 1090, ".18", true))
            (flower("birds", "assets/decor/birds.webp", 1199, 1799, ".10", true))
            (flower("holly", "assets/flora/holly.webp", 1500, 1000, ".12", true))
            div class="wrap" {
                h2 class="title" { "Прайс" }
                p class="intro" { "Цены в рублях. Точную стоимость сложного окрашивания называю после консультации: она зависит от длины и густоты волос." }
                div class="bento" {
                    @for card in PRICE_CARDS { (price_card(card)) }
                }
            }
        }
    }
}

fn price_card(card: &PriceCard) -> Markup {
    html! {
        section class={ "card glass card--" (card.slot) } {
            h3 { (card.title) }
            @match &card.body {
                CardBody::List(items) => { ul { (price_rows(items)) } }
                CardBody::Big(items) => { ul class="card__big" { (price_rows(items)) } }
                CardBody::Split(first, second) => {
                    div class="card__split" {
                        ul { (price_rows(first)) }
                        ul { (price_rows(second)) }
                    }
                }
                CardBody::Range { price, note } => {
                    p class="card__range" { (price) }
                    p class="card__note" { (note) }
                }
            }
        }
    }
}

fn price_rows(items: &[PriceItem]) -> Markup {
    html! {
        @for item in items {
            li { span { (item.label) } b { (item.price) } }
        }
    }
}

fn portfolio() -> Markup {
    html! {
        section class="portfolio" id=(Section::Portfolio.id()) {
            (flower("basswood", "assets/flora/basswood.webp", 1500, 1000, ".15", true))
            div class="wrap" {
                h2 class="title" { "Работы" }
                p class="intro" { "Наведите на фото, чтобы узнать, что на нём сделано. Нажмите, чтобы рассмотреть ближе." }
                ul class="shots" {
                    @for shot in SHOTS { (shot_tile(shot)) }
                }
            }
        }
    }
}

fn shot_tile(shot: &Shot) -> Markup {
    let (modifier, sizes) = match shot.layout {
        ShotLayout::Big => ("big", "(min-width: 900px) 50vw, 100vw"),
        ShotLayout::Standard => ("standard", "(min-width: 900px) 25vw, 50vw"),
        ShotLayout::Top => ("top", "(min-width: 900px) 25vw, 50vw"),
    };
    let small = format!("assets/img/{}-700.webp", shot.slug);
    let full = format!("assets/img/{}-1400.webp", shot.slug);
    let srcset = format!("{small} {}w, {full} {}w", shot.small_width, shot.width);
    html! {
        li class={ "shot shot--" (modifier) } {
            button type="button" data-full=(full) {
                img src=(small) srcset=(srcset) sizes=(sizes) width=(shot.width) height=(shot.height)
                    alt=(shot.alt) loading="lazy" decoding="async";
                span class="shot__veil" {
                    strong { (shot.caption) }
                }
            }
        }
    }
}

fn contacts() -> Markup {
    html! {
        section class="contacts" id=(Section::Contacts.id()) {
            img class="fresco" src="assets/decor/fresco.webp" alt="" width="1800" height="817" loading="lazy";
            div class="wrap" {
                div class="stage" {
                    div class="contacts__sheet glass" {
                        h2 class="title" { "Запись и контакты" }
                        p class="intro" { "Напишите в Instagram: расскажите, что хотите сделать, и пришлите фото волос сейчас. Отвечу и подберу время." }
                        a class="insta" href=(INSTAGRAM_URL) target="_blank" rel="noopener noreferrer" {
                            (icon(48, 44, "1.6", r#"<rect x="7" y="7" width="34" height="34" rx="10"/><circle cx="24" cy="24" r="8"/><circle cx="34" cy="14" r="1.2" fill="currentColor"/>"#))
                            span { (INSTAGRAM_HANDLE) }
                        }
                        div class="contacts__actions" {
                            a class="btn" href=(PHONE_URL) {
                                (icon(24, 22, "1.4", r#"<path d="M7 3 L10 8 L7.5 10.5 C9 13 11 15 13.5 16.5 L16 14 L21 17 V19 C21 20 20 21 19 21 C10 21 3 14 3 5 C3 4 4 3 5 3 Z"/>"#))
                                span { (PHONE_LABEL) }
                            }
                            // Скрыта, пока WASM не загрузился: без него QR не построить.
                            button class="btn" id="share-open" type="button" hidden {
                                (icon(48, 22, "2.4", r#"<rect x="7" y="7" width="13" height="13" rx="2"/><rect x="28" y="7" width="13" height="13" rx="2"/><rect x="7" y="28" width="13" height="13" rx="2"/><path d="M28 28 H34 V34 M41 28 V30 M28 41 H30 M36 41 H41 V36"/>"#))
                                span { "QR-код сайта" }
                            }
                            @if SHOW_MESSENGERS {
                                @for messenger in MESSENGERS { (messenger_link(messenger)) }
                            }
                        }
                    }
                }
            }
            footer class="foot" {
                p class="foot__sign" { "Beauty starts here" }
                p class="foot__name" { (BRAND) }
            }
        }
    }
}

fn messenger_link(messenger: &Messenger) -> Markup {
    html! {
        @if let Some(url) = messenger.url {
            a class="btn" href=(url) target="_blank" rel="noopener noreferrer" {
                (icon(48, 22, "2", messenger.icon))
                span { (messenger.label) }
            }
        }
    }
}

fn lightbox() -> Markup {
    html! {
        dialog class="lightbox" id="lightbox" aria-label="Просмотр работы" {
            (close_button())
            figure {
                img id="lightbox-img" alt="";
                figcaption id="lightbox-cap" {}
            }
        }
    }
}

fn share_dialog() -> Markup {
    html! {
        dialog class="share" id="share" aria-labelledby="share-title" {
            (close_button())
            div class="share__body glass" {
                h2 class="share__title" id="share-title" { "Поделиться сайтом" }
                canvas class="share__card" id="share-canvas" width="1080" height="1440" role="img" aria-label="Карточка с QR-кодом сайта" {}
                p class="share__url" id="share-url" {}
                div class="share__actions" {
                    a class="btn btn--solid" id="share-download" download="oficerova-qr.png" { "Скачать картинку" }
                    button class="btn" id="share-native" type="button" hidden { "Отправить ссылку" }
                }
            }
        }
    }
}

fn close_button() -> Markup {
    html! {
        button class="dialog-close" type="button" aria-label="Закрыть" data-close {
            (icon(24, 26, "1.4", r#"<path d="M5 5 L19 19"/><path d="M19 5 L5 19"/>"#))
        }
    }
}

/// Линейная SVG-иконка: квадратный `view_box`, отрисованный размер `size` в пикселях.
fn icon(view_box: u32, size: u32, stroke_width: &str, shapes: &str) -> Markup {
    PreEscaped(format!(
        r#"<svg viewBox="0 0 {view_box} {view_box}" width="{size}" height="{size}" stroke-width="{stroke_width}" {ICON_STROKE}>{shapes}</svg>"#
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_contains_every_section_and_navigation_link() {
        let html = page().into_string();
        assert!(html.starts_with("<!DOCTYPE html>"));
        for section in Section::ALL {
            assert!(html.contains(&format!(r#"id="{}""#, section.id())), "нет секции {section:?}");
            assert!(html.contains(&format!(r##"href="#{}""##, section.id())), "нет ссылки на {section:?}");
        }
    }

    #[test]
    fn page_lists_shots_in_order_without_numbers() {
        let html = portfolio().into_string();
        let mut remaining = html.as_str();
        for shot in SHOTS {
            let asset = format!("assets/img/{}-1400.webp", shot.slug);
            let position = remaining.find(&asset).expect("portfolio photo is missing or out of order");
            remaining = &remaining[position + asset.len()..];
            assert!(html.contains(shot.caption));
        }
        assert!(!html.contains('№'));
    }

    #[test]
    fn text_content_is_escaped() {
        let markup = html! { p { ("<script>alert(1)</script>") } }.into_string();
        assert!(!markup.contains("<script>"));
    }

    #[test]
    fn messengers_stay_hidden_until_enabled() {
        let html = page().into_string();
        assert_eq!(html.contains("t.me/voficerova"), SHOW_MESSENGERS);
    }
}

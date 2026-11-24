//! Глубина: цветы под стеклом движутся медленнее страницы и чуть отзываются на курсор.
//!
//! Каждый слой с атрибутом `data-depth` плавно догоняет свою цель, а не привязан к прокрутке
//! жёстко: движение получает инерцию и не дёргается на колесе мыши. Кадры запрашиваются,
//! только пока хоть один слой ещё в пути.

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, MouseEvent, Window};

use super::dom;

/// Какую долю оставшегося пути слой проходит за кадр. Меньше — мягче и дольше.
const EASE: f64 = 0.085;
/// Максимальный сдвиг от курсора в пикселях для слоя с глубиной 1.
const POINTER_REACH_PX: f64 = 110.0;
/// Наклон в градусах на пиксель вертикального и горизонтального смещения.
const TILT_PER_PX: (f64, f64) = (0.012, 0.03);
/// Насколько слой с глубиной 1 «приближается», проходя центр экрана.
const SWELL: f64 = 0.35;
/// Ближе этого расстояния слой считается дошедшим до цели.
const SETTLED_PX: f64 = 0.15;

/// Ячейка для замыкания кадра анимации: пуста, пока оно не создано.
type FrameSlot = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;

struct Layer {
    element: HtmlElement,
    depth: f64,
    x: f64,
    y: f64,
}

struct Scene {
    window: Window,
    layers: Vec<Layer>,
    /// Положение курсора по горизонтали и вертикали, от −1 до 1 относительно центра окна.
    pointer: (f64, f64),
    animating: bool,
}

impl Scene {
    /// Сдвигает слои к целям. `ease = 1.0` ставит их на место сразу. Возвращает `true`,
    /// когда все слои дошли.
    fn step(&mut self, ease: f64) -> bool {
        let height = dom::viewport_height(&self.window).max(1.0);
        let middle = height / 2.0;
        let mut settled = true;

        for layer in &mut self.layers {
            let Some(parent) = layer.element.parent_element() else { continue };
            let frame = parent.get_bounding_client_rect();
            let target_y = (middle - (frame.top() + frame.height() / 2.0)) * layer.depth
                + self.pointer.1 * layer.depth * POINTER_REACH_PX * 0.5;
            let target_x = self.pointer.0 * layer.depth * POINTER_REACH_PX;

            layer.x += (target_x - layer.x) * ease;
            layer.y += (target_y - layer.y) * ease;
            settled &= (target_x - layer.x).abs() < SETTLED_PX && (target_y - layer.y).abs() < SETTLED_PX;

            let closeness = 1.0 - (layer.y / (layer.depth * height)).abs().min(1.0);
            let scale = 1.0 + layer.depth * SWELL * closeness;
            let tilt = layer.y * TILT_PER_PX.0 + layer.x * TILT_PER_PX.1;
            let transform = format!(
                "translate3d({:.2}px, {:.2}px, 0) rotate({:.3}deg) scale({:.4})",
                layer.x, layer.y, tilt, scale
            );
            let _ = layer.element.style().set_property("transform", &transform);
        }
        settled
    }
}

pub fn init() -> Option<()> {
    let window = dom::window()?;
    let reduced_motion = window.match_media("(prefers-reduced-motion: reduce)").ok()??.matches();
    if reduced_motion {
        return None;
    }

    let layers: Vec<Layer> = dom::query_all("[data-depth]")
        .into_iter()
        .filter_map(|element| {
            let depth: f64 = element.get_attribute("data-depth")?.parse().ok()?;
            let element = element.dyn_into::<HtmlElement>().ok()?;
            (depth > 0.0).then_some(Layer { element, depth, x: 0.0, y: 0.0 })
        })
        .collect();
    if layers.is_empty() {
        return None;
    }

    let scene = Rc::new(RefCell::new(Scene { window: window.clone(), layers, pointer: (0.0, 0.0), animating: false }));
    scene.borrow_mut().step(1.0);

    // Кадр перезапрашивает сам себя, поэтому замыкание хранится там, откуда само себя достанет.
    let frame: FrameSlot = Rc::new(RefCell::new(None));
    let request_frame = {
        let (window, frame) = (window.clone(), frame.clone());
        move || {
            if let Some(callback) = frame.borrow().as_ref() {
                let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
            }
        }
    };
    *frame.borrow_mut() = Some(Closure::new({
        let (scene, request_frame) = (scene.clone(), request_frame.clone());
        move || {
            let settled = scene.borrow_mut().step(EASE);
            if settled {
                scene.borrow_mut().animating = false;
            } else {
                request_frame();
            }
        }
    }));

    let wake = {
        let scene = scene.clone();
        move || {
            let mut scene = scene.borrow_mut();
            if !scene.animating {
                scene.animating = true;
                request_frame();
            }
        }
    };

    {
        let wake = wake.clone();
        dom::on_passive(&window, "scroll", move |_: Event| wake());
    }
    {
        let wake = wake.clone();
        dom::on(&window, "resize", move |_: Event| wake());
    }
    // `mousemove` есть только у мыши и трекпада: на телефоне слои следуют одной прокрутке.
    dom::on_passive(&window.clone(), "mousemove", move |event: MouseEvent| {
        let (width, height) = (dom::viewport_width(&window).max(1.0), dom::viewport_height(&window).max(1.0));
        scene.borrow_mut().pointer = (
            f64::from(event.client_x()) / width * 2.0 - 1.0,
            f64::from(event.client_y()) / height * 2.0 - 1.0,
        );
        wake();
    });

    Some(())
}

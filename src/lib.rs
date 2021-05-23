use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const FONT_FAMILY: &str = "monospace";
const FONT_SIZE: usize = 30;
const FONT_COLOR: &str = "#f8f8f2";
const BACKGROUND_COLOR: &str = "#282a36";

struct Cursor {
    x: usize,
    y: usize,
}

fn set_canvas_size(canvas: &web_sys::HtmlCanvasElement, height: u32, width: u32) {
    canvas.set_height(height as u32);
    canvas.set_width(width as u32);
}

fn write(context: &web_sys::CanvasRenderingContext2d, cursor: &mut Cursor, text: &str) {
    context.set_fill_style(&JsValue::from_str(FONT_COLOR));
    context.fill_text(text, cursor.x as f64, cursor.y as f64).unwrap();
    let font_width = get_font_width(&context);
    cursor.x = cursor.x + (text.len() * font_width);
}

fn line_break(cursor: &mut Cursor) {
    cursor.x = 0;
    cursor.y = cursor.y + FONT_SIZE;
}

fn erase(context: &web_sys::CanvasRenderingContext2d, x: f64, y: f64) {
    let font_width = get_font_width(&context);

    context.set_fill_style(&JsValue::from_str(BACKGROUND_COLOR));
    context.fill_rect(x - font_width as f64, y - FONT_SIZE as f64, font_width as f64, FONT_SIZE as f64);
}

fn backspace(context: &web_sys::CanvasRenderingContext2d, cursor: &mut Cursor) {
    let font_width = get_font_width(&context);
    erase(&context, cursor.x as f64, cursor.y as f64);

    if cursor.x == 0 {
        cursor.x = 0; // TODO: move to end of line
        cursor.y = cursor.y - FONT_SIZE;
    } else {
        cursor.x = cursor.x - font_width;
        cursor.y = cursor.y;
    }
}

// TODO: calculate font width only once
fn get_font_width(context: &web_sys::CanvasRenderingContext2d) -> usize {
    let text_metrics: web_sys::TextMetrics = context.measure_text(" ").unwrap();
    text_metrics.width() as usize
}

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let screen_height: u32 = window.inner_height().unwrap().as_f64().unwrap() as u32;
    let screen_width: u32 = window.inner_width().unwrap().as_f64().unwrap() as u32;

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    set_canvas_size(&canvas, screen_height, screen_width);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();


    // set background color
    context.set_fill_style(&JsValue::from_str(BACKGROUND_COLOR));
    context.fill_rect(0.0, 0.0, screen_width as f64, screen_height as f64);

    // set font color
    context.set_font(&format!("{}px {}", FONT_SIZE, FONT_FAMILY));

    let mut cursor = Cursor {
        x: 0,
        y: FONT_SIZE,
    };

    write(&context, &mut cursor, "Hello from Rust!");

    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let key = &event.key();

        log(&format!("{} pressed. Cursor at {}, {}", key, cursor.x, cursor.y));

        match key.as_str() {
            "Shift" => (),
            "Alt" => (),
            "Control" => (),
            "Enter" => line_break(&mut cursor),
            "Backspace" => backspace(&context, &mut cursor),
            _ => write(&context, &mut cursor, key),
        }
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

mod colors;
mod shape;
mod wheel;

use crate::wheel::{WheelOptions, make_wheel};
use std::f64;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;

#[wasm_bindgen]
pub fn draw_emotions_wheel(element_id: String) -> Result<(), JsValue> {
    wasm_log::init(wasm_log::Config::default());

    let document = window().unwrap().document().unwrap();
    let canvas_el = document.get_element_by_id(&element_id).unwrap();

    {
        let canvas: web_sys::HtmlCanvasElement = canvas_el
            .clone()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let width = window().unwrap().inner_width()?.as_f64().unwrap() as u32 * 4;
        let height = window().unwrap().inner_height()?.as_f64().unwrap() as u32 * 4;
        let wo = WheelOptions {
            width: width as u32,
            height: height as u32,
            radius: f64::min(width as f64, height as f64) * 0.4,
        };
        make_wheel(wo, &canvas)?;

        canvas_el.set_attribute("style", &format!("width: {}px", width / 4))?;
    }

    Ok(())
}

mod shape;

use crate::shape::{Arc, Draw, DrawOptions, DrawTextOptions, Point};
use std::f64;
use emotions::build_emotion_tree;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::Math::min;
use web_sys::window;

#[wasm_bindgen]
pub async fn okok() -> Result<(), JsValue> {
    wasm_log::init(wasm_log::Config::default());

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let inner_width = window().unwrap().inner_width()?.as_f64().unwrap() as u32;
    let inner_height = window().unwrap().inner_height()?.as_f64().unwrap() as u32;

    canvas.set_width(inner_width);
    canvas.set_height(inner_height);

    let mut context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let center = Point(inner_width as f64 / 2.0, inner_height as f64 / 2.0);

    fn color() -> String {
        random_color::RandomColor::new().to_rgb_string()
    }

    let emotions_tree = build_emotion_tree();
    let depth_0 = emotions_tree.nodes_at_depth(3);

    let mut draw_options = DrawOptions {
        fill_style: Some(color()),
        line_width: Some(2.5),
        stroke_style: Some(String::from("white")),
    };

    let draw_text_options = DrawTextOptions {
        font: Some(String::from("10px sans-serif")),
        fill_style: Some(String::from("black")),
    };

    let num = depth_0.len();
    let radius = min(inner_width as f64, inner_height as f64) * 0.4;
    for idx in 0..num  {
        draw_options.fill_style = Some(color());

        let sliver = Arc::make_sliver(idx, num, radius, center.clone());
        sliver.draw(&draw_options, &mut context)?;

        sliver.scale(0.4)
            .draw(&draw_options, &mut context)?
            .draw_text(depth_0[idx].name, &draw_text_options, &mut context)?;

        sliver.draw_text("lallaal", &draw_text_options, &mut context)?;
    }

    context.set_fill_style_str("white");
    context.arc(center.0, center.1, radius * 0.2, 0.0, 2.0 * f64::consts::PI)?;
    context.fill();

    // context.restore();

    // context.move_to(0.0, 0.0);
    // context.line_to(0.0, 20.0);
    // context.set_line_width(5.0);
    // context.set_stroke_style_str("green");
    // context.stroke();

    Ok(())
}

pub async fn sleep(millis: i32) {
    let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, millis)
            .unwrap();
    };
    let p = js_sys::Promise::new(&mut cb);
    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}

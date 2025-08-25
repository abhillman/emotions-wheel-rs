use crate::colors::COLOR_PALETTE;
use crate::shape::{Arc, Circle, Draw, DrawOptions, DrawTextOptions, Point};
use emotions::build_emotion_tree;
use wasm_bindgen::{JsCast, JsValue};

pub struct WheelOptions {
    pub width: u32,
    pub height: u32,
    pub radius: f64,
}

impl WheelOptions {
    fn center(&self) -> Point {
        Point(self.width as f64 / 2.0, self.height as f64 / 2.0)
    }

    fn as_circle(&self) -> Circle {
        Circle {
            center: self.center(),
            radius: self.radius,
        }
    }
}

pub fn make_wheel(
    wheel_options: WheelOptions,
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<(), JsValue> {
    canvas.set_width(wheel_options.width);
    canvas.set_height(wheel_options.height);

    let mut context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let mut color_idx = 0;
    let mut get_color = || {
        let color = COLOR_PALETTE[color_idx % COLOR_PALETTE.len()].to_owned();
        color_idx += 1;
        color
    };

    let get_draw_options = |color: String| -> DrawOptions {
        DrawOptions {
            fill_style: Some(color),
            line_width: Some(2.5),
            stroke_style: Some(String::from("white")),
        }
    };

    let get_draw_text_options = |_radius: f64| -> DrawTextOptions {
        let font_size = wheel_options.radius / 38.0;
        DrawTextOptions {
            font: Some(format!("{font_size}px sans-serif")),
            fill_style: Some(String::from("black")),
            radius: _radius,
        }
    };

    let emotions_tree = build_emotion_tree();
    let depth_2 = emotions_tree.nodes_at_depth(2);
    let depth_3 = emotions_tree.nodes_at_depth(3);

    let mut d2_idx = 0;
    let mut d3_idx = 0;

    let circle = wheel_options.as_circle();
    for child1 in &emotions_tree.children {
        let last_d3_idx = d3_idx;
        let color = get_color();

        for child2 in &child1.children {
            for child3 in &child2.children {
                let sliver = Arc::from_circle(d3_idx, depth_3.len(), &circle);
                sliver.draw(&get_draw_options(color.clone()), &mut context)?;
                sliver.draw_text(
                    child3.name,
                    &get_draw_text_options(circle.scale(0.85).radius),
                    &mut context,
                )?;
                d3_idx += 1;
            }
            let circle = circle.scale(0.7);
            let sliver = Arc::from_circle(d2_idx, depth_2.len(), &circle);
            sliver.draw(&get_draw_options(color.clone()), &mut context)?;
            sliver.draw_text(
                child2.name,
                &get_draw_text_options(circle.scale(0.78).radius),
                &mut context,
            )?;
            d2_idx += 1;
        }
        let circle = circle.scale(0.38);
        let sliver0 = Arc::from_circle(last_d3_idx, depth_3.len(), &circle);
        let sliver1 = Arc::from_circle(d3_idx - 1, depth_3.len(), &circle);
        let sliver = Arc::join(sliver0, sliver1).unwrap();
        sliver.draw(&get_draw_options(color), &mut context)?;
        sliver.draw_text(
            child1.name,
            &get_draw_text_options(circle.radius * 0.71),
            &mut context,
        )?;
    }

    context.fill();

    context.set_fill_style_str("white");
    let circle = wheel_options.as_circle();
    context.arc(
        circle.center.0,
        circle.center.1,
        circle.radius * 0.15,
        0.0,
        2.0 * std::f64::consts::PI,
    )?;
    context.fill();

    Ok(())
}

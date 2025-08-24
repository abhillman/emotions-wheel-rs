use anyhow::Result;
use log::warn;
use js_sys::Math::{cos, sin};
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

type Number = f64;

#[derive(Debug, Clone)]
pub struct Point(pub Number, pub Number);

#[derive(Debug)]
pub struct Arc {
    start_angle: Number,
    end_angle: Number,
    radius: Number,
    center: Point,
}

impl Arc {
    pub fn make_sliver(n: usize, total: usize, radius: Number, center: Point) -> Arc {
        Arc {
            start_angle: std::f64::consts::PI * (2.0 / total as f64) * n as f64 - 0.5 * std::f64::consts::PI,
            end_angle: std::f64::consts::PI * (2.0 / total as f64) * ((n as f64) + 1.0) - 0.5 * std::f64::consts::PI,
            radius,
            center,
        }
    }

    pub fn scale(&self, scale: f64) -> Arc {
        match self {
            Arc {
                start_angle,
                end_angle,
                radius,
                center,
            } => Arc {
                start_angle: start_angle.to_owned(),
                end_angle: end_angle.to_owned(),
                radius: radius * scale,
                center: center.to_owned(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrawOptions {
    pub fill_style: Option<String>,
    pub line_width: Option<Number>,
    pub stroke_style: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DrawTextOptions {
    pub fill_style: Option<String>,
    pub font: Option<String>,
}

impl Default for DrawOptions {
    fn default() -> DrawOptions {
        DrawOptions {
            fill_style: None, // TODO: colors
            line_width: Some(1.0),
            stroke_style: Some(String::from("white")),
        }
    }
}

impl Draw for Arc {
    fn draw(
        &self,
        draw_options: &DrawOptions,
        context: &mut CanvasRenderingContext2d,
    ) -> Result<&Self, JsValue> {
        context.save();

        context.begin_path();
        context.move_to(self.center.0, self.center.1);

        context.arc(
            self.center.0,
            self.center.1,
            self.radius,
            self.start_angle,
            self.end_angle,
        )?;

        if let Some(fill_style) = &draw_options.fill_style {
            warn!("fill style: {:?}", fill_style);
            context.set_fill_style_str(fill_style);
        }

        if let Some(line_width) = draw_options.line_width {
            warn!("line width: {:?}", line_width);
            context.set_line_width(line_width);
        }

        if let Some(stroke) = &draw_options.stroke_style {
            warn!("stroke style: {:?}", stroke);
            context.set_stroke_style_str(stroke);
        }

        context.stroke();
        context.close_path();
        context.fill();

        context.move_to(self.center.0, self.center.1);
        context.stroke();

        context.restore();

        Ok(self)
    }

    fn draw_text<'a>(&self, text: &'a str, draw_options: &DrawTextOptions, context: &mut CanvasRenderingContext2d) -> Result<&Self, JsValue> {
        let x = self.center.0 + self.radius * 0.7 * cos((self.start_angle + self.end_angle) / 2.0);
        let y = self.center.1 + self.radius * 0.7 * sin((self.start_angle + self.end_angle) / 2.0);

        context.save();
        context.begin_path();
        context.translate(x, y)?;
        let angle = (self.start_angle + self.end_angle) / 2.0;
        context.rotate(angle)?;

        if angle > 0.5 * std::f64::consts::PI && angle < 1.5 * std::f64::consts::PI {
            context.scale(-1.0, -1.0)?;
        }

        if let Some(fill_style) = &draw_options.fill_style {
            context.set_fill_style_str(fill_style);
        }

        if let Some(font) = &draw_options.font {
            context.set_font(font);
        }

        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.fill_text(text, 0.0, 0.0)?;
        context.close_path();
        context.restore();

        Ok(self)
    }
}

pub trait Draw {
    fn draw(
        &self,
        draw_options: &DrawOptions,
        context: &mut CanvasRenderingContext2d,
    ) -> Result<&Self, JsValue>;

    fn draw_text(
        &self,
        text: &str,
        draw_options: &DrawTextOptions,
        context: &mut CanvasRenderingContext2d,
    ) -> Result<&Self, JsValue>;
}

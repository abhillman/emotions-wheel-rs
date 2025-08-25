use anyhow::{Result, bail};
use js_sys::Math::{cos, sin};
use std::cmp::PartialEq;
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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

pub struct Circle {
    pub radius: Number,
    pub center: Point,
}

impl Circle {
    pub fn scale(&self, scale: f64) -> Circle {
        Circle {
            radius: self.radius * scale,
            center: self.center.clone(),
        }
    }
}

impl Arc {
    pub fn from_circle(n: usize, total: usize, circle: &Circle) -> Arc {
        Arc::make_sliver(n, total, circle.radius, circle.center.clone())
    }

    pub fn make_sliver(n: usize, total: usize, radius: Number, center: Point) -> Arc {
        Arc {
            start_angle: std::f64::consts::PI * (2.0 / total as f64) * n as f64
                - 0.5 * std::f64::consts::PI,
            end_angle: std::f64::consts::PI * (2.0 / total as f64) * ((n as f64) + 1.0)
                - 0.5 * std::f64::consts::PI,
            radius,
            center,
        }
    }

    #[allow(dead_code)]
    pub fn join(arc0: Arc, arc2: Arc) -> Result<Arc> {
        if arc0.radius != arc2.radius {
            bail!("radius mismatch");
        }

        if arc0.center != arc2.center {
            bail!("center mismatch");
        }

        let start_angle = f64::min(arc0.start_angle, arc2.start_angle);
        let end_angle = f64::max(arc0.end_angle, arc2.end_angle);
        let radius = arc0.radius;
        let center = arc0.center;

        Ok(Arc {
            start_angle,
            end_angle,
            radius,
            center,
        })
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
    pub radius: Number,
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
            context.set_fill_style_str(fill_style);
        }

        if let Some(line_width) = draw_options.line_width {
            context.set_line_width(line_width);
        }

        if let Some(stroke) = &draw_options.stroke_style {
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

    fn draw_text(
        &self,
        text: &str,
        draw_options: &DrawTextOptions,
        context: &mut CanvasRenderingContext2d,
    ) -> Result<&Self, JsValue> {
        let x =
            self.center.0 + draw_options.radius * cos((self.start_angle + self.end_angle) / 2.0);
        let y =
            self.center.1 + draw_options.radius * sin((self.start_angle + self.end_angle) / 2.0);

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

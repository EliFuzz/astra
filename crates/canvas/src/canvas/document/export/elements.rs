use super::points::relative_points;
use crate::shapes::{
    Arrow, Diamond, Ellipse, Freehand, Line, PathStyle, Rectangle, Shape, ShapeStyle, Text,
};
use kurbo::Point;
use serde_json::Value;

const CLOSED_PATH_THRESHOLD: f64 = 10.0;

pub(in crate::canvas::document::export) fn shape_from_element(
    elem: &Value,
    elem_type: &str,
    x: f64,
    y: f64,
    style: ShapeStyle,
) -> Option<Shape> {
    match elem_type {
        "rectangle" => {
            let width = elem.get("width").and_then(|v| v.as_f64()).unwrap_or(100.0);
            let height = elem.get("height").and_then(|v| v.as_f64()).unwrap_or(100.0);
            let mut rect = Rectangle::new(Point::new(x, y), width, height);
            rect.style = style;
            if elem.get("roundness").is_some() {
                rect.corner_radius = Rectangle::DEFAULT_ADAPTIVE_RADIUS
                    .min(width / 4.0)
                    .min(height / 4.0);
            }
            Some(Shape::Rectangle(rect))
        }
        "diamond" => {
            let width = elem.get("width").and_then(|v| v.as_f64()).unwrap_or(100.0);
            let height = elem.get("height").and_then(|v| v.as_f64()).unwrap_or(100.0);
            let mut diamond = Diamond::new(Point::new(x, y), width, height);
            diamond.style = style;
            Some(Shape::Diamond(diamond))
        }
        "ellipse" => {
            let width = elem.get("width").and_then(|v| v.as_f64()).unwrap_or(100.0);
            let height = elem.get("height").and_then(|v| v.as_f64()).unwrap_or(100.0);
            let center = Point::new(x + width / 2.0, y + height / 2.0);
            let mut ellipse = Ellipse::new(center, width / 2.0, height / 2.0);
            ellipse.style = style;
            Some(Shape::Ellipse(ellipse))
        }
        "freedraw" => {
            let freehand_points = relative_points(elem, x, y);
            if freehand_points.is_empty() {
                return None;
            }
            let mut freehand = Freehand::from_points(freehand_points.clone());
            freehand.style = style;
            if freehand_points.len() >= 3 {
                let first = freehand_points.first().unwrap();
                let last = freehand_points.last().unwrap();
                if (first.x - last.x).abs() < CLOSED_PATH_THRESHOLD
                    && (first.y - last.y).abs() < CLOSED_PATH_THRESHOLD
                {
                    freehand.closed = true;
                }
            }
            Some(Shape::Freehand(freehand))
        }
        "line" => {
            let line_points = relative_points(elem, x, y);
            if line_points.len() < 2 {
                return None;
            }
            let path_style = if elem.get("roundness").map(|r| !r.is_null()).unwrap_or(false) {
                PathStyle::Flowing
            } else {
                PathStyle::Direct
            };
            let mut line = Line::from_points(line_points.clone(), path_style);
            line.style = style;
            if line_points.len() >= 3 {
                let first = line_points.first().unwrap();
                let last = line_points.last().unwrap();
                if (first.x - last.x).abs() < CLOSED_PATH_THRESHOLD
                    && (first.y - last.y).abs() < CLOSED_PATH_THRESHOLD
                {
                    line.closed = true;
                }
            }
            Some(Shape::Line(line))
        }
        "arrow" => {
            let arrow_points = relative_points(elem, x, y);
            if arrow_points.len() < 2 {
                return None;
            }
            let elbowed = elem
                .get("elbowed")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let has_roundness = elem.get("roundness").map(|r| !r.is_null()).unwrap_or(false);
            let path_style = if elbowed {
                PathStyle::Angular
            } else if has_roundness {
                PathStyle::Flowing
            } else {
                PathStyle::Direct
            };
            let mut arrow = Arrow::from_points(arrow_points, path_style);
            arrow.style = style;
            Some(Shape::Arrow(arrow))
        }
        "text" => {
            let content = elem
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let font_size = elem
                .get("fontSize")
                .and_then(|v| v.as_f64())
                .unwrap_or(20.0);
            let mut text = Text::new(Point::new(x, y), content);
            text.font_size = font_size;
            text.style = style;
            Some(Shape::Text(text))
        }
        _ => None,
    }
}

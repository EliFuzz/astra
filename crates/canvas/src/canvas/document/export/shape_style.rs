use super::color::parse_hex_color;
use crate::shapes::{FillPattern, ShapeStyle, Sloppiness};
use serde_json::Value;

pub(in crate::canvas::document::export) fn shape_style_from_element(elem: &Value) -> ShapeStyle {
    let stroke_color = parse_hex_color(
        elem.get("strokeColor")
            .and_then(|v| v.as_str())
            .unwrap_or("#000000"),
    );
    let bg_color = elem
        .get("backgroundColor")
        .and_then(|v| v.as_str())
        .unwrap_or("transparent");
    let fill_color = if bg_color == "transparent" {
        None
    } else {
        Some(parse_hex_color(bg_color))
    };

    let stroke_width = elem
        .get("strokeWidth")
        .and_then(|v| v.as_f64())
        .unwrap_or(2.0);
    let roughness = elem.get("roughness").and_then(|v| v.as_i64()).unwrap_or(1);
    let sloppiness = match roughness {
        0 => Sloppiness::Architect,
        1 => Sloppiness::Artist,
        _ => Sloppiness::Cartoonist,
    };

    let fill_pattern = match elem
        .get("fillStyle")
        .and_then(|v| v.as_str())
        .unwrap_or("solid")
    {
        "hachure" => FillPattern::Hachure,
        "cross-hatch" => FillPattern::CrossHatch,
        "zigzag" => FillPattern::ZigZag,
        _ => FillPattern::Solid,
    };

    ShapeStyle {
        stroke_color,
        stroke_width,
        fill_color,
        fill_pattern,
        sloppiness,
        seed: elem.get("seed").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
        opacity: elem
            .get("opacity")
            .and_then(|v| v.as_f64())
            .map(|v| if v > 1.0 { v / 100.0 } else { v })
            .unwrap_or(1.0),
    }
}

use kurbo::Point;
use serde_json::Value;

pub(in crate::canvas::document::export) fn relative_points(elem: &Value, x: f64, y: f64) -> Vec<Point> {
    let Some(pts) = elem.get("points").and_then(|p| p.as_array()) else {
        return Vec::new();
    };
    pts.iter()
        .filter_map(|p| p.as_array())
        .filter_map(|arr| {
            let px = arr.first().and_then(|v| v.as_f64())?;
            let py = arr.get(1).and_then(|v| v.as_f64())?;
            Some(Point::new(x + px, y + py))
        })
        .collect()
}

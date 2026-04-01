use kurbo::Rect;

const GUIDE_AXIS_MATCH_EPS: f64 = 0.1;

pub fn collect_for_vertical_guide(guide_x: f64, other_bounds: &[Rect]) -> Vec<f64> {
    let mut out = Vec::new();
    for ob in other_bounds {
        let ob_cx = (ob.x0 + ob.x1) / 2.0;
        for ob_x in [ob.x0, ob.x1, ob_cx] {
            if (ob_x - guide_x).abs() < GUIDE_AXIS_MATCH_EPS {
                out.extend([ob.y0, ob.y1, (ob.y0 + ob.y1) / 2.0]);
            }
        }
    }
    out
}

pub fn collect_for_horizontal_guide(guide_y: f64, other_bounds: &[Rect]) -> Vec<f64> {
    let mut out = Vec::new();
    for ob in other_bounds {
        let ob_cy = (ob.y0 + ob.y1) / 2.0;
        for ob_y in [ob.y0, ob.y1, ob_cy] {
            if (ob_y - guide_y).abs() < GUIDE_AXIS_MATCH_EPS {
                out.extend([ob.x0, ob.x1, (ob.x0 + ob.x1) / 2.0]);
            }
        }
    }
    out
}

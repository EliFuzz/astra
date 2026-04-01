use kurbo::Point;

#[derive(Debug, Clone)]
pub struct SmartGuide {
    pub kind: SmartGuideKind,
    pub position: f64,
    pub start: f64,
    pub end: f64,
    pub snap_points: Vec<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmartGuideKind {
    Vertical,
    Horizontal,
    EqualSpacingH,
    EqualSpacingV,
}

#[derive(Debug, Clone, Default)]
pub struct SmartGuideResult {
    pub point: Point,
    pub guides: Vec<SmartGuide>,
    pub snapped_x: bool,
    pub snapped_y: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SnapResult {
    pub point: Point,
    pub snapped_x: bool,
    pub snapped_y: bool,
}

impl SnapResult {
    pub fn none(point: Point) -> Self {
        Self {
            point,
            snapped_x: false,
            snapped_y: false,
        }
    }

    pub fn is_snapped(&self) -> bool {
        self.snapped_x || self.snapped_y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AngleSnapResult {
    pub point: Point,
    pub angle_degrees: f64,
    pub original_angle_degrees: f64,
    pub snapped: bool,
    pub distance: f64,
}

impl AngleSnapResult {
    pub fn none(point: Point, start: Point) -> Self {
        let dx = point.x - start.x;
        let dy = point.y - start.y;
        let angle = dy.atan2(dx).to_degrees();
        let angle_normalized = if angle < 0.0 { angle + 360.0 } else { angle };
        let distance = (dx * dx + dy * dy).sqrt();

        Self {
            point,
            angle_degrees: angle_normalized,
            original_angle_degrees: angle_normalized,
            snapped: false,
            distance,
        }
    }
}

use super::{ShapeId, ShapeStyle, ShapeTrait, StrokeStyle};
use kurbo::{Affine, BezPath, PathEl, Point, Rect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diamond {
    pub(crate) id: ShapeId,
    pub position: Point,
    pub width: f64,
    pub height: f64,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub stroke_style: StrokeStyle,
    #[serde(default)]
    pub corner_radius: f64,
    pub style: ShapeStyle,
}

impl Diamond {
    pub fn new(position: Point, width: f64, height: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            width,
            height,
            rotation: 0.0,
            stroke_style: StrokeStyle::default(),
            corner_radius: 0.0,
            style: ShapeStyle::default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstruct(
        id: ShapeId,
        position: Point,
        width: f64,
        height: f64,
        rotation: f64,
        stroke_style: StrokeStyle,
        corner_radius: f64,
        style: ShapeStyle,
    ) -> Self {
        Self {
            id,
            position,
            width,
            height,
            rotation,
            stroke_style,
            corner_radius,
            style,
        }
    }

    pub fn from_corners(p1: Point, p2: Point) -> Self {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        Self::new(
            Point::new(min_x, min_y),
            (p2.x - p1.x).abs(),
            (p2.y - p1.y).abs(),
        )
    }

    pub fn diamond_path(&self) -> BezPath {
        let cx = self.position.x + self.width / 2.0;
        let cy = self.position.y + self.height / 2.0;
        let top = Point::new(cx, self.position.y);
        let right = Point::new(self.position.x + self.width, cy);
        let bottom = Point::new(cx, self.position.y + self.height);
        let left = Point::new(self.position.x, cy);

        let mut path = BezPath::new();

        if self.corner_radius > 0.0 {
            let r = self.corner_radius.min(self.width / 4.0).min(self.height / 4.0);
            let hw = self.width / 2.0;
            let hh = self.height / 2.0;
            let diag = (hw * hw + hh * hh).sqrt();
            let t = if diag > 0.0 { (r / diag).min(0.5) } else { 0.0 };

            let lerp = |a: Point, b: Point, f: f64| Point::new(a.x + (b.x - a.x) * f, a.y + (b.y - a.y) * f);

            let t_r = lerp(top, right, t);
            let r_t = lerp(right, top, t);
            let r_b = lerp(right, bottom, t);
            let b_r = lerp(bottom, right, t);
            let b_l = lerp(bottom, left, t);
            let l_b = lerp(left, bottom, t);
            let l_t = lerp(left, top, t);
            let t_l = lerp(top, left, t);

            path.push(PathEl::MoveTo(t_r));
            path.push(PathEl::LineTo(r_t));
            path.push(PathEl::QuadTo(right, r_b));
            path.push(PathEl::LineTo(b_r));
            path.push(PathEl::QuadTo(bottom, b_l));
            path.push(PathEl::LineTo(l_b));
            path.push(PathEl::QuadTo(left, l_t));
            path.push(PathEl::LineTo(t_l));
            path.push(PathEl::QuadTo(top, t_r));
            path.push(PathEl::ClosePath);
        } else {
            path.push(PathEl::MoveTo(top));
            path.push(PathEl::LineTo(right));
            path.push(PathEl::LineTo(bottom));
            path.push(PathEl::LineTo(left));
            path.push(PathEl::ClosePath);
        }

        path
    }
}

impl ShapeTrait for Diamond {
    fn id(&self) -> ShapeId {
        self.id
    }

    fn bounds(&self) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            self.position.x + self.width,
            self.position.y + self.height,
        )
    }

    fn hit_test(&self, point: Point, tolerance: f64) -> bool {
        let cx = self.position.x + self.width / 2.0;
        let cy = self.position.y + self.height / 2.0;
        let half_w = self.width / 2.0 + tolerance;
        let half_h = self.height / 2.0 + tolerance;
        if half_w <= 0.0 || half_h <= 0.0 {
            return false;
        }
        let dx = (point.x - cx).abs();
        let dy = (point.y - cy).abs();
        dx / half_w + dy / half_h <= 1.0
    }

    fn to_path(&self) -> BezPath {
        self.diamond_path()
    }

    fn style(&self) -> &ShapeStyle {
        &self.style
    }

    fn style_mut(&mut self) -> &mut ShapeStyle {
        &mut self.style
    }

    fn transform(&mut self, affine: Affine) {
        let tl = affine * self.position;
        let br = affine * Point::new(self.position.x + self.width, self.position.y + self.height);
        self.position = Point::new(tl.x.min(br.x), tl.y.min(br.y));
        self.width = (br.x - tl.x).abs();
        self.height = (br.y - tl.y).abs();
    }

    fn clone_box(&self) -> Box<dyn ShapeTrait + Send + Sync> {
        Box::new(self.clone())
    }
}

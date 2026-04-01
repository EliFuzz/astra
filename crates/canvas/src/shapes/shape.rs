use super::{
    Arrow, Diamond, Ellipse, Freehand, Group, Image, Line, Math, Rectangle, Text,
};
use astra_core::{ShapeId, ShapeStyle, ShapeTrait};
use kurbo::{BezPath, Point, Rect};
use serde::{Deserialize, Serialize};

use super::arrow::BindSide;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    Rectangle(Rectangle),
    Diamond(Diamond),
    Ellipse(Ellipse),
    Line(Line),
    Arrow(Arrow),
    Freehand(Freehand),
    Text(Text),
    Group(Group),
    Image(Image),
    Math(Math),
}

impl Shape {
    pub fn id(&self) -> ShapeId {
        match self {
            Shape::Rectangle(s) => s.id(),
            Shape::Diamond(s) => s.id(),
            Shape::Ellipse(s) => s.id(),
            Shape::Line(s) => s.id(),
            Shape::Arrow(s) => s.id(),
            Shape::Freehand(s) => s.id(),
            Shape::Text(s) => s.id(),
            Shape::Group(s) => s.id(),
            Shape::Image(s) => s.id(),
            Shape::Math(s) => s.id(),
        }
    }

    pub fn bounds(&self) -> Rect {
        match self {
            Shape::Rectangle(s) => s.bounds(),
            Shape::Diamond(s) => s.bounds(),
            Shape::Ellipse(s) => s.bounds(),
            Shape::Line(s) => s.bounds(),
            Shape::Arrow(s) => s.bounds(),
            Shape::Freehand(s) => s.bounds(),
            Shape::Text(s) => s.bounds(),
            Shape::Group(s) => s.bounds(),
            Shape::Image(s) => s.bounds(),
            Shape::Math(s) => s.bounds(),
        }
    }

    pub fn snap_points(&self) -> Vec<Point> {
        match self {
            Shape::Line(s) => s.all_points(),
            Shape::Arrow(s) => s.all_points(),
            Shape::Group(_) | Shape::Freehand(_) => Vec::new(),
            _ => super::shape_geometry::rect_midpoints(self.bounds()).to_vec(),
        }
    }

    pub fn border_attachment_point(&self, side: BindSide, focus: f64) -> Point {
        let b = self.bounds();
        let c = b.center();
        let hw = b.width() / 2.0;
        let hh = b.height() / 2.0;
        match side {
            BindSide::Top => Point::new(c.x + focus * hw, b.y0),
            BindSide::Bottom => Point::new(c.x + focus * hw, b.y1),
            BindSide::Left => Point::new(b.x0, c.y + focus * hh),
            BindSide::Right => Point::new(b.x1, c.y + focus * hh),
        }
    }

    pub fn find_arrow_binding_snap(
        &self,
        point: Point,
        midpoint_radius: f64,
        border_radius: f64,
    ) -> Option<(Point, BindSide, f64)> {
        match self {
            Shape::Line(_) | Shape::Arrow(_) | Shape::Group(_) | Shape::Freehand(_) => return None,
            _ => {}
        }
        let b = self.bounds();
        if b.width() < 1.0 || b.height() < 1.0 {
            return None;
        }
        let mids = super::shape_geometry::rect_midpoints(b);
        let sides = [
            BindSide::Top,
            BindSide::Right,
            BindSide::Bottom,
            BindSide::Left,
        ];
        for (mp, side) in mids.iter().zip(sides.iter()) {
            let dx = point.x - mp.x;
            let dy = point.y - mp.y;
            if (dx * dx + dy * dy).sqrt() <= midpoint_radius {
                return Some((*mp, *side, 0.0));
            }
        }
        let closest = super::shape_geometry::closest_point_on_rect_border(point, b);
        let dx = point.x - closest.x;
        let dy = point.y - closest.y;
        if (dx * dx + dy * dy).sqrt() <= border_radius {
            let (side, focus) = super::shape_geometry::classify_border_point(closest, b);
            return Some((closest, side, focus));
        }
        None
    }

    pub fn intersects_rect(&self, rect: Rect) -> bool {
        match self {
            Shape::Line(s) => {
                super::shape_geometry::line_segments_intersect_rect(&s.all_points(), rect)
            }
            Shape::Arrow(s) => {
                super::shape_geometry::line_segments_intersect_rect(&s.all_points(), rect)
            }
            _ => {
                let bounds = self.bounds();
                rect.intersect(bounds.inflate(1.0, 1.0)).area() > 0.0
            }
        }
    }

    pub fn hit_test(&self, point: Point, tolerance: f64) -> bool {
        match self {
            Shape::Rectangle(s) => s.hit_test(point, tolerance),
            Shape::Diamond(s) => s.hit_test(point, tolerance),
            Shape::Ellipse(s) => s.hit_test(point, tolerance),
            Shape::Line(s) => s.hit_test(point, tolerance),
            Shape::Arrow(s) => s.hit_test(point, tolerance),
            Shape::Freehand(s) => s.hit_test(point, tolerance),
            Shape::Text(s) => s.hit_test(point, tolerance),
            Shape::Group(s) => s.hit_test(point, tolerance),
            Shape::Image(s) => s.hit_test(point, tolerance),
            Shape::Math(s) => s.hit_test(point, tolerance),
        }
    }

    pub fn to_path(&self) -> BezPath {
        match self {
            Shape::Rectangle(s) => s.to_path(),
            Shape::Diamond(s) => s.to_path(),
            Shape::Ellipse(s) => s.to_path(),
            Shape::Line(s) => s.to_path(),
            Shape::Arrow(s) => s.to_path(),
            Shape::Freehand(s) => s.to_path(),
            Shape::Text(s) => s.to_path(),
            Shape::Group(s) => s.to_path(),
            Shape::Image(s) => s.to_path(),
            Shape::Math(s) => s.to_path(),
        }
    }

    pub fn style(&self) -> &ShapeStyle {
        match self {
            Shape::Rectangle(s) => s.style(),
            Shape::Diamond(s) => s.style(),
            Shape::Ellipse(s) => s.style(),
            Shape::Line(s) => s.style(),
            Shape::Arrow(s) => s.style(),
            Shape::Freehand(s) => s.style(),
            Shape::Text(s) => s.style(),
            Shape::Group(s) => s.style(),
            Shape::Image(s) => s.style(),
            Shape::Math(s) => s.style(),
        }
    }

    pub fn style_mut(&mut self) -> &mut ShapeStyle {
        match self {
            Shape::Rectangle(s) => s.style_mut(),
            Shape::Diamond(s) => s.style_mut(),
            Shape::Ellipse(s) => s.style_mut(),
            Shape::Line(s) => s.style_mut(),
            Shape::Arrow(s) => s.style_mut(),
            Shape::Freehand(s) => s.style_mut(),
            Shape::Text(s) => s.style_mut(),
            Shape::Group(s) => s.style_mut(),
            Shape::Image(s) => s.style_mut(),
            Shape::Math(s) => s.style_mut(),
        }
    }
}

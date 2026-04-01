use super::shape::Shape;
use super::{Group, Image, ShapeTrait};
use kurbo::Affine;
use uuid::Uuid;

impl Shape {
    pub fn transform(&mut self, affine: Affine) {
        match self {
            Shape::Rectangle(s) => s.transform(affine),
            Shape::Diamond(s) => s.transform(affine),
            Shape::Ellipse(s) => s.transform(affine),
            Shape::Line(s) => s.transform(affine),
            Shape::Arrow(s) => s.transform(affine),
            Shape::Freehand(s) => s.transform(affine),
            Shape::Text(s) => s.transform(affine),
            Shape::Group(s) => s.transform(affine),
            Shape::Image(s) => s.transform(affine),
            Shape::Math(s) => s.transform(affine),
        }
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Shape::Group(_))
    }

    pub fn as_group(&self) -> Option<&Group> {
        match self {
            Shape::Group(g) => Some(g),
            _ => None,
        }
    }

    pub fn as_group_mut(&mut self) -> Option<&mut Group> {
        match self {
            Shape::Group(g) => Some(g),
            _ => None,
        }
    }

    pub fn regenerate_id(&mut self) {
        let new_id = Uuid::new_v4();
        match self {
            Shape::Rectangle(s) => s.id = new_id,
            Shape::Diamond(s) => s.id = new_id,
            Shape::Ellipse(s) => s.id = new_id,
            Shape::Line(s) => s.id = new_id,
            Shape::Arrow(s) => s.id = new_id,
            Shape::Freehand(s) => s.id = new_id,
            Shape::Text(s) => s.id = new_id,
            Shape::Group(s) => s.id = new_id,
            Shape::Image(s) => s.id = new_id,
            Shape::Math(s) => s.id = new_id,
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self, Shape::Image(_))
    }

    pub fn as_image(&self) -> Option<&Image> {
        match self {
            Shape::Image(img) => Some(img),
            _ => None,
        }
    }

    pub fn rotation(&self) -> f64 {
        match self {
            Shape::Rectangle(r) => r.rotation,
            Shape::Diamond(d) => d.rotation,
            Shape::Ellipse(e) => e.rotation,
            Shape::Text(t) => t.rotation,
            Shape::Image(i) => i.rotation,
            Shape::Math(m) => m.rotation,
            Shape::Group(g) => g.rotation,
            _ => 0.0,
        }
    }

    pub fn set_rotation(&mut self, rotation: f64) {
        match self {
            Shape::Rectangle(r) => r.rotation = rotation,
            Shape::Diamond(d) => d.rotation = rotation,
            Shape::Ellipse(e) => e.rotation = rotation,
            Shape::Text(t) => t.rotation = rotation,
            Shape::Image(i) => i.rotation = rotation,
            Shape::Math(m) => m.rotation = rotation,
            Shape::Group(g) => g.rotation = rotation,
            _ => {}
        }
    }

    pub fn supports_rotation(&self) -> bool {
        matches!(
            self,
            Shape::Rectangle(_)
                | Shape::Diamond(_)
                | Shape::Ellipse(_)
                | Shape::Text(_)
                | Shape::Image(_)
                | Shape::Math(_)
                | Shape::Group(_)
        )
    }
}

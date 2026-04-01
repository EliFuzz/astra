use super::handles::{Handle, HandleKind, HandleShape};
use crate::shapes::{Shape, ShapeTrait};
use kurbo::Point;

pub(super) fn get_shape_handles(shape: &Shape) -> Vec<Handle> {
    match shape {
        Shape::Rectangle(r) => {
            let bounds = r.bounds();
            vec![
                Handle::new(HandleKind::TopLeft, Point::new(bounds.x0, bounds.y0)),
                Handle::new(HandleKind::TopRight, Point::new(bounds.x1, bounds.y0)),
                Handle::new(HandleKind::BottomLeft, Point::new(bounds.x0, bounds.y1)),
                Handle::new(HandleKind::BottomRight, Point::new(bounds.x1, bounds.y1)),
            ]
        }
        Shape::Ellipse(e) => {
            let bounds = e.bounds();
            vec![
                Handle::new(HandleKind::TopLeft, Point::new(bounds.x0, bounds.y0)),
                Handle::new(HandleKind::TopRight, Point::new(bounds.x1, bounds.y0)),
                Handle::new(HandleKind::BottomLeft, Point::new(bounds.x0, bounds.y1)),
                Handle::new(HandleKind::BottomRight, Point::new(bounds.x1, bounds.y1)),
            ]
        }
        Shape::Line(l) => {
            vec![
                Handle::new(HandleKind::Start, l.start).with_shape(HandleShape::Circle),
                Handle::new(HandleKind::End, l.end).with_shape(HandleShape::Circle),
            ]
        }
        Shape::Arrow(a) => {
            vec![
                Handle::new(HandleKind::Start, a.start).with_shape(HandleShape::Circle),
                Handle::new(HandleKind::End, a.end).with_shape(HandleShape::Circle),
            ]
        }
        Shape::Freehand(f) => {
            let bounds = f.bounds();
            vec![
                Handle::new(HandleKind::TopLeft, Point::new(bounds.x0, bounds.y0)),
                Handle::new(HandleKind::TopRight, Point::new(bounds.x1, bounds.y0)),
                Handle::new(HandleKind::BottomLeft, Point::new(bounds.x0, bounds.y1)),
                Handle::new(HandleKind::BottomRight, Point::new(bounds.x1, bounds.y1)),
            ]
        }
        Shape::Text(t) => {
            let bounds = t.bounds();
            vec![
                Handle::new(HandleKind::TopLeft, Point::new(bounds.x0, bounds.y0)),
                Handle::new(HandleKind::TopRight, Point::new(bounds.x1, bounds.y0)),
                Handle::new(HandleKind::BottomLeft, Point::new(bounds.x0, bounds.y1)),
                Handle::new(HandleKind::BottomRight, Point::new(bounds.x1, bounds.y1)),
            ]
        }
        Shape::Group(g) => {
            let bounds = g.bounds();
            vec![
                Handle::new(HandleKind::TopLeft, Point::new(bounds.x0, bounds.y0)),
                Handle::new(HandleKind::TopRight, Point::new(bounds.x1, bounds.y0)),
                Handle::new(HandleKind::BottomLeft, Point::new(bounds.x0, bounds.y1)),
                Handle::new(HandleKind::BottomRight, Point::new(bounds.x1, bounds.y1)),
            ]
        }
        Shape::Image(img) => {
            let bounds = img.bounds();
            vec![
                Handle::new(HandleKind::TopLeft, Point::new(bounds.x0, bounds.y0)),
                Handle::new(HandleKind::TopRight, Point::new(bounds.x1, bounds.y0)),
                Handle::new(HandleKind::BottomLeft, Point::new(bounds.x0, bounds.y1)),
                Handle::new(HandleKind::BottomRight, Point::new(bounds.x1, bounds.y1)),
            ]
        }
        Shape::Diamond(d) => {
            let bounds = d.bounds();
            vec![
                Handle::new(HandleKind::TopLeft, Point::new(bounds.x0, bounds.y0)),
                Handle::new(HandleKind::TopRight, Point::new(bounds.x1, bounds.y0)),
                Handle::new(HandleKind::BottomLeft, Point::new(bounds.x0, bounds.y1)),
                Handle::new(HandleKind::BottomRight, Point::new(bounds.x1, bounds.y1)),
            ]
        }
        Shape::Math(m) => {
            let bounds = m.bounds();
            vec![
                Handle::new(HandleKind::TopLeft, Point::new(bounds.x0, bounds.y0)),
                Handle::new(HandleKind::TopRight, Point::new(bounds.x1, bounds.y0)),
                Handle::new(HandleKind::BottomLeft, Point::new(bounds.x0, bounds.y1)),
                Handle::new(HandleKind::BottomRight, Point::new(bounds.x1, bounds.y1)),
            ]
        }
    }
}

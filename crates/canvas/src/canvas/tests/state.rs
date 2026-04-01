use crate::canvas::{Canvas, CanvasDocument};
use crate::shapes::{Rectangle, Shape, ShapeTrait};
use kurbo::Point;

#[test]
fn canvas_selection_follows_select_and_clear() {
    let mut canvas = Canvas::new();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    let id = rect.id();

    canvas.document.add_shape(Shape::Rectangle(rect));

    assert!(!canvas.is_selected(id));
    canvas.select(id);
    assert!(canvas.is_selected(id));
    canvas.clear_selection();
    assert!(!canvas.is_selected(id));
}

#[test]
fn delete_selected_removes_shapes_and_clears_selection() {
    let mut canvas = Canvas::new();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    let id = rect.id();

    canvas.document.add_shape(Shape::Rectangle(rect));
    canvas.select(id);
    canvas.delete_selected();

    assert!(canvas.document.is_empty());
    assert!(canvas.selection.is_empty());
}

#[test]
fn replace_document_clears_selection() {
    let mut canvas = Canvas::new();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    let id = rect.id();
    canvas.document.add_shape(Shape::Rectangle(rect));
    canvas.select(id);

    let mut replacement = CanvasDocument::new();
    replacement.add_shape(Shape::Rectangle(Rectangle::new(
        Point::new(10.0, 10.0),
        50.0,
        50.0,
    )));

    canvas.replace_document(replacement);

    assert!(canvas.selection.is_empty());
    assert_eq!(canvas.document.len(), 1);
}

#[test]
fn insert_shapes_centered_at_selects_inserted_shapes() {
    let mut canvas = Canvas::new();
    let shapes = vec![
        Shape::Rectangle(Rectangle::new(Point::new(0.0, 0.0), 10.0, 10.0)),
        Shape::Rectangle(Rectangle::new(Point::new(20.0, 0.0), 10.0, 10.0)),
    ];

    let ids = canvas.insert_shapes_centered_at(shapes, Point::new(100.0, 200.0));

    assert_eq!(ids.len(), 2);
    assert_eq!(canvas.selection, ids);

    let bounds = canvas.document.bounds().unwrap();
    assert_eq!(bounds.center(), Point::new(100.0, 200.0));
}

#[test]
fn document_new_is_empty() {
    let doc = CanvasDocument::new();
    assert!(doc.is_empty());
}

#[test]
fn document_add_shape_increments_len_and_get_shape() {
    let mut doc = CanvasDocument::new();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));
    assert_eq!(doc.len(), 1);
    assert!(doc.get_shape(id).is_some());
}

#[test]
fn document_remove_shape_empties_document() {
    let mut doc = CanvasDocument::new();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));
    let removed = doc.remove_shape(id);
    assert!(removed.is_some());
    assert!(doc.is_empty());
}

#[test]
fn document_z_order_bring_to_front_and_send_to_back() {
    let mut doc = CanvasDocument::new();
    let rect1 = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    let rect2 = Rectangle::new(Point::new(50.0, 50.0), 100.0, 100.0);
    let id1 = rect1.id();
    let id2 = rect2.id();
    doc.add_shape(Shape::Rectangle(rect1));
    doc.add_shape(Shape::Rectangle(rect2));
    assert_eq!(doc.z_order, vec![id1, id2]);
    doc.bring_to_front(id1);
    assert_eq!(doc.z_order, vec![id2, id1]);
    doc.send_to_back(id1);
    assert_eq!(doc.z_order, vec![id1, id2]);
}

#[test]
fn document_shapes_at_point_hit_order() {
    let mut doc = CanvasDocument::new();
    let rect1 = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    let rect2 = Rectangle::new(Point::new(50.0, 50.0), 100.0, 100.0);
    let id1 = rect1.id();
    let id2 = rect2.id();
    doc.add_shape(Shape::Rectangle(rect1));
    doc.add_shape(Shape::Rectangle(rect2));
    let hits = doc.shapes_at_point(Point::new(75.0, 75.0), 0.0);
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0], id2);
    assert_eq!(hits[1], id1);
    let hits = doc.shapes_at_point(Point::new(25.0, 25.0), 0.0);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0], id1);
}

#[test]
fn document_undo_add_shape_and_redo() {
    let mut doc = CanvasDocument::new();
    doc.push_undo();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));
    assert_eq!(doc.len(), 1);
    assert!(doc.can_undo());
    assert!(doc.undo());
    assert!(doc.is_empty());
    assert!(doc.can_redo());
    assert!(doc.redo());
    assert_eq!(doc.len(), 1);
    assert!(doc.get_shape(id).is_some());
}

#[test]
fn document_undo_remove_shape_restores_shape() {
    let mut doc = CanvasDocument::new();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));
    doc.push_undo();
    doc.remove_shape(id);
    assert!(doc.is_empty());
    assert!(doc.undo());
    assert_eq!(doc.len(), 1);
    assert!(doc.get_shape(id).is_some());
}

#[test]
fn document_new_action_after_undo_clears_redo() {
    let mut doc = CanvasDocument::new();
    doc.push_undo();
    let rect1 = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    doc.add_shape(Shape::Rectangle(rect1));
    assert!(doc.undo());
    assert!(doc.can_redo());
    doc.push_undo();
    let rect2 = Rectangle::new(Point::new(50.0, 50.0), 100.0, 100.0);
    doc.add_shape(Shape::Rectangle(rect2));
    assert!(!doc.can_redo());
}

#[test]
fn document_undo_redo_no_ops_when_stack_empty() {
    let mut doc = CanvasDocument::new();
    assert!(!doc.can_undo());
    assert!(!doc.undo());
    assert!(!doc.can_redo());
    assert!(!doc.redo());
}

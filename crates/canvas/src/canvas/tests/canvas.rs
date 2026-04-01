use crate::{
    Camera,
    canvas::CanvasDocument,
    shapes::{Ellipse, Rectangle, SerializableColor, Shape, ShapeStyle, ShapeTrait},
    snap::{GRID_SIZE, snap_point, snap_to_grid},
};
use kurbo::{Point, Rect, Vec2};

fn filled_style() -> ShapeStyle {
    let mut style = ShapeStyle::default();
    style.fill_color = Some(SerializableColor::black());
    style
}

#[test]
fn document_add_shape_present_in_map_and_z_order() {
    let mut doc = CanvasDocument::new();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));
    assert!(doc.shapes.contains_key(&id));
    assert!(doc.z_order.contains(&id));
    assert_eq!(doc.len(), 1);
}

#[test]
fn document_remove_shape_absent_from_map_and_z_order() {
    let mut doc = CanvasDocument::new();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));
    let removed = doc.remove_shape(id);
    assert!(removed.is_some());
    assert!(!doc.shapes.contains_key(&id));
    assert!(!doc.z_order.contains(&id));
    assert_eq!(doc.len(), 0);
}

#[test]
fn document_undo_restores_previous_state() {
    let mut doc = CanvasDocument::new();

    doc.push_undo();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let rect_id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));

    doc.push_undo();
    let ellipse = Ellipse::new(Point::new(200.0, 200.0), 40.0, 30.0);
    let ellipse_id = ellipse.id();
    doc.add_shape(Shape::Ellipse(ellipse));

    assert_eq!(doc.len(), 2);
    assert!(doc.can_undo());

    let did_undo = doc.undo();
    assert!(did_undo);
    assert_eq!(doc.len(), 1);
    assert!(doc.shapes.contains_key(&rect_id));
    assert!(!doc.shapes.contains_key(&ellipse_id));
    assert!(doc.can_redo());
}

#[test]
fn document_redo_restores_undone_state() {
    let mut doc = CanvasDocument::new();

    doc.push_undo();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let rect_id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));

    doc.undo();
    assert!(!doc.shapes.contains_key(&rect_id));

    let did_redo = doc.redo();
    assert!(did_redo);
    assert!(doc.shapes.contains_key(&rect_id));
    assert!(!doc.can_redo());
}

#[test]
fn document_undo_on_empty_stack_returns_false() {
    let mut doc = CanvasDocument::new();
    assert!(!doc.can_undo());
    assert!(!doc.undo());
}

#[test]
fn document_shapes_at_point_returns_hit_shape() {
    let mut doc = CanvasDocument::new();
    let mut rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    rect.style = filled_style();
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));

    let hits = doc.shapes_at_point(Point::new(50.0, 50.0), 2.0);
    assert!(hits.contains(&id));

    let misses = doc.shapes_at_point(Point::new(200.0, 200.0), 2.0);
    assert!(!misses.contains(&id));
}

#[test]
fn document_shapes_in_rect_returns_overlapping_shapes() {
    let mut doc = CanvasDocument::new();
    let rect = Rectangle::new(Point::new(50.0, 50.0), 100.0, 100.0);
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));

    let selection_rect = Rect::new(0.0, 0.0, 110.0, 110.0);
    let found = doc.shapes_in_rect(selection_rect);
    assert!(found.contains(&id));

    let non_overlapping = Rect::new(300.0, 300.0, 400.0, 400.0);
    let not_found = doc.shapes_in_rect(non_overlapping);
    assert!(!not_found.contains(&id));
}

#[test]
fn document_bring_to_front_reorders_z_order() {
    let mut doc = CanvasDocument::new();
    let r1 = Rectangle::new(Point::new(0.0, 0.0), 50.0, 50.0);
    let r2 = Rectangle::new(Point::new(10.0, 10.0), 50.0, 50.0);
    let id1 = r1.id();
    let id2 = r2.id();
    doc.add_shape(Shape::Rectangle(r1));
    doc.add_shape(Shape::Rectangle(r2));

    assert_eq!(doc.z_order.last().copied(), Some(id2));
    doc.bring_to_front(id1);
    assert_eq!(doc.z_order.last().copied(), Some(id1));
    assert_eq!(doc.z_order.first().copied(), Some(id2));
}

#[test]
fn camera_screen_world_roundtrip() {
    let camera = Camera::default();
    let world_point = Point::new(100.0, 200.0);
    let screen = camera.world_to_screen(world_point);
    let back = camera.screen_to_world(screen);
    assert!((back.x - world_point.x).abs() < 1e-9);
    assert!((back.y - world_point.y).abs() < 1e-9);
}

#[test]
fn camera_pan_updates_offset() {
    let mut camera = Camera::default();
    let delta = Vec2::new(50.0, -30.0);
    camera.pan(delta);
    assert_eq!(camera.offset, delta);

    camera.pan(delta);
    assert_eq!(camera.offset, delta + delta);
}

#[test]
fn camera_zoom_at_preserves_pivot_world_point() {
    let mut camera = Camera::default();
    let pivot_screen = Point::new(400.0, 300.0);
    let pivot_world_before = camera.screen_to_world(pivot_screen);

    camera.zoom_at(pivot_screen, 2.0);

    let pivot_world_after = camera.screen_to_world(pivot_screen);
    assert!((pivot_world_after.x - pivot_world_before.x).abs() < 1e-9);
    assert!((pivot_world_after.y - pivot_world_before.y).abs() < 1e-9);
}

#[test]
fn snap_to_grid_rounds_to_nearest_grid_line() {
    let result = snap_to_grid(Point::new(27.0, 43.0), GRID_SIZE);
    assert_eq!(result.point.x, 20.0);
    assert_eq!(result.point.y, 40.0);
    assert!(result.snapped_x);
    assert!(result.snapped_y);

    let exact = snap_to_grid(Point::new(40.0, 60.0), GRID_SIZE);
    assert_eq!(exact.point.x, 40.0);
    assert_eq!(exact.point.y, 60.0);
}

#[test]
fn snap_point_disabled_returns_original_point() {
    let original = Point::new(13.7, 57.9);
    let result = snap_point(original, false, GRID_SIZE);
    assert_eq!(result.point.x, original.x);
    assert_eq!(result.point.y, original.y);
    assert!(!result.snapped_x);
    assert!(!result.snapped_y);
}

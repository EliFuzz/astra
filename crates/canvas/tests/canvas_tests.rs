use astra_canvas::{
    Camera, Canvas, CanvasDocument,
    shapes::{Rectangle, Shape, ShapeTrait},
};
use kurbo::{Point, Vec2};

#[test]
fn new_canvas_document_starts_empty() {
    let doc = CanvasDocument::new();
    assert!(doc.shapes.is_empty());
    assert!(doc.z_order.is_empty());
}

#[test]
fn new_canvas_document_has_unique_id() {
    let a = CanvasDocument::new();
    let b = CanvasDocument::new();
    assert_ne!(a.id, b.id);
}

#[test]
fn new_canvas_document_default_name_is_untitled() {
    assert_eq!(CanvasDocument::new().name, "Untitled");
}

#[test]
fn add_shape_increments_len_and_populates_z_order() {
    let mut doc = CanvasDocument::new();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));
    assert_eq!(doc.len(), 1);
    assert!(doc.z_order.contains(&id));
}

#[test]
fn get_shape_returns_inserted_shape() {
    let mut doc = CanvasDocument::new();
    let rect = Rectangle::new(Point::new(10.0, 20.0), 80.0, 40.0);
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));
    assert!(doc.get_shape(id).is_some());
}

#[test]
fn remove_shape_decrements_len_and_removes_from_z_order() {
    let mut doc = CanvasDocument::new();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));
    doc.remove_shape(id);
    assert_eq!(doc.len(), 0);
    assert!(!doc.z_order.contains(&id));
}

#[test]
fn undo_reverts_added_shape() {
    let mut doc = CanvasDocument::new();
    doc.push_undo();
    doc.add_shape(Shape::Rectangle(Rectangle::new(
        Point::new(0.0, 0.0),
        100.0,
        50.0,
    )));
    assert_eq!(doc.len(), 1);
    doc.undo();
    assert_eq!(doc.len(), 0);
}

#[test]
fn redo_reapplies_reverted_shape() {
    let mut doc = CanvasDocument::new();
    doc.push_undo();
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let id = rect.id();
    doc.add_shape(Shape::Rectangle(rect));
    doc.undo();
    doc.redo();
    assert!(doc.shapes.contains_key(&id));
}

#[test]
fn bring_to_front_moves_shape_to_last_z_position() {
    let mut doc = CanvasDocument::new();
    let r1 = Rectangle::new(Point::new(0.0, 0.0), 10.0, 10.0);
    let r2 = Rectangle::new(Point::new(10.0, 10.0), 10.0, 10.0);
    let id1 = r1.id();
    doc.add_shape(Shape::Rectangle(r1));
    doc.add_shape(Shape::Rectangle(r2));
    doc.bring_to_front(id1);
    assert_eq!(*doc.z_order.last().unwrap(), id1);
}

#[test]
fn canvas_new_has_empty_selection() {
    assert!(Canvas::new().selection.is_empty());
}

#[test]
fn canvas_document_json_roundtrip_preserves_shape_count() {
    let mut doc = CanvasDocument::new();
    doc.add_shape(Shape::Rectangle(Rectangle::new(
        Point::new(0.0, 0.0),
        50.0,
        50.0,
    )));
    let json = doc.to_json().unwrap();
    let restored = CanvasDocument::from_json(&json).unwrap();
    assert_eq!(restored.len(), 1);
}

#[test]
fn pinch_zoom_in_increases_camera_zoom() {
    let mut camera = Camera::new();
    let initial_zoom = camera.zoom;
    let center = Point::new(400.0, 300.0);
    let delta = 0.1;
    camera.zoom_at(center, 1.0 + delta);
    assert!(camera.zoom > initial_zoom);
}

#[test]
fn pinch_zoom_out_decreases_camera_zoom() {
    let mut camera = Camera::new();
    let initial_zoom = camera.zoom;
    let center = Point::new(400.0, 300.0);
    let delta = -0.1;
    camera.zoom_at(center, 1.0 + delta);
    assert!(camera.zoom < initial_zoom);
}

#[test]
fn pinch_zoom_preserves_world_point_under_cursor() {
    let mut camera = Camera::new();
    camera.offset = Vec2::new(100.0, 50.0);
    camera.zoom = 1.5;
    let screen_center = Point::new(400.0, 300.0);
    let world_before = camera.screen_to_world(screen_center);
    camera.zoom_at(screen_center, 1.1);
    let world_after = camera.screen_to_world(screen_center);
    assert!((world_after.x - world_before.x).abs() < 1e-9);
    assert!((world_after.y - world_before.y).abs() < 1e-9);
}

#[test]
fn pinch_zoom_soft_clamped_at_min() {
    let mut camera = Camera::new();
    camera.zoom_at(Point::new(0.0, 0.0), 1e-40);
    assert!(camera.zoom >= 1e-15 * 0.999);
}

#[test]
fn pinch_zoom_soft_clamped_at_max() {
    let mut camera = Camera::new();
    camera.zoom = 1e14;
    camera.zoom_at(Point::new(0.0, 0.0), 1e10);
    assert!(camera.zoom <= 1e15 * 1.001);
}

#[test]
fn pinch_zero_delta_does_not_change_zoom() {
    let mut camera = Camera::new();
    let initial_zoom = camera.zoom;
    let initial_offset = camera.offset;
    camera.zoom_at(Point::new(200.0, 200.0), 1.0);
    assert!((camera.zoom - initial_zoom).abs() < f64::EPSILON);
    assert_eq!(camera.offset, initial_offset);
}

#[test]
fn copy_selected_as_json_serializes_selected_shapes() {
    let mut canvas = Canvas::new();
    let first = canvas.insert_shape_and_select(Shape::Rectangle(Rectangle::new(
        Point::new(0.0, 0.0),
        40.0,
        20.0,
    )));
    let second = Rectangle::new(Point::new(100.0, 0.0), 40.0, 20.0);
    let second_id = second.id();
    canvas.document.add_shape(Shape::Rectangle(second));
    canvas.add_to_selection(second_id);

    let json = canvas.copy_selected_as_json().unwrap();

    assert!(json.contains(&first.to_string()));
    assert!(json.contains(&second_id.to_string()));
}

#[test]
fn paste_shapes_from_json_regenerates_ids() {
    let mut canvas = Canvas::new();
    let original = Rectangle::new(Point::new(10.0, 10.0), 50.0, 30.0);
    let original_id = original.id();
    let json = serde_json::to_string(&vec![Shape::Rectangle(original)]).unwrap();

    let ids = canvas
        .paste_shapes_from_json(&json, Vec2::new(20.0, 20.0))
        .unwrap();

    assert_eq!(ids.len(), 1);
    assert_ne!(ids[0], original_id);
    assert!(canvas.document.get_shape(ids[0]).is_some());
}

use crate::event_handler::{EventHandler, SelectionRect};

#[test]
fn event_handler_default_state() {
    let handler = EventHandler::new();
    assert!(handler.manipulation.is_none());
    assert!(handler.multi_move.is_none());
    assert!(handler.selection_rect.is_none());
    assert!(handler.editing_text.is_none());
    assert!(!handler.is_manipulating());
    assert!(!handler.is_selecting());
}

#[test]
fn selection_rect_to_rect_normalizes_coordinates() {
    use kurbo::Point;
    let sel_rect = SelectionRect {
        start: Point::new(100.0, 200.0),
        current: Point::new(50.0, 100.0),
    };
    let rect = sel_rect.to_rect();
    assert_eq!(rect.x0, 50.0);
    assert_eq!(rect.y0, 100.0);
    assert_eq!(rect.x1, 100.0);
    assert_eq!(rect.y1, 200.0);
}

#[test]
fn laser_trail_decays_with_time() {
    let mut handler = EventHandler::new();
    use kurbo::Point;
    handler.laser_trail.push_back((Point::ZERO, 0.1));
    handler.laser_trail.push_back((Point::ZERO, 2.0));
    handler.update_laser_trail(0.3);
    assert_eq!(handler.laser_trail.len(), 1);
    assert!(handler.laser_trail[0].1 < 2.0);
}

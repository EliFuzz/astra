use super::ToolManager;
use super::super::ToolKind;
use kurbo::Point;

#[test]
fn test_tool_selection() {
    let mut tm = ToolManager::new();
    assert_eq!(tm.current_tool, ToolKind::Select);

    tm.set_tool(ToolKind::Rectangle);
    assert_eq!(tm.current_tool, ToolKind::Rectangle);
}

#[test]
fn test_tool_interaction() {
    let mut tm = ToolManager::new();
    tm.set_tool(ToolKind::Rectangle);

    assert!(!tm.is_active());

    tm.begin(Point::new(0.0, 0.0));
    assert!(tm.is_active());

    tm.update(Point::new(50.0, 50.0));

    let preview = tm.preview_shape();
    assert!(preview.is_some());

    let shape = tm.end(Point::new(100.0, 100.0));
    assert!(shape.is_some());
    assert!(!tm.is_active());
}

#[test]
fn test_cancel_interaction() {
    let mut tm = ToolManager::new();
    tm.set_tool(ToolKind::Rectangle);

    tm.begin(Point::new(0.0, 0.0));
    assert!(tm.is_active());

    tm.cancel();
    assert!(!tm.is_active());
}

#[test]
fn test_select_tool_no_shape() {
    let mut tm = ToolManager::new();
    tm.set_tool(ToolKind::Select);

    tm.begin(Point::new(0.0, 0.0));
    let shape = tm.end(Point::new(100.0, 100.0));
    assert!(shape.is_none());
}

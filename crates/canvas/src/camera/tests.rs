use super::state::*;
use kurbo::{Point, Vec2};

#[test]
fn test_default_camera() {
    let camera = Camera::new();
    assert_eq!(camera.offset, Vec2::ZERO);
    assert!((camera.zoom - BASE_ZOOM).abs() < f64::EPSILON);
}

#[test]
fn test_screen_to_world_identity() {
    let camera = Camera::new();
    let screen = Point::new(100.0, 200.0);
    let world = camera.screen_to_world(screen);
    assert!((world.x - screen.x / BASE_ZOOM).abs() < f64::EPSILON);
    assert!((world.y - screen.y / BASE_ZOOM).abs() < f64::EPSILON);
}

#[test]
fn test_screen_to_world_with_offset() {
    let mut camera = Camera::new();
    camera.offset = Vec2::new(50.0, 100.0);
    let screen = Point::new(100.0, 200.0);
    let world = camera.screen_to_world(screen);
    assert!((world.x - (screen.x - 50.0) / BASE_ZOOM).abs() < f64::EPSILON);
    assert!((world.y - (screen.y - 100.0) / BASE_ZOOM).abs() < f64::EPSILON);
}

#[test]
fn test_screen_to_world_with_zoom() {
    let mut camera = Camera::new();
    camera.zoom = 2.0;
    let world = camera.screen_to_world(Point::new(100.0, 200.0));
    assert!((world.x - 50.0).abs() < f64::EPSILON);
    assert!((world.y - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_roundtrip_conversion() {
    let mut camera = Camera::new();
    camera.offset = Vec2::new(30.0, -20.0);
    camera.zoom = 1.5;
    let original = Point::new(123.0, 456.0);
    let world = camera.screen_to_world(original);
    let back = camera.world_to_screen(world);
    assert!((back.x - original.x).abs() < 1e-10);
    assert!((back.y - original.y).abs() < 1e-10);
}

#[test]
fn test_zoom_soft_clamp_extremes() {
    let mut camera = Camera::new();
    camera.zoom_at(Point::ZERO, 1e-40);
    assert!(camera.zoom >= ZOOM_SOFT_MIN);
    assert!(camera.zoom <= ZOOM_SOFT_MIN + 0.001);
    camera.zoom = BASE_ZOOM;
    camera.zoom_at(Point::ZERO, 1e40);
    assert!(camera.zoom >= ZOOM_SOFT_MAX - 0.1);
    assert!(camera.zoom <= ZOOM_SOFT_MAX);
}

#[test]
fn test_pan() {
    let mut camera = Camera::new();
    camera.pan(Vec2::new(10.0, 20.0));
    assert!((camera.offset.x - 10.0).abs() < f64::EPSILON);
    assert!((camera.offset.y - 20.0).abs() < f64::EPSILON);
}

#[test]
fn zoom_level_anchor_and_extreme_zoom_pivot() {
    let mut c = Camera::new();
    c.offset = Vec2::new(12.0, -34.0);
    let a = Point::new(200.0, 150.0);
    let w = c.screen_to_world(a);
    c.set_zoom_level(4.2, a);
    assert!((c.zoom_level() - 4.2).abs() < 1e-9);
    let w2 = c.screen_to_world(a);
    assert!((w2.x - w.x).abs() < 1e-9 && (w2.y - w.y).abs() < 1e-9);
    c.zoom = 1e12;
    c.offset = Vec2::new(400.0, 300.0);
    let p = Point::new(400.0, 300.0);
    let wp = c.screen_to_world(p);
    c.zoom_at(p, 1.25);
    let wp2 = c.screen_to_world(p);
    assert!((wp2.x - wp.x).abs() < 1e-3 && (wp2.y - wp.y).abs() < 1e-3);
}

#[test]
fn origin_rebased_matches_transform() {
    let c = Camera::new();
    assert_eq!(c.origin_rebased_transform(), c.transform());
    let mut c = Camera::new();
    c.zoom = 2.0;
    c.offset = Vec2::new(5e6, -3e6);
    let (t, r) = (c.transform(), c.origin_rebased_transform());
    for p in [Point::ORIGIN, Point::new(1e7, -2e6), Point::new(-500.0, 800.0)] {
        let (a, b) = (t * p, r * p);
        assert!((a.x - b.x).abs() < 1e-6 && (a.y - b.y).abs() < 1e-6);
    }
}

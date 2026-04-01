use super::apply_manipulation;
use crate::selection::{Corner, Handle, HandleKind, get_handles};
use crate::shapes::{Line, Rectangle, Shape, ShapeTrait};
use kurbo::Point;

#[test]
fn test_line_handles() {
    let line = Line::new(Point::new(0.0, 0.0), Point::new(100.0, 100.0));
    let handles = get_handles(&Shape::Line(line));

    assert_eq!(handles.len(), 3);
    assert!(matches!(handles[0].kind, HandleKind::Endpoint(0)));
    assert!(matches!(handles[1].kind, HandleKind::Endpoint(1)));
    assert!(matches!(handles[2].kind, HandleKind::SegmentMidpoint(0)));
}

#[test]
fn test_rectangle_handles() {
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let handles = get_handles(&Shape::Rectangle(rect));

    assert_eq!(handles.len(), 5);
    assert!(matches!(
        handles[0].kind,
        HandleKind::Corner(Corner::TopLeft)
    ));
    assert!(matches!(handles[4].kind, HandleKind::Rotate));
}

#[test]
fn test_handle_hit_test() {
    let handle = Handle::new(Point::new(50.0, 50.0), HandleKind::Endpoint(0));

    assert!(handle.hit_test(Point::new(50.0, 50.0), 10.0));
    assert!(handle.hit_test(Point::new(55.0, 55.0), 10.0));
    assert!(!handle.hit_test(Point::new(70.0, 70.0), 10.0));
}

#[test]
fn test_apply_endpoint_manipulation() {
    let line = Line::new(Point::new(0.0, 0.0), Point::new(100.0, 100.0));
    let shape = Shape::Line(line);

    let result = apply_manipulation(
        &shape,
        Some(HandleKind::Endpoint(1)),
        kurbo::Vec2::new(10.0, 20.0),
        false,
    );

    if let Shape::Line(line) = result {
        assert!((line.end.x - 110.0).abs() < f64::EPSILON);
        assert!((line.end.y - 120.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected Line shape");
    }
}

#[test]
fn test_apply_corner_manipulation() {
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
    let shape = Shape::Rectangle(rect);

    let result = apply_manipulation(
        &shape,
        Some(HandleKind::Corner(Corner::BottomRight)),
        kurbo::Vec2::new(50.0, 50.0),
        false,
    );

    if let Shape::Rectangle(rect) = result {
        assert!((rect.width - 150.0).abs() < f64::EPSILON);
        assert!((rect.height - 150.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected Rectangle shape");
    }
}

#[test]
fn test_freehand_resize() {
    use crate::shapes::Freehand;

    let freehand = Freehand::from_points(vec![
        Point::new(0.0, 0.0),
        Point::new(50.0, 0.0),
        Point::new(50.0, 50.0),
        Point::new(0.0, 50.0),
    ]);
    let shape = Shape::Freehand(freehand);

    let result = apply_manipulation(
        &shape,
        Some(HandleKind::Corner(Corner::BottomRight)),
        kurbo::Vec2::new(50.0, 50.0),
        false,
    );

    if let Shape::Freehand(freehand) = result {
        let bounds = freehand.bounds();
        assert!((bounds.width() - 100.0).abs() < 0.1);
        assert!((bounds.height() - 100.0).abs() < 0.1);
    } else {
        panic!("Expected Freehand shape");
    }
}

#[test]
fn test_image_resize() {
    use crate::shapes::{Image, ImageFormat, ShapeStyle};

    let image = Image {
        id: uuid::Uuid::new_v4(),
        position: Point::new(0.0, 0.0),
        width: 100.0,
        height: 100.0,
        source_width: 200,
        source_height: 200,
        format: ImageFormat::Png,
        data_base64: String::new(),
        rotation: 0.0,
        corner_radius: 0.0,
        style: ShapeStyle::default(),
    };
    let shape = Shape::Image(image);

    let result = apply_manipulation(
        &shape,
        Some(HandleKind::Corner(Corner::BottomRight)),
        kurbo::Vec2::new(50.0, 50.0),
        false,
    );

    if let Shape::Image(image) = result {
        assert!((image.width - 150.0).abs() < f64::EPSILON);
        assert!((image.height - 150.0).abs() < f64::EPSILON);
    } else {
        panic!("Expected Image shape");
    }
}

#[test]
fn test_aspect_ratio_resize() {
    let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let shape = Shape::Rectangle(rect);

    let result = apply_manipulation(
        &shape,
        Some(HandleKind::Corner(Corner::BottomRight)),
        kurbo::Vec2::new(100.0, 100.0),
        true,
    );

    if let Shape::Rectangle(rect) = result {
        let aspect = rect.width / rect.height;
        assert!((aspect - 2.0).abs() < 0.1);
    } else {
        panic!("Expected Rectangle shape");
    }
}

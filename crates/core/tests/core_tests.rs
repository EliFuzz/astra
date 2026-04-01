use astra_core::{
    Duration, FillPattern, Instant, SerializableColor, ShapeStyle, Sloppiness, StrokeStyle,
    generate_seed, parse_color, point_to_polyline_dist, point_to_segment_dist,
};
use kurbo::Point;

#[test]
fn serializable_color_new_roundtrips_components() {
    let c = SerializableColor::new(10, 20, 30, 255);
    assert_eq!((c.r, c.g, c.b, c.a), (10, 20, 30, 255));
}

#[test]
fn serializable_color_black_is_opaque_zero_rgb() {
    let b = SerializableColor::black();
    assert_eq!((b.r, b.g, b.b, b.a), (0, 0, 0, 255));
}

#[test]
fn serializable_color_white_is_max_rgb_full_alpha() {
    let w = SerializableColor::white();
    assert_eq!((w.r, w.g, w.b, w.a), (255, 255, 255, 255));
}

#[test]
fn serializable_color_transparent_has_zero_alpha() {
    assert_eq!(SerializableColor::transparent().a, 0);
}

#[test]
fn shape_style_default_stroke_is_black_opaque() {
    let style = ShapeStyle::default();
    assert_eq!(style.stroke_color, SerializableColor::black());
}

#[test]
fn shape_style_default_fill_is_none() {
    assert!(ShapeStyle::default().fill_color.is_none());
}

#[test]
fn shape_style_default_opacity_is_one() {
    assert!((ShapeStyle::default().opacity - 1.0).abs() < f64::EPSILON);
}

#[test]
fn shape_style_default_stroke_width_is_two() {
    assert!((ShapeStyle::default().stroke_width - 2.0).abs() < f64::EPSILON);
}

#[test]
fn sloppiness_next_cycles_through_all_variants() {
    assert_eq!(Sloppiness::Architect.next(), Sloppiness::Artist);
    assert_eq!(Sloppiness::Artist.next(), Sloppiness::Cartoonist);
    assert_eq!(Sloppiness::Cartoonist.next(), Sloppiness::Drunk);
    assert_eq!(Sloppiness::Drunk.next(), Sloppiness::Architect);
}

#[test]
fn fill_pattern_next_cycles() {
    assert_eq!(FillPattern::Solid.next(), FillPattern::Hachure);
    assert_eq!(FillPattern::ZigZagLine.next(), FillPattern::Solid);
}

#[test]
fn stroke_style_next_cycles() {
    assert_eq!(StrokeStyle::Solid.next(), StrokeStyle::Dashed);
    assert_eq!(StrokeStyle::Dashed.next(), StrokeStyle::Dotted);
    assert_eq!(StrokeStyle::Dotted.next(), StrokeStyle::Solid);
}

#[test]
fn generate_seed_produces_distinct_values() {
    let a = generate_seed();
    let b = generate_seed();
    assert_ne!(a, b);
}

#[test]
fn point_to_segment_dist_perpendicular_offset_is_exact() {
    let dist = point_to_segment_dist(
        Point::new(5.0, 3.0),
        Point::new(0.0, 0.0),
        Point::new(10.0, 0.0),
    );
    assert!((dist - 3.0).abs() < 1e-9);
}

#[test]
fn point_to_polyline_dist_returns_minimum_segment_distance() {
    let polyline = [
        Point::new(0.0, 0.0),
        Point::new(10.0, 0.0),
        Point::new(10.0, 10.0),
    ];
    let dist = point_to_polyline_dist(Point::new(5.0, 2.0), &polyline);
    assert!((dist - 2.0).abs() < 1e-9);
}

#[test]
fn parse_color_accepts_trimmed_six_digit_hex_with_hash() {
    let c = parse_color("  #aabbcc  ").expect("valid hex");
    let sc = SerializableColor::from(c);
    assert_eq!((sc.r, sc.g, sc.b, sc.a), (0xaa, 0xbb, 0xcc, 255));
}

#[test]
fn parse_color_rejects_invalid_inputs() {
    assert!(parse_color("").is_none());
    assert!(parse_color("#aabb").is_none());
    assert!(parse_color("#gg0000").is_none());
    assert!(parse_color("aabbcc").is_none());
}

#[test]
fn instant_now_returns_monotonic_timestamps() {
    let t1 = Instant::now();
    let t2 = Instant::now();
    assert!(t2 >= t1);
}

#[test]
fn duration_from_millis_roundtrips() {
    let d = Duration::from_millis(500);
    assert_eq!(d.as_millis(), 500);
}

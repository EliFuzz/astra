use astra_canvas::shapes::FillPattern;
use kurbo::{BezPath, PathEl, Point, Rect};
use roughr::core::{FillStyle, OptionsBuilder};

struct SimpleRng {
    state: u32,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self { state: seed.max(1) }
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u32() as f64 / u32::MAX as f64) * 2.0 - 1.0
    }

    fn offset(&mut self, amount: f64) -> f64 {
        self.next_f64() * amount
    }
}

pub(crate) fn apply_hand_drawn_effect(
    path: &BezPath,
    roughness: f64,
    zoom: f64,
    seed: u32,
    stroke_index: u32,
) -> BezPath {
    if roughness <= 0.0 {
        return path.clone();
    }

    let scale = 1.0 / zoom.sqrt();
    let max_randomness_offset = roughness * 2.0 * scale;
    let bowing = roughness * 1.0;
    let combined_seed = seed.wrapping_add(stroke_index.wrapping_mul(99991));
    let mut rng = SimpleRng::new(combined_seed);

    let mut result = BezPath::new();
    let mut last_point = Point::ZERO;

    for el in path.elements() {
        match el {
            PathEl::MoveTo(p) => {
                let wobbled = Point::new(
                    p.x + rng.offset(max_randomness_offset),
                    p.y + rng.offset(max_randomness_offset),
                );
                result.move_to(wobbled);
                last_point = *p;
            }
            PathEl::LineTo(p) => {
                let dx = p.x - last_point.x;
                let dy = p.y - last_point.y;
                let len = (dx * dx + dy * dy).sqrt();

                let bow_offset = bowing * roughness * len / 200.0;
                let bow = rng.offset(bow_offset) * scale;

                let (perp_x, perp_y) = if len > 0.001 {
                    (-dy / len, dx / len)
                } else {
                    (0.0, 0.0)
                };

                let mid_x = (last_point.x + p.x) / 2.0 + perp_x * bow;
                let mid_y = (last_point.y + p.y) / 2.0 + perp_y * bow;

                let end = Point::new(
                    p.x + rng.offset(max_randomness_offset),
                    p.y + rng.offset(max_randomness_offset),
                );

                result.quad_to(Point::new(mid_x, mid_y), end);
                last_point = *p;
            }
            PathEl::QuadTo(p1, p2) => {
                let wobbled_p1 = Point::new(
                    p1.x + rng.offset(max_randomness_offset * 0.7),
                    p1.y + rng.offset(max_randomness_offset * 0.7),
                );
                let wobbled_p2 = Point::new(
                    p2.x + rng.offset(max_randomness_offset),
                    p2.y + rng.offset(max_randomness_offset),
                );
                result.quad_to(wobbled_p1, wobbled_p2);
                last_point = *p2;
            }
            PathEl::CurveTo(p1, p2, p3) => {
                let wobbled_p1 = Point::new(
                    p1.x + rng.offset(max_randomness_offset * 0.5),
                    p1.y + rng.offset(max_randomness_offset * 0.5),
                );
                let wobbled_p2 = Point::new(
                    p2.x + rng.offset(max_randomness_offset * 0.5),
                    p2.y + rng.offset(max_randomness_offset * 0.5),
                );
                let wobbled_p3 = Point::new(
                    p3.x + rng.offset(max_randomness_offset),
                    p3.y + rng.offset(max_randomness_offset),
                );
                result.curve_to(wobbled_p1, wobbled_p2, wobbled_p3);
                last_point = *p3;
            }
            PathEl::ClosePath => {
                result.close_path();
            }
        }
    }

    result
}

pub(crate) fn generate_fill_pattern(
    pattern: FillPattern,
    bounds: Rect,
    stroke_width: f64,
    seed: u32,
) -> BezPath {
    use roughr::core::{OpSetType, OpType};

    let fill_style = match pattern {
        FillPattern::Solid => return BezPath::new(),
        FillPattern::Hachure => FillStyle::Hachure,
        FillPattern::ZigZag => FillStyle::ZigZag,
        FillPattern::CrossHatch => FillStyle::CrossHatch,
        FillPattern::Dots => FillStyle::Dots,
        FillPattern::Dashed => FillStyle::Dashed,
        FillPattern::ZigZagLine => FillStyle::ZigZagLine,
    };

    let fill_color: roughr::Srgba = roughr::Srgba::new(0.0, 0.0, 0.0, 1.0);
    let options = OptionsBuilder::default()
        .seed(seed as u64)
        .fill_style(fill_style)
        .fill(fill_color)
        .stroke(fill_color)
        .fill_weight((stroke_width * 0.5) as f32)
        .hachure_gap((stroke_width * 4.0) as f32)
        .build()
        .unwrap();

    let generator = roughr::generator::Generator::default();
    let drawing = generator.rectangle::<f64>(
        bounds.x0,
        bounds.y0,
        bounds.width(),
        bounds.height(),
        &Some(options),
    );

    let mut path = BezPath::new();
    for set in drawing.sets.iter() {
        if set.op_set_type == OpSetType::FillSketch {
            for op in set.ops.iter() {
                match op.op {
                    OpType::Move => {
                        path.move_to(Point::new(op.data[0], op.data[1]));
                    }
                    OpType::LineTo => {
                        path.line_to(Point::new(op.data[0], op.data[1]));
                    }
                    OpType::BCurveTo => {
                        path.curve_to(
                            Point::new(op.data[0], op.data[1]),
                            Point::new(op.data[2], op.data[3]),
                            Point::new(op.data[4], op.data[5]),
                        );
                    }
                }
            }
        }
    }
    path
}

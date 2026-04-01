use crate::shapes::BindSide;
use kurbo::{Point, Rect};

const OBSTACLE_MARGIN: f64 = 20.0;

pub fn compute_elbow_path(
    start: Point,
    exit_side: Option<BindSide>,
    end: Point,
    entry_side: Option<BindSide>,
) -> Vec<Point> {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    if dx.abs() < 1.0 && dy.abs() < 1.0 {
        return vec![];
    }
    let h_first = matches!(exit_side, Some(BindSide::Left | BindSide::Right))
        || (exit_side.is_none() && dx.abs() >= dy.abs());
    let same = matches!(
        (exit_side, entry_side),
        (
            Some(BindSide::Left | BindSide::Right),
            Some(BindSide::Left | BindSide::Right)
        ) | (
            Some(BindSide::Top | BindSide::Bottom),
            Some(BindSide::Top | BindSide::Bottom)
        )
    );
    let back = !same
        && match exit_side {
            Some(BindSide::Right) => dx < 0.0,
            Some(BindSide::Left) => dx > 0.0,
            Some(BindSide::Bottom) => dy < 0.0,
            Some(BindSide::Top) => dy > 0.0,
            None => false,
        };
    if same || back {
        if h_first {
            if dy.abs() < 1.0 {
                return vec![];
            }
            let mx = (start.x + end.x) / 2.0;
            return vec![Point::new(mx, start.y), Point::new(mx, end.y)];
        }
        if dx.abs() < 1.0 {
            return vec![];
        }
        let my = (start.y + end.y) / 2.0;
        return vec![Point::new(start.x, my), Point::new(end.x, my)];
    }
    if h_first {
        if dy.abs() < 1.0 {
            vec![]
        } else {
            vec![Point::new(end.x, start.y)]
        }
    } else {
        if dx.abs() < 1.0 {
            vec![]
        } else {
            vec![Point::new(start.x, end.y)]
        }
    }
}

pub fn compute_routed_path(
    start: Point,
    exit_side: Option<BindSide>,
    end: Point,
    entry_side: Option<BindSide>,
    obstacles: &[Rect],
) -> Vec<Point> {
    if obstacles.is_empty() {
        return compute_elbow_path(start, exit_side, end, entry_side);
    }
    let vs = exit_pt(start, exit_side);
    let ve = exit_pt(end, entry_side);
    let mx = (vs.x + ve.x) / 2.0;
    let my = (vs.y + ve.y) / 2.0;
    let bb = obstacles
        .iter()
        .fold(
            Rect::new(
                vs.x.min(ve.x),
                vs.y.min(ve.y),
                vs.x.max(ve.x),
                vs.y.max(ve.y),
            ),
            |a, r| a.union(*r),
        )
        .inflate(OBSTACLE_MARGIN, OBSTACLE_MARGIN);

    let c1 = [Point::new(ve.x, vs.y)];
    let c2 = [Point::new(vs.x, ve.y)];
    let c3 = [Point::new(mx, vs.y), Point::new(mx, ve.y)];
    let c4 = [Point::new(vs.x, my), Point::new(ve.x, my)];
    let c5 = [Point::new(vs.x, bb.y0), Point::new(ve.x, bb.y0)];
    let c6 = [Point::new(vs.x, bb.y1), Point::new(ve.x, bb.y1)];
    let c7 = [Point::new(bb.x0, vs.y), Point::new(bb.x0, ve.y)];
    let c8 = [Point::new(bb.x1, vs.y), Point::new(bb.x1, ve.y)];

    for inner in [c1.as_slice(), &c2, &c3, &c4, &c5, &c6, &c7, &c8] {
        if !route_blocked(vs, inner, ve, obstacles) {
            return make_route(vs, start, inner, ve, end);
        }
    }
    compute_elbow_path(start, exit_side, end, entry_side)
}

fn exit_pt(p: Point, side: Option<BindSide>) -> Point {
    match side {
        Some(BindSide::Right) => Point::new(p.x + OBSTACLE_MARGIN, p.y),
        Some(BindSide::Left) => Point::new(p.x - OBSTACLE_MARGIN, p.y),
        Some(BindSide::Bottom) => Point::new(p.x, p.y + OBSTACLE_MARGIN),
        Some(BindSide::Top) => Point::new(p.x, p.y - OBSTACLE_MARGIN),
        None => p,
    }
}

fn route_blocked(vs: Point, inner: &[Point], ve: Point, obs: &[Rect]) -> bool {
    let first = inner.first().copied().unwrap_or(ve);
    if seg_hits(vs, first, obs) {
        return true;
    }
    for w in inner.windows(2) {
        if seg_hits(w[0], w[1], obs) {
            return true;
        }
    }
    if let Some(&last) = inner.last() {
        seg_hits(last, ve, obs)
    } else {
        false
    }
}

fn seg_hits(a: Point, b: Point, obs: &[Rect]) -> bool {
    obs.iter().any(|r| {
        let (x0, x1, y0, y1) = (r.x0 - 1.0, r.x1 + 1.0, r.y0 - 1.0, r.y1 + 1.0);
        if (a.y - b.y).abs() < 0.5 {
            a.y > y0 && a.y < y1 && a.x.min(b.x) < x1 && a.x.max(b.x) > x0
        } else {
            a.x > x0 && a.x < x1 && a.y.min(b.y) < y1 && a.y.max(b.y) > y0
        }
    })
}

fn make_route(vs: Point, start: Point, inner: &[Point], ve: Point, end: Point) -> Vec<Point> {
    let near = |a: Point, b: Point| (a.x - b.x).abs() < 0.5 && (a.y - b.y).abs() < 0.5;
    let mut r = Vec::with_capacity(inner.len() + 2);
    if !near(vs, start) {
        r.push(vs);
    }
    for &p in inner {
        if r.last().is_none_or(|&l: &Point| !near(l, p)) {
            r.push(p);
        }
    }
    if !near(ve, end) && r.last().is_none_or(|&l: &Point| !near(l, ve)) {
        r.push(ve);
    }
    r
}

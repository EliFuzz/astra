use super::super::VelloRenderer;
use kurbo::{Affine, BezPath, Point, Rect, Stroke};
use peniko::{Color, Fill};

const MAX_LINE_COUNT: f64 = 150.0;
const MIN_LINE_COUNT: f64 = 5.0;
const SCALE_UP: [f64; 4] = [5.0, 10.0, 50.0, 100.0];
const SCALE_DOWN: [f64; 3] = [2.0, 5.0, 10.0];
const MAX_ADAPT_STEPS: usize = 128;
const MAX_DRAW_STEPS: usize = 152;
const CROSS_HALF: f64 = 3.0;
const DOT_HALF: f64 = 1.5;

fn world_bounds(viewport: Rect, transform: Affine) -> Option<(f64, f64, f64, f64)> {
    let inv = transform.inverse();
    let c = [
        inv * Point::new(viewport.x0, viewport.y0),
        inv * Point::new(viewport.x1, viewport.y0),
        inv * Point::new(viewport.x0, viewport.y1),
        inv * Point::new(viewport.x1, viewport.y1),
    ];
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for p in c {
        if !(p.x.is_finite() && p.y.is_finite()) {
            return None;
        }
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }
    Some((min_x, min_y, max_x, max_y))
}

fn line_count(ww: f64, wh: f64, gs: f64) -> Option<(f64, f64)> {
    let c = (ww / gs).ceil();
    let r = (wh / gs).ceil();
    (c.is_finite() && r.is_finite()).then_some((c, r))
}

fn adapt_grid_size(mut gs: f64, ww: f64, wh: f64) -> Option<f64> {
    if !gs.is_finite() || gs <= 0.0 {
        return None;
    }
    let ww = ww.abs();
    let wh = wh.abs();
    let mut k = 0usize;
    while k < MAX_ADAPT_STEPS {
        let (c, r) = line_count(ww, wh, gs)?;
        if c <= MAX_LINE_COUNT && r <= MAX_LINE_COUNT {
            break;
        }
        gs *= SCALE_UP[k % 4];
        k += 1;
        if !gs.is_finite() || gs <= 0.0 {
            return None;
        }
    }
    let (c, r) = line_count(ww, wh, gs)?;
    if c > MAX_LINE_COUNT || r > MAX_LINE_COUNT {
        return None;
    }
    k = 0;
    while k < MAX_ADAPT_STEPS {
        let (c, r) = line_count(ww, wh, gs)?;
        let m = c.max(r);
        if !m.is_finite() {
            return None;
        }
        if m >= MIN_LINE_COUNT || m == 0.0 {
            break;
        }
        gs /= SCALE_DOWN[k % 3];
        k += 1;
        if !gs.is_finite() || gs <= 0.0 {
            return None;
        }
    }
    k = 0;
    while k < MAX_ADAPT_STEPS {
        let (c, r) = line_count(ww, wh, gs)?;
        if c <= MAX_LINE_COUNT && r <= MAX_LINE_COUNT {
            return Some(gs);
        }
        gs *= SCALE_UP[k % 4];
        k += 1;
        if !gs.is_finite() || gs <= 0.0 {
            return None;
        }
    }
    None
}

fn density_cols_rows(sx: f64, sy: f64, ex: f64, ey: f64, gs: f64) -> (f64, f64) {
    if !gs.is_finite() || gs <= 0.0 {
        return (0.0, 0.0);
    }
    let cols = ((ex - sx).abs() / gs).floor() + 1.0;
    let rows = ((ey - sy).abs() / gs).floor() + 1.0;
    (cols.max(0.0), rows.max(0.0))
}

fn faded_alpha(cols: f64, rows: f64, base: u8) -> u8 {
    let density = cols.max(rows) / 150.0;
    let scale = if density > 0.7 {
        ((1.0 - density) / 0.3 * 100.0).clamp(0.0, 100.0) as u8
    } else {
        100
    };
    (base as f64 * f64::from(scale) / 100.0).round() as u8
}

fn axis_points(mut t: f64, end: f64, gs: f64, mut push: impl FnMut(f64)) {
    let mut n = 0usize;
    while t <= end + gs * 1e-9 && n < MAX_DRAW_STEPS {
        push(t);
        t += gs;
        n += 1;
    }
}

impl VelloRenderer {
    fn effective_grid(
        viewport: Rect,
        transform: Affine,
        base_grid_size: f64,
    ) -> Option<(f64, f64, f64, f64, f64)> {
        let (min_x, min_y, max_x, max_y) = world_bounds(viewport, transform)?;
        let world_w = max_x - min_x;
        let world_h = max_y - min_y;
        if !(world_w.is_finite() && world_h.is_finite()) {
            return None;
        }
        let gs = adapt_grid_size(base_grid_size, world_w, world_h)?;
        let sx = (min_x / gs).floor() * gs;
        let sy = (min_y / gs).floor() * gs;
        let ex = (max_x / gs).ceil() * gs;
        let ey = (max_y / gs).ceil() * gs;
        (sx.is_finite() && sy.is_finite() && ex.is_finite() && ey.is_finite())
            .then_some((sx, sy, ex, ey, gs))
    }

    pub(crate) fn render_grid_lines(
        &mut self,
        viewport: Rect,
        transform: Affine,
        grid_size: f64,
    ) {
        let Some((sx, sy, ex, ey, gs)) = Self::effective_grid(viewport, transform, grid_size) else {
            return;
        };
        let (c, r) = density_cols_rows(sx, sy, ex, ey, gs);
        let color = Color::from_rgba8(60, 80, 110, faded_alpha(c, r, 50));
        let stroke = Stroke::new(0.5);
        let mut path = BezPath::new();
        axis_points(sx, ex, gs, |x| {
            path.move_to(Point::new(x, sy));
            path.line_to(Point::new(x, ey));
        });
        axis_points(sy, ey, gs, |y| {
            path.move_to(Point::new(sx, y));
            path.line_to(Point::new(ex, y));
        });
        self.scene.stroke(&stroke, transform, color, None, &path);
    }

    pub(super) fn render_horizontal_lines(
        &mut self,
        viewport: Rect,
        transform: Affine,
        grid_size: f64,
    ) {
        let Some((sx, sy, _, ey, gs)) = Self::effective_grid(viewport, transform, grid_size) else {
            return;
        };
        let a = transform.as_coeffs()[0];
        if !a.is_finite() || a.abs() < 1e-300 {
            return;
        }
        let ex = sx + viewport.width() / a;
        let (c, r) = density_cols_rows(sx, sy, ex, ey, gs);
        let color = Color::from_rgba8(60, 80, 110, faded_alpha(c, r, 50));
        let stroke = Stroke::new(0.5);
        let mut path = BezPath::new();
        axis_points(sy, ey, gs, |y| {
            path.move_to(Point::new(sx, y));
            path.line_to(Point::new(ex, y));
        });
        self.scene.stroke(&stroke, transform, color, None, &path);
    }

    pub(super) fn render_grid_crosses(
        &mut self,
        viewport: Rect,
        transform: Affine,
        grid_size: f64,
    ) {
        let Some((sx, sy, ex, ey, gs)) = Self::effective_grid(viewport, transform, grid_size) else {
            return;
        };
        let (c, r) = density_cols_rows(sx, sy, ex, ey, gs);
        let color = Color::from_rgba8(60, 80, 110, faded_alpha(c, r, 35));
        let stroke = Stroke::new(1.0);
        let mut path = BezPath::new();
        let mut nx = 0usize;
        let mut x = sx;
        while x <= ex + gs * 1e-9 && nx < MAX_DRAW_STEPS {
            axis_points(sy, ey, gs, |y| {
                path.move_to(Point::new(x - CROSS_HALF, y));
                path.line_to(Point::new(x + CROSS_HALF, y));
                path.move_to(Point::new(x, y - CROSS_HALF));
                path.line_to(Point::new(x, y + CROSS_HALF));
            });
            x += gs;
            nx += 1;
        }
        self.scene.stroke(&stroke, transform, color, None, &path);
    }

    pub(super) fn render_grid_dots(
        &mut self,
        viewport: Rect,
        transform: Affine,
        grid_size: f64,
    ) {
        let Some((sx, sy, ex, ey, gs)) = Self::effective_grid(viewport, transform, grid_size) else {
            return;
        };
        let (c, r) = density_cols_rows(sx, sy, ex, ey, gs);
        let color = Color::from_rgba8(160, 160, 160, faded_alpha(c, r, 70));
        let mut path = BezPath::new();
        let mut nx = 0usize;
        let mut x = sx;
        while x <= ex + gs * 1e-9 && nx < MAX_DRAW_STEPS {
            axis_points(sy, ey, gs, |y| {
                let q = Rect::new(x - DOT_HALF, y - DOT_HALF, x + DOT_HALF, y + DOT_HALF);
                path.move_to(Point::new(q.x0, q.y0));
                path.line_to(Point::new(q.x1, q.y0));
                path.line_to(Point::new(q.x1, q.y1));
                path.line_to(Point::new(q.x0, q.y1));
                path.close_path();
            });
            x += gs;
            nx += 1;
        }
        self.scene.fill(Fill::NonZero, transform, color, None, &path);
    }
}

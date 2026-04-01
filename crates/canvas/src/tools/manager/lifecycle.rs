use crate::platform::Instant;
use super::create::generate_tool_seed;
use super::super::{ToolKind, ToolState};
use super::tool_manager::ToolManager;
use crate::shapes::Shape;
use kurbo::Point;

impl ToolManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_tool(&mut self, tool: ToolKind) {
        self.current_tool = tool;
        self.state = ToolState::Idle;
    }

    pub fn begin(&mut self, point: Point) {
        if self.current_tool == ToolKind::Freehand || self.current_tool == ToolKind::Highlighter {
            self.freehand_points.clear();
            self.freehand_pressures.clear();
            self.freehand_points.push(point);
            self.freehand_pressures.push(1.0);
            self.last_point_time = Some(Instant::now());
            self.last_point_pos = Some(point);
            self.smoothed_pressure = 1.0;
            self.msd_pos = point;
            self.msd_vel = Point::ZERO;
        }

        self.state = ToolState::Active {
            start: point,
            current: point,
            preview: Box::new(None),
            seed: generate_tool_seed(),
        };
    }

    pub fn update(&mut self, point: Point) {
        if let ToolState::Active { current, .. } = &mut self.state {
            *current = point;

            if self.current_tool == ToolKind::Freehand || self.current_tool == ToolKind::Highlighter {
                if self.calligraphy_mode {
                    const M: f64 = 0.5;
                    const K: f64 = 120.0;
                    const C: f64 = 15.49;
                    const DT: f64 = 1.0 / 60.0;

                    let dx = point.x - self.msd_pos.x;
                    let dy = point.y - self.msd_pos.y;
                    let ax = (K * dx - C * self.msd_vel.x) / M;
                    let ay = (K * dy - C * self.msd_vel.y) / M;

                    self.msd_vel.x += ax * DT;
                    self.msd_vel.y += ay * DT;
                    self.msd_pos.x += self.msd_vel.x * DT;
                    self.msd_pos.y += self.msd_vel.y * DT;

                    if let Some(last) = self.freehand_points.last() {
                        let dist = ((self.msd_pos.x - last.x).powi(2)
                            + (self.msd_pos.y - last.y).powi(2))
                        .sqrt();
                        if dist > 2.0 {
                            let pressure = self.compute_pressure(self.msd_pos);
                            self.freehand_points.push(self.msd_pos);
                            self.freehand_pressures.push(pressure);
                        }
                    }
                } else {
                    let pressure = self.compute_pressure(point);
                    self.freehand_points.push(point);
                    self.freehand_pressures.push(pressure);
                }
            }
        }
    }

    fn compute_pressure(&mut self, point: Point) -> f64 {
        if !self.pressure_simulation {
            return 1.0;
        }

        let now = Instant::now();
        let raw_pressure = if let (Some(last_time), Some(last_pos)) =
            (self.last_point_time, self.last_point_pos)
        {
            let dt = now.duration_since(last_time).as_secs_f64();
            if dt > 0.001 {
                let dist = ((point.x - last_pos.x).powi(2) + (point.y - last_pos.y).powi(2)).sqrt();
                let velocity = dist / dt;
                let normalized_velocity = (velocity / 800.0).min(3.0);
                let pressure = (-normalized_velocity).exp();
                pressure.clamp(0.2, 1.0)
            } else {
                self.smoothed_pressure
            }
        } else {
            1.0
        };

        const SMOOTHING: f64 = 0.3;
        self.smoothed_pressure =
            self.smoothed_pressure * (1.0 - SMOOTHING) + raw_pressure * SMOOTHING;

        self.last_point_time = Some(now);
        self.last_point_pos = Some(point);
        self.smoothed_pressure
    }

    pub fn end(&mut self, point: Point) -> Option<Shape> {
        if let ToolState::Active { start, seed, .. } = &self.state {
            let start = *start;
            let seed = *seed;
            let shape = self.create_shape_with_seed(start, point, seed);
            self.state = ToolState::Idle;
            self.freehand_points.clear();
            shape
        } else {
            None
        }
    }

    pub fn cancel(&mut self) {
        self.state = ToolState::Idle;
        self.freehand_points.clear();
        self.freehand_pressures.clear();
        self.last_point_time = None;
        self.last_point_pos = None;
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, ToolState::Active { .. })
    }

    pub fn freehand_points(&self) -> &[Point] {
        &self.freehand_points
    }

    pub fn freehand_pressures(&self) -> &[f64] {
        &self.freehand_pressures
    }
}

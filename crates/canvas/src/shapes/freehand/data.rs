use crate::shapes::{ShapeId, ShapeStyle};
use kurbo::Point;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Freehand {
    pub(crate) id: ShapeId,
    pub points: Vec<Point>,
    #[serde(default)]
    pub pressures: Vec<f64>,
    #[serde(default)]
    pub closed: bool,
    pub style: ShapeStyle,
}

impl Freehand {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            points: Vec::new(),
            pressures: Vec::new(),
            closed: false,
            style: ShapeStyle::default(),
        }
    }

    pub fn from_points(points: Vec<Point>) -> Self {
        Self {
            id: Uuid::new_v4(),
            points,
            pressures: Vec::new(),
            closed: false,
            style: ShapeStyle::default(),
        }
    }

    pub fn from_points_with_pressure(points: Vec<Point>, pressures: Vec<f64>) -> Self {
        Self {
            id: Uuid::new_v4(),
            points,
            pressures,
            closed: false,
            style: ShapeStyle::default(),
        }
    }

    pub fn reconstruct(
        id: ShapeId,
        points: Vec<Point>,
        pressures: Vec<f64>,
        style: ShapeStyle,
    ) -> Self {
        Self {
            id,
            points,
            pressures,
            closed: false,
            style,
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn add_point_with_pressure(&mut self, point: Point, pressure: f64) {
        self.points.push(point);
        self.pressures.push(pressure.clamp(0.0, 1.0));
    }

    pub fn pressure_at(&self, index: usize) -> f64 {
        self.pressures.get(index).copied().unwrap_or(1.0)
    }

    pub fn has_pressure(&self) -> bool {
        !self.pressures.is_empty()
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn simplify(&mut self, tolerance: f64) {
        if self.points.len() < 3 {
            return;
        }

        let (new_points, new_pressures) = super::simplify::rdp_simplify_with_pressure(
            &self.points,
            &self.pressures,
            tolerance,
        );
        self.points = new_points;
        self.pressures = new_pressures;
    }
}

impl Default for Freehand {
    fn default() -> Self {
        Self::new()
    }
}

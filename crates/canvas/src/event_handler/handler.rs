use std::collections::VecDeque;

use crate::canvas::Canvas;
use crate::selection::{ManipulationState, MultiMoveState};
use crate::shapes::ShapeId;
use crate::snap::{AngleSnapResult, SmartGuide, SnapResult};
use kurbo::Point;

use super::state::{RotationState, SelectionRect};

pub const LASER_TRAIL_CAP: usize = 30;

pub struct EventHandler {
    pub manipulation: Option<ManipulationState>,
    pub multi_move: Option<MultiMoveState>,
    pub selection_rect: Option<SelectionRect>,
    pub editing_text: Option<ShapeId>,
    pub text_edit_anchor: Option<Point>,
    pub text_edit_size: Option<(f64, f64)>,
    pub text_edit_parent_shape: Option<ShapeId>,
    pub last_snap: Option<SnapResult>,
    pub last_angle_snap: Option<AngleSnapResult>,
    pub line_start_point: Option<Point>,
    pub smart_guides: Vec<SmartGuide>,
    pub rotation_state: Option<RotationState>,
    pub eraser_points: Vec<Point>,
    pub eraser_radius: f64,
    pub laser_position: Option<Point>,
    pub laser_trail: VecDeque<(Point, f64)>,
    pub pending_math_edit: Option<ShapeId>,
    pub pending_image_insert: Option<Point>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            manipulation: None,
            multi_move: None,
            selection_rect: None,
            editing_text: None,
            text_edit_anchor: None,
            text_edit_size: None,
            text_edit_parent_shape: None,
            last_snap: None,
            last_angle_snap: None,
            line_start_point: None,
            smart_guides: Vec::new(),
            rotation_state: None,
            eraser_points: Vec::new(),
            eraser_radius: 10.0,
            laser_position: None,
            laser_trail: VecDeque::new(),
            pending_math_edit: None,
            pending_image_insert: None,
        }
    }

    pub fn is_manipulating(&self) -> bool {
        self.manipulation.is_some() || self.multi_move.is_some()
    }

    pub fn is_selecting(&self) -> bool {
        self.selection_rect.is_some()
    }

    pub fn selection_rect(&self) -> Option<&SelectionRect> {
        self.selection_rect.as_ref()
    }

    pub fn manipulation(&self) -> Option<&ManipulationState> {
        self.manipulation.as_ref()
    }

    pub fn clear_snap(&mut self) {
        self.last_snap = None;
        self.last_angle_snap = None;
        self.line_start_point = None;
        self.smart_guides.clear();
    }

    pub(super) fn apply_eraser(&mut self, canvas: &mut Canvas) {
        if self.eraser_points.is_empty() {
            return;
        }

        let radius = self.eraser_radius;
        let mut shapes_to_remove: Vec<ShapeId> = Vec::new();

        for shape in canvas.document.shapes_ordered() {
            for &point in &self.eraser_points {
                if shape.hit_test(point, radius) {
                    shapes_to_remove.push(shape.id());
                    break;
                }
            }
        }

        if !shapes_to_remove.is_empty() {
            canvas.document.push_undo();
            for id in shapes_to_remove {
                canvas.document.remove_shape(id);
            }
        }
    }

    pub fn update_laser_trail(&mut self, dt: f64) {
        for (_, alpha) in self.laser_trail.iter_mut() {
            *alpha -= dt * 4.0;
        }
        while self
            .laser_trail
            .front()
            .is_some_and(|(_, alpha)| *alpha <= 0.0)
        {
            self.laser_trail.pop_front();
        }
    }

    pub fn eraser_path(&self) -> &[Point] {
        &self.eraser_points
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

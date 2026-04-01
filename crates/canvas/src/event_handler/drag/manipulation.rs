use super::super::snap_helpers::{
    collect_routing_obstacles, find_shape_binding_snap, get_line_other_endpoint, make_arrow_binding,
};
use super::super::{EventHandler, RotationState};
use super::snap_corner::snap_corner_or_default;
use super::snap_line_endpoint::{SnapLineEndpointParams, snap_line_endpoint};
use crate::canvas::Canvas;
use crate::elbow::compute_routed_path;
use crate::input::InputState;
use crate::selection::{
    HandleKind, apply_manipulation, apply_rotation, get_manipulation_target_position,
};
use crate::shapes::Shape;
use crate::snap::SnapResult;
use kurbo::Point;

impl EventHandler {
    pub(super) fn drag_manipulation(
        &mut self,
        canvas: &mut Canvas,
        world_point: Point,
        input: &InputState,
        grid_snap_enabled: bool,
        smart_snap_enabled: bool,
        angle_snap_enabled: bool,
    ) {
        let Some(manip) = self.manipulation.as_mut() else {
            return;
        };

        if matches!(manip.handle, Some(HandleKind::Rotate)) {
            let snap_to_15deg = input.shift();
            let center = manip.original_shape.bounds().center();
            let shape_id = manip.shape_id;

            if let Some(mut shape) = canvas.document.get_shape_mut(shape_id) {
                let angle = apply_rotation(&mut shape, world_point, snap_to_15deg);
                self.rotation_state = Some(RotationState {
                    center,
                    angle,
                    snapped: snap_to_15deg,
                });
            }

            self.manipulation.as_mut().unwrap().current_point = world_point;
            return;
        }

        let is_line_or_arrow = matches!(manip.original_shape, Shape::Line(_) | Shape::Arrow(_));
        let raw_delta = kurbo::Vec2::new(
            world_point.x - manip.start_point.x,
            world_point.y - manip.start_point.y,
        );
        let original_position =
            get_manipulation_target_position(&manip.original_shape, manip.handle);
        let target_position = Point::new(
            original_position.x + raw_delta.x,
            original_position.y + raw_delta.y,
        );
        let shape_id = manip.shape_id;
        let handle = manip.handle;
        let original_shape = manip.original_shape.clone();

        let snap_result = if is_line_or_arrow && handle.is_some() {
            let other_endpoint = get_line_other_endpoint(&original_shape, handle);

            let is_arrow = matches!(original_shape, Shape::Arrow(_));
            let is_endpoint = matches!(handle, Some(HandleKind::Endpoint(_)));

            if is_arrow && is_endpoint {
                let binding_hit = find_shape_binding_snap(canvas, target_position, &[shape_id]);
                if let Some((target_id, bind_pt, bind_side, bind_focus)) = binding_hit {
                    let new_binding = Some(make_arrow_binding(target_id, bind_side, bind_focus));
                    let manip = self.manipulation.as_mut().unwrap();
                    match handle {
                        Some(HandleKind::Endpoint(0)) => {
                            manip.pending_start_binding = Some(new_binding);
                        }
                        _ => {
                            manip.pending_end_binding = Some(new_binding);
                        }
                    }
                    SnapResult::none(bind_pt)
                } else {
                    let manip = self.manipulation.as_mut().unwrap();
                    match handle {
                        Some(HandleKind::Endpoint(0)) => {
                            manip.pending_start_binding = Some(None);
                        }
                        _ => {
                            manip.pending_end_binding = Some(None);
                        }
                    }
                    snap_line_endpoint(&mut SnapLineEndpointParams {
                        last_angle_snap: &mut self.last_angle_snap,
                        line_start_point: &mut self.line_start_point,
                        smart_guides: &mut self.smart_guides,
                        canvas,
                        shape_id,
                        original_shape: &original_shape,
                        handle,
                        target_position,
                        other_endpoint,
                        grid_snap_enabled,
                        smart_snap_enabled,
                        angle_snap_enabled,
                    })
                }
            } else {
                snap_line_endpoint(&mut SnapLineEndpointParams {
                    last_angle_snap: &mut self.last_angle_snap,
                    line_start_point: &mut self.line_start_point,
                    smart_guides: &mut self.smart_guides,
                    canvas,
                    shape_id,
                    original_shape: &original_shape,
                    handle,
                    target_position,
                    other_endpoint,
                    grid_snap_enabled,
                    smart_snap_enabled,
                    angle_snap_enabled,
                })
            }
        } else {
            snap_corner_or_default(
                &mut self.smart_guides,
                canvas,
                shape_id,
                handle,
                target_position,
                grid_snap_enabled,
                smart_snap_enabled,
            )
        };

        if snap_result.is_snapped() && self.last_angle_snap.is_none() {
            self.last_snap = Some(snap_result);
        }

        let adjusted_delta = kurbo::Vec2::new(
            snap_result.point.x - original_position.x,
            snap_result.point.y - original_position.y,
        );

        let manip = self.manipulation.as_mut().unwrap();
        manip.current_point = Point::new(
            manip.start_point.x + adjusted_delta.x,
            manip.start_point.y + adjusted_delta.y,
        );

        let mut new_shape =
            apply_manipulation(&original_shape, handle, adjusted_delta, input.shift());

        let is_arrow_endpoint = matches!(&new_shape, Shape::Arrow(_))
            && matches!(handle, Some(HandleKind::Endpoint(_)));

        if is_arrow_endpoint {
            let manip_ref = self.manipulation.as_ref().unwrap();
            let pending_start = manip_ref.pending_start_binding.clone();
            let pending_end = manip_ref.pending_end_binding.clone();

            if let Shape::Arrow(arrow) = &mut new_shape {
                if let Some(sb) = pending_start {
                    arrow.start_binding = sb;
                }
                if let Some(eb) = pending_end {
                    arrow.end_binding = eb;
                }
                if arrow.start_binding.is_some() || arrow.end_binding.is_some() {
                    let exit_side = arrow.start_binding.as_ref().map(|b| b.side);
                    let entry_side = arrow.end_binding.as_ref().map(|b| b.side);
                    let mut exclude = vec![shape_id];
                    if let Some(b) = &arrow.start_binding {
                        exclude.push(b.target_id);
                    }
                    if let Some(b) = &arrow.end_binding {
                        exclude.push(b.target_id);
                    }
                    let obstacles = collect_routing_obstacles(canvas, &exclude);
                    arrow.intermediate_points = compute_routed_path(
                        arrow.start,
                        exit_side,
                        arrow.end,
                        entry_side,
                        &obstacles,
                    );
                } else {
                    arrow.intermediate_points.clear();
                }
            }
        }

        if let Some(mut shape) = canvas.document.get_shape_mut(shape_id) {
            *shape = new_shape;
        }
    }
}

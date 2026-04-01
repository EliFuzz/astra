use super::super::EventHandler;
use super::super::snap_helpers::{collect_routing_obstacles, collect_snap_candidates};
use crate::canvas::Canvas;
use crate::elbow::compute_routed_path;
use crate::shapes::{Shape, ShapeId};
use crate::snap::{
    GRID_SIZE, MULTI_MOVE_SNAP_RADIUS, SMART_GUIDE_THRESHOLD, detect_smart_guides, snap_to_grid,
};
use kurbo::{Point, Rect};

type ArrowUpdate = (
    ShapeId,
    Point,
    Option<crate::shapes::BindSide>,
    Point,
    Option<crate::shapes::BindSide>,
);

impl EventHandler {
    pub(super) fn drag_multi_move(
        &mut self,
        canvas: &mut Canvas,
        world_point: Point,
        grid_snap_enabled: bool,
        smart_snap_enabled: bool,
    ) {
        let Some(mm) = &mut self.multi_move else {
            return;
        };

        let raw_delta = kurbo::Vec2::new(
            world_point.x - mm.start_point.x,
            world_point.y - mm.start_point.y,
        );

        let movement_threshold = 2.0;
        let has_meaningful_movement = raw_delta.hypot() > movement_threshold;

        let reference_shape = mm.original_shapes.values().next();
        let snap_result = if let Some(ref_shape) = reference_shape {
            let original_bounds = ref_shape.bounds();
            let target_bounds = Rect::new(
                original_bounds.x0 + raw_delta.x,
                original_bounds.y0 + raw_delta.y,
                original_bounds.x1 + raw_delta.x,
                original_bounds.y1 + raw_delta.y,
            );

            let exclude_ids: Vec<ShapeId> = mm.original_shapes.keys().copied().collect();
            let mut final_delta = raw_delta;

            if has_meaningful_movement {
                if smart_snap_enabled {
                    let snap_zone =
                        target_bounds.inflate(MULTI_MOVE_SNAP_RADIUS, MULTI_MOVE_SNAP_RADIUS);
                    let other_bounds =
                        collect_snap_candidates(canvas, &exclude_ids, Some(snap_zone));

                    let guide_result =
                        detect_smart_guides(target_bounds, &other_bounds, SMART_GUIDE_THRESHOLD);
                    if guide_result.snapped_x || guide_result.snapped_y {
                        final_delta.x += guide_result.point.x - target_bounds.x0;
                        final_delta.y += guide_result.point.y - target_bounds.y0;
                        self.smart_guides = guide_result.guides;
                    }
                }

                if grid_snap_enabled {
                    let target_pos = Point::new(
                        original_bounds.x0 + final_delta.x,
                        original_bounds.y0 + final_delta.y,
                    );
                    let snap_result = snap_to_grid(target_pos, GRID_SIZE);
                    final_delta.x = snap_result.point.x - original_bounds.x0;
                    final_delta.y = snap_result.point.y - original_bounds.y0;
                    self.last_snap = Some(snap_result);
                }
            }

            final_delta
        } else {
            raw_delta
        };

        let mm = self.multi_move.as_mut().unwrap();
        mm.current_point = Point::new(
            mm.start_point.x + snap_result.x,
            mm.start_point.y + snap_result.y,
        );

        let translation = kurbo::Affine::translate(snap_result);
        if mm.is_duplicate {
            for (idx, &dup_id) in mm.duplicated_ids.iter().enumerate() {
                let original_shape = mm.original_shapes.values().nth(idx);
                if let (Some(orig), Some(mut shape)) =
                    (original_shape, canvas.document.get_shape_mut(dup_id))
                {
                    let mut new_shape = orig.clone();
                    new_shape.transform(translation);
                    *shape = new_shape;
                }
            }
        } else {
            let updates: Vec<(ShapeId, Shape)> = mm
                .original_shapes
                .iter()
                .map(|(&id, orig)| {
                    let mut new_shape = orig.clone();
                    new_shape.transform(translation);
                    (id, new_shape)
                })
                .collect();
            for (shape_id, new_shape) in updates {
                if let Some(mut shape) = canvas.document.get_shape_mut(shape_id) {
                    *shape = new_shape;
                }
            }

            update_bound_arrows(canvas, mm.original_shapes.keys().copied().collect());
        }
    }
}

fn update_bound_arrows(canvas: &mut Canvas, moved_ids: Vec<ShapeId>) {
    let mut arrow_updates: Vec<ArrowUpdate> = Vec::new();

    for &moved_id in &moved_ids {
        let bound_ids = canvas.document.arrows_bound_to(moved_id);
        for arrow_id in bound_ids {
            if let Some(Shape::Arrow(arrow)) = canvas.document.get_shape(arrow_id) {
                let start = arrow.start;
                let end = arrow.end;
                let start_binding = arrow.start_binding.clone();
                let end_binding = arrow.end_binding.clone();

                let new_start = if let Some(b) = &start_binding {
                    if moved_ids.contains(&b.target_id) {
                        if let Some(target) = canvas.document.get_shape(b.target_id) {
                            target.border_attachment_point(b.side, b.focus)
                        } else {
                            start
                        }
                    } else {
                        start
                    }
                } else {
                    start
                };

                let new_end = if let Some(b) = &end_binding {
                    if moved_ids.contains(&b.target_id) {
                        if let Some(target) = canvas.document.get_shape(b.target_id) {
                            target.border_attachment_point(b.side, b.focus)
                        } else {
                            end
                        }
                    } else {
                        end
                    }
                } else {
                    end
                };

                let exit_side = start_binding.as_ref().map(|b| b.side);
                let entry_side = end_binding.as_ref().map(|b| b.side);

                if (new_start.x - start.x).abs() > 0.5
                    || (new_start.y - start.y).abs() > 0.5
                    || (new_end.x - end.x).abs() > 0.5
                    || (new_end.y - end.y).abs() > 0.5
                {
                    arrow_updates.push((arrow_id, new_start, exit_side, new_end, entry_side));
                }
            }
        }
    }

    for (arrow_id, new_start, exit_side, new_end, entry_side) in arrow_updates {
        let mut exclude = vec![arrow_id];
        exclude.extend(moved_ids.iter().copied());

        let obstacles = collect_routing_obstacles(canvas, &exclude);

        if let Some(mut guard) = canvas.document.get_shape_mut(arrow_id) {
            if let Shape::Arrow(arrow) = &mut *guard {
                arrow.start = new_start;
                arrow.end = new_end;
                arrow.intermediate_points =
                    compute_routed_path(new_start, exit_side, new_end, entry_side, &obstacles);
            }
        }
    }
}

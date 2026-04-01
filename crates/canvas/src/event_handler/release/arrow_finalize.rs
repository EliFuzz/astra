use crate::canvas::Canvas;
use crate::elbow::compute_routed_path;
use crate::shapes::{Arrow, ShapeId};

use super::super::snap_helpers::collect_routing_obstacles;

pub(super) fn finalize_arrow_route(arrow: &mut Arrow, arrow_id: ShapeId, canvas: &Canvas) {
    if arrow.start_binding.is_some() || arrow.end_binding.is_some() {
        let exit_side = arrow.start_binding.as_ref().map(|b| b.side);
        let entry_side = arrow.end_binding.as_ref().map(|b| b.side);
        let mut exclude = vec![arrow_id];
        if let Some(b) = &arrow.start_binding {
            exclude.push(b.target_id);
        }
        if let Some(b) = &arrow.end_binding {
            exclude.push(b.target_id);
        }
        let obstacles = collect_routing_obstacles(canvas, &exclude);
        arrow.intermediate_points =
            compute_routed_path(arrow.start, exit_side, arrow.end, entry_side, &obstacles);
    } else {
        arrow.intermediate_points.clear();
    }
}

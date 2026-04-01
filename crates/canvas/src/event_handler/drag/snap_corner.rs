use super::super::snap_helpers::collect_snap_candidates;
use crate::canvas::Canvas;
use crate::selection::HandleKind;
use crate::shapes::ShapeId;
use crate::snap::{
    ENDPOINT_SNAP_RADIUS, GRID_SIZE, SMART_GUIDE_THRESHOLD, SmartGuide, SnapResult,
    detect_smart_guides_for_point, snap_to_grid,
};
use kurbo::{Point, Rect, Size};

pub(crate) fn snap_corner_or_default(
    smart_guides: &mut Vec<SmartGuide>,
    canvas: &Canvas,
    shape_id: ShapeId,
    handle: Option<HandleKind>,
    target_position: Point,
    grid_snap_enabled: bool,
    smart_snap_enabled: bool,
) -> SnapResult {
    let is_corner_resize = matches!(handle, Some(HandleKind::Corner(_)));

    if is_corner_resize && smart_snap_enabled {
        let snap_zone = Rect::from_center_size(
            target_position,
            Size::new(ENDPOINT_SNAP_RADIUS * 2.0, ENDPOINT_SNAP_RADIUS * 2.0),
        );
        let other_bounds = collect_snap_candidates(canvas, &[shape_id], Some(snap_zone));

        let guide_result =
            detect_smart_guides_for_point(target_position, &other_bounds, SMART_GUIDE_THRESHOLD);

        let mut result_point = guide_result.point;

        if grid_snap_enabled {
            let grid_result = snap_to_grid(result_point, GRID_SIZE);
            if !guide_result.snapped_x {
                result_point.x = grid_result.point.x;
            }
            if !guide_result.snapped_y {
                result_point.y = grid_result.point.y;
            }
        }

        if guide_result.snapped_x || guide_result.snapped_y {
            *smart_guides = guide_result.guides;
        }

        SnapResult {
            point: result_point,
            snapped_x: guide_result.snapped_x || grid_snap_enabled,
            snapped_y: guide_result.snapped_y || grid_snap_enabled,
        }
    } else if grid_snap_enabled {
        snap_to_grid(target_position, GRID_SIZE)
    } else {
        SnapResult::none(target_position)
    }
}

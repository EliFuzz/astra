use super::super::snap_helpers::{collect_snap_candidates, self_snap_rects};
use crate::canvas::Canvas;
use crate::selection::HandleKind;
use crate::shapes::{Shape, ShapeId};
use crate::snap::{
    AngleSnapResult, ENDPOINT_SNAP_RADIUS, GRID_SIZE, SMART_GUIDE_THRESHOLD, SmartGuide,
    SnapResult, detect_smart_guides_for_point, snap_line_endpoint_isometric,
    snap_ray_to_smart_guides, snap_to_grid,
};
use kurbo::{Point, Rect, Size};

pub(crate) struct SnapLineEndpointParams<'a> {
    pub last_angle_snap: &'a mut Option<AngleSnapResult>,
    pub line_start_point: &'a mut Option<Point>,
    pub smart_guides: &'a mut Vec<SmartGuide>,
    pub canvas: &'a Canvas,
    pub shape_id: ShapeId,
    pub original_shape: &'a Shape,
    pub handle: Option<HandleKind>,
    pub target_position: Point,
    pub other_endpoint: Point,
    pub grid_snap_enabled: bool,
    pub smart_snap_enabled: bool,
    pub angle_snap_enabled: bool,
}

pub(crate) fn snap_line_endpoint(params: &mut SnapLineEndpointParams<'_>) -> SnapResult {
    if params.angle_snap_enabled {
        let angle_result = snap_line_endpoint_isometric(
            params.other_endpoint,
            params.target_position,
            true,
            false,
            false,
            GRID_SIZE,
        );

        let mut final_point = angle_result.point;

        if params.smart_snap_enabled {
            let snap_zone = Rect::from_center_size(
                angle_result.point,
                Size::new(ENDPOINT_SNAP_RADIUS * 2.0, ENDPOINT_SNAP_RADIUS * 2.0),
            );
            let mut other_bounds =
                collect_snap_candidates(params.canvas, &[params.shape_id], Some(snap_zone));
            other_bounds.extend(self_snap_rects(params.original_shape, params.handle));

            let guide_result = snap_ray_to_smart_guides(
                params.other_endpoint,
                angle_result.angle_degrees,
                angle_result.point,
                &other_bounds,
                SMART_GUIDE_THRESHOLD,
            );

            if guide_result.snapped_x || guide_result.snapped_y {
                final_point = guide_result.point;
                *params.smart_guides = guide_result.guides;
            }
        }

        if params.grid_snap_enabled && !params.smart_snap_enabled {
            let grid_result = snap_line_endpoint_isometric(
                params.other_endpoint,
                params.target_position,
                true,
                true,
                false,
                GRID_SIZE,
            );
            final_point = grid_result.point;
        }

        if angle_result.snapped {
            *params.last_angle_snap = Some(AngleSnapResult {
                point: final_point,
                ..angle_result
            });
            *params.line_start_point = Some(params.other_endpoint);
        }

        SnapResult {
            point: final_point,
            snapped_x: angle_result.snapped,
            snapped_y: angle_result.snapped,
        }
    } else if params.smart_snap_enabled {
        let snap_zone = Rect::from_center_size(
            params.target_position,
            Size::new(ENDPOINT_SNAP_RADIUS * 2.0, ENDPOINT_SNAP_RADIUS * 2.0),
        );
        let mut other_bounds =
            collect_snap_candidates(params.canvas, &[params.shape_id], Some(snap_zone));
        other_bounds.extend(self_snap_rects(params.original_shape, params.handle));

        let guide_result = detect_smart_guides_for_point(
            params.target_position,
            &other_bounds,
            SMART_GUIDE_THRESHOLD,
        );

        let mut result_point = guide_result.point;

        if params.grid_snap_enabled {
            let grid_result = snap_to_grid(result_point, GRID_SIZE);
            result_point = grid_result.point;
        }

        if guide_result.snapped_x || guide_result.snapped_y {
            *params.smart_guides = guide_result.guides;
        }

        SnapResult {
            point: result_point,
            snapped_x: guide_result.snapped_x || params.grid_snap_enabled,
            snapped_y: guide_result.snapped_y || params.grid_snap_enabled,
        }
    } else if params.grid_snap_enabled {
        snap_to_grid(params.target_position, GRID_SIZE)
    } else {
        SnapResult::none(params.target_position)
    }
}

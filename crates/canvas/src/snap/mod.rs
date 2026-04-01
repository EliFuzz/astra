mod constants;

pub mod angle;
pub mod grid;
pub mod smart_guides;
pub mod types;

#[cfg(test)]
mod tests;

pub use angle::{
    ANGLE_SNAP_INCREMENT, snap_angle, snap_line_endpoint, snap_line_endpoint_isometric,
};
pub use constants::{
    ARROW_BIND_BORDER_RADIUS, ARROW_BIND_MIDPOINT_RADIUS, ENDPOINT_SNAP_RADIUS,
    EQUAL_SPACING_SNAP_RADIUS, GRID_SIZE, MULTI_MOVE_SNAP_RADIUS, SMART_GUIDE_THRESHOLD,
};
pub use grid::{snap_point, snap_ray_to_grid_lines, snap_to_grid};
pub use smart_guides::{
    detect_smart_guides, detect_smart_guides_for_point, snap_ray_to_smart_guides,
};
pub use types::{AngleSnapResult, SmartGuide, SmartGuideKind, SmartGuideResult, SnapResult};

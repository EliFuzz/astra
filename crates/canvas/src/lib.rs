mod platform;
pub mod spatial_index;
pub mod camera;
pub mod canvas;
pub mod elbow;
pub mod event_handler;
pub mod input;
pub mod selection;
pub mod shapes;
pub mod snap;
pub mod tools;
pub mod interaction;

pub use spatial_index::UniformGrid;
pub use camera::Camera;
pub use canvas::{AlignMode, Canvas, CanvasDocument, ShapeMut};
pub use event_handler::{EventHandler, RotationState, SelectionRect};
pub use input::InputState;
pub use selection::{ManipulationState, MultiMoveState};
pub use shapes::{ArrowBinding, BindSide};
pub use snap::{
    ARROW_BIND_BORDER_RADIUS, ARROW_BIND_MIDPOINT_RADIUS, ENDPOINT_SNAP_RADIUS,
    EQUAL_SPACING_SNAP_RADIUS, GRID_SIZE, MULTI_MOVE_SNAP_RADIUS, SMART_GUIDE_THRESHOLD,
    SmartGuide, SmartGuideKind, SmartGuideResult, SnapResult, detect_smart_guides,
    detect_smart_guides_for_point, snap_point, snap_ray_to_smart_guides, snap_to_grid,
};
pub use interaction::{EditingKind, Handle, HandleKind, HandleShape, WidgetManager, WidgetState};

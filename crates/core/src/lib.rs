pub mod color;
pub mod geometry;
pub mod platform;
pub mod png;
pub mod preferences;
pub mod shape;
pub mod style;

pub use color::{SerializableColor, parse_color};
pub use geometry::{point_to_polyline_dist, point_to_segment_dist};
pub use platform::{Duration, Instant};
pub use preferences::PreferredGridStyle;
pub use shape::{ShapeId, ShapeTrait};
pub use style::{
    FillPattern, ShapeStyle, Sloppiness, StrokeStyle, generate_seed,
};

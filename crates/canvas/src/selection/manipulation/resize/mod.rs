mod bounds;
mod complex;
mod shapes;

pub(crate) use complex::{apply_corner_resize_freehand, apply_corner_resize_group};
pub(crate) use shapes::{
    apply_corner_resize_diamond, apply_corner_resize_ellipse, apply_corner_resize_image,
    apply_corner_resize_math, apply_corner_resize_rect, apply_corner_resize_text,
};
pub(crate) use bounds::resize_bounds;

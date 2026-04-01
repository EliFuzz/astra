use crate::shapes::{Shape, ShapeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(super) const MAX_UNDO_HISTORY: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DocumentSnapshot {
    pub(super) shapes: HashMap<ShapeId, Shape>,
    pub(super) z_order: Vec<ShapeId>,
}

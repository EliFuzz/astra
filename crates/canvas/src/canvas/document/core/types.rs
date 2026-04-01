use crate::shapes::{Shape, ShapeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::snapshot::DocumentSnapshot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasDocument {
    pub id: String,
    pub name: String,
    pub shapes: HashMap<ShapeId, Shape>,
    pub z_order: Vec<ShapeId>,
    #[serde(skip)]
    pub spatial_index: crate::spatial_index::UniformGrid,
    #[serde(skip)]
    pub(in crate::canvas::document::core) undo_stack: Vec<DocumentSnapshot>,
    #[serde(skip)]
    pub(in crate::canvas::document::core) redo_stack: Vec<DocumentSnapshot>,
}

impl Default for CanvasDocument {
    fn default() -> Self {
        Self::new()
    }
}

impl CanvasDocument {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "Untitled".to_string(),
            shapes: HashMap::new(),
            z_order: Vec::new(),
            spatial_index: crate::spatial_index::UniformGrid::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub(super) fn snapshot(&self) -> DocumentSnapshot {
        DocumentSnapshot {
            shapes: self.shapes.clone(),
            z_order: self.z_order.clone(),
        }
    }

    pub fn clone_for_export(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            shapes: self.shapes.clone(),
            z_order: self.z_order.clone(),
            spatial_index: self.spatial_index.clone(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }
}

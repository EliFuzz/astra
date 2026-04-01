use std::ops::{Deref, DerefMut};

use crate::shapes::{Shape, ShapeId};

use super::types::CanvasDocument;

pub struct ShapeMut<'a> {
    doc: &'a mut CanvasDocument,
    id: ShapeId,
}

impl<'a> ShapeMut<'a> {
    pub(crate) fn new(doc: &'a mut CanvasDocument, id: ShapeId) -> Option<Self> {
        if !doc.shapes.contains_key(&id) {
            return None;
        }
        Some(Self { doc, id })
    }
}

impl Deref for ShapeMut<'_> {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        self.doc.shapes.get(&self.id).expect("shape exists")
    }
}

impl DerefMut for ShapeMut<'_> {
    fn deref_mut(&mut self) -> &mut Shape {
        self.doc.shapes.get_mut(&self.id).expect("shape exists")
    }
}

impl Drop for ShapeMut<'_> {
    fn drop(&mut self) {
        if let Some(shape) = self.doc.shapes.get(&self.id) {
            let b = shape.bounds();
            self.doc.spatial_index.sync(self.id, b);
        }
    }
}

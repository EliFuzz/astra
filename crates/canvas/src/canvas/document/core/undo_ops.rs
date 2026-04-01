use super::snapshot::MAX_UNDO_HISTORY;
use super::types::CanvasDocument;

impl CanvasDocument {
    pub fn push_undo(&mut self) {
        let snapshot = self.snapshot();
        self.undo_stack.push(snapshot);
        self.redo_stack.clear();
        if self.undo_stack.len() > MAX_UNDO_HISTORY {
            self.undo_stack.remove(0);
        }
    }

    pub fn undo(&mut self) -> bool {
        let Some(snapshot) = self.undo_stack.pop() else {
            return false;
        };
        let current = self.snapshot();
        self.redo_stack.push(current);
        self.shapes = snapshot.shapes;
        self.z_order = snapshot.z_order;
        self.spatial_index.rebuild(&self.shapes);
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(snapshot) = self.redo_stack.pop() else {
            return false;
        };
        let current = self.snapshot();
        self.undo_stack.push(current);
        self.shapes = snapshot.shapes;
        self.z_order = snapshot.z_order;
        self.spatial_index.rebuild(&self.shapes);
        true
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}

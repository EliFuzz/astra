use super::handles::Handle;
use super::shape_handles;
use super::state::{EditingKind, WidgetState};
use crate::shapes::{Shape, ShapeId};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct WidgetManager {
    states: HashMap<ShapeId, WidgetState>,
    selected: HashSet<ShapeId>,
    focused: Option<ShapeId>,
    hovered: Option<ShapeId>,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            selected: HashSet::new(),
            focused: None,
            hovered: None,
        }
    }

    pub fn state(&self, id: ShapeId) -> WidgetState {
        self.states.get(&id).cloned().unwrap_or_default()
    }

    pub fn set_state(&mut self, id: ShapeId, state: WidgetState) {
        if state.is_selected() {
            self.selected.insert(id);
        } else {
            self.selected.remove(&id);
        }

        if state.is_editing() {
            self.focused = Some(id);
        } else if self.focused == Some(id) {
            self.focused = None;
        }

        self.states.insert(id, state);
    }

    pub fn is_selected(&self, id: ShapeId) -> bool {
        self.selected.contains(&id)
    }

    pub fn selected(&self) -> &HashSet<ShapeId> {
        &self.selected
    }

    pub fn focused(&self) -> Option<ShapeId> {
        self.focused
    }

    pub fn hovered(&self) -> Option<ShapeId> {
        self.hovered
    }

    pub fn set_hovered(&mut self, id: Option<ShapeId>) {
        if let Some(old_id) = self.hovered {
            if Some(old_id) != id {
                if let Some(state) = self.states.get(&old_id) {
                    if *state == WidgetState::Hovered {
                        self.states.insert(old_id, WidgetState::Normal);
                    }
                }
            }
        }

        if let Some(new_id) = id {
            let current = self.state(new_id);
            if current == WidgetState::Normal {
                self.states.insert(new_id, WidgetState::Hovered);
            }
        }

        self.hovered = id;
    }

    pub fn select(&mut self, id: ShapeId) {
        self.clear_selection();
        self.add_to_selection(id);
    }

    pub fn add_to_selection(&mut self, id: ShapeId) {
        self.set_state(id, WidgetState::Selected);
    }

    pub fn deselect(&mut self, id: ShapeId) {
        if self.selected.contains(&id) {
            self.set_state(id, WidgetState::Normal);
        }
    }

    pub fn clear_selection(&mut self) {
        let selected: Vec<_> = self.selected.iter().copied().collect();
        for id in selected {
            self.set_state(id, WidgetState::Normal);
        }
        self.selected.clear();
    }

    pub fn enter_editing(&mut self, id: ShapeId, kind: EditingKind) {
        if let Some(old_id) = self.focused {
            if old_id != id {
                self.exit_editing();
            }
        }

        self.set_state(id, WidgetState::Editing(kind));
    }

    pub fn exit_editing(&mut self) {
        if let Some(id) = self.focused {
            self.set_state(id, WidgetState::Selected);
        }
    }

    pub fn is_editing(&self) -> bool {
        self.focused.is_some()
    }

    pub fn is_editing_shape(&self, id: ShapeId) -> bool {
        self.focused == Some(id)
    }

    pub fn remove(&mut self, id: ShapeId) {
        self.states.remove(&id);
        self.selected.remove(&id);
        if self.focused == Some(id) {
            self.focused = None;
        }
        if self.hovered == Some(id) {
            self.hovered = None;
        }
    }

    pub fn get_handles(&self, shape: &Shape) -> Vec<Handle> {
        let id = shape.id();
        if !self.is_selected(id) {
            return vec![];
        }

        shape_handles::get_shape_handles(shape)
    }
}

impl Default for WidgetManager {
    fn default() -> Self {
        Self::new()
    }
}

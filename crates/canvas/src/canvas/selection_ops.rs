use super::state::Canvas;
use crate::shapes::ShapeId;
use kurbo::{Affine, Rect, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignMode {
    Left,
    Right,
    Top,
    Bottom,
    CenterHorizontal,
    CenterVertical,
}

impl Canvas {
    pub fn transform_selected(&mut self, affine: Affine) {
        for &id in &self.selection.clone() {
            if let Some(mut shape) = self.document.get_shape_mut(id) {
                shape.transform(affine);
            }
        }
    }

    pub fn nudge_selected(&mut self, delta: Vec2) {
        if self.selection.is_empty() {
            return;
        }

        self.document.push_undo();
        self.transform_selected(Affine::translate(delta));
    }

    pub fn align_selected(&mut self, mode: AlignMode) {
        if self.selection.len() < 2 {
            return;
        }
        self.document.push_undo();
        let bounds: Vec<(ShapeId, Rect)> = self
            .selection
            .iter()
            .filter_map(|&id| self.document.get_shape(id).map(|s| (id, s.bounds())))
            .collect();

        match mode {
            AlignMode::Left => {
                let target = bounds
                    .iter()
                    .map(|(_, b)| b.x0)
                    .fold(f64::INFINITY, f64::min);
                for (id, b) in &bounds {
                    if let Some(mut s) = self.document.get_shape_mut(*id) {
                        s.transform(Affine::translate(Vec2::new(target - b.x0, 0.0)));
                    }
                }
            }
            AlignMode::Right => {
                let target = bounds
                    .iter()
                    .map(|(_, b)| b.x1)
                    .fold(f64::NEG_INFINITY, f64::max);
                for (id, b) in &bounds {
                    if let Some(mut s) = self.document.get_shape_mut(*id) {
                        s.transform(Affine::translate(Vec2::new(target - b.x1, 0.0)));
                    }
                }
            }
            AlignMode::Top => {
                let target = bounds
                    .iter()
                    .map(|(_, b)| b.y0)
                    .fold(f64::INFINITY, f64::min);
                for (id, b) in &bounds {
                    if let Some(mut s) = self.document.get_shape_mut(*id) {
                        s.transform(Affine::translate(Vec2::new(0.0, target - b.y0)));
                    }
                }
            }
            AlignMode::Bottom => {
                let target = bounds
                    .iter()
                    .map(|(_, b)| b.y1)
                    .fold(f64::NEG_INFINITY, f64::max);
                for (id, b) in &bounds {
                    if let Some(mut s) = self.document.get_shape_mut(*id) {
                        s.transform(Affine::translate(Vec2::new(0.0, target - b.y1)));
                    }
                }
            }
            AlignMode::CenterHorizontal => {
                let combined = bounds.iter().fold(None::<Rect>, |acc, (_, b)| {
                    Some(acc.map(|a| a.union(*b)).unwrap_or(*b))
                });
                if let Some(r) = combined {
                    let cy = r.center().y;
                    for (id, b) in &bounds {
                        if let Some(mut s) = self.document.get_shape_mut(*id) {
                            s.transform(Affine::translate(Vec2::new(0.0, cy - b.center().y)));
                        }
                    }
                }
            }
            AlignMode::CenterVertical => {
                let combined = bounds.iter().fold(None::<Rect>, |acc, (_, b)| {
                    Some(acc.map(|a| a.union(*b)).unwrap_or(*b))
                });
                if let Some(r) = combined {
                    let cx = r.center().x;
                    for (id, b) in &bounds {
                        if let Some(mut s) = self.document.get_shape_mut(*id) {
                            s.transform(Affine::translate(Vec2::new(cx - b.center().x, 0.0)));
                        }
                    }
                }
            }
        }
    }

    pub fn duplicate_selected(&mut self) -> Vec<ShapeId> {
        if self.selection.is_empty() {
            return Vec::new();
        }
        self.document.push_undo();
        let offset = Affine::translate(Vec2::new(20.0, 20.0));
        let new_ids: Vec<ShapeId> = self
            .selection
            .clone()
            .iter()
            .filter_map(|&id| {
                let mut shape = self.document.get_shape(id)?.clone();
                shape.regenerate_id();
                shape.transform(offset);
                let new_id = shape.id();
                self.document.add_shape(shape);
                Some(new_id)
            })
            .collect();
        self.clear_selection();
        for &id in &new_ids {
            self.add_to_selection(id);
        }
        new_ids
    }

    pub fn bring_to_front_selected(&mut self) {
        if self.selection.is_empty() {
            return;
        }
        self.document.push_undo();
        for &id in &self.selection.clone() {
            self.document.bring_to_front(id);
        }
    }

    pub fn send_to_back_selected(&mut self) {
        if self.selection.is_empty() {
            return;
        }
        self.document.push_undo();
        for &id in self.selection.clone().iter().rev() {
            self.document.send_to_back(id);
        }
    }

    pub fn bring_forward_selected(&mut self) {
        if self.selection.is_empty() {
            return;
        }
        self.document.push_undo();
        let mut ids = self.selection.clone();
        ids.sort_by_key(|id| {
            self.document
                .z_order
                .iter()
                .position(|z| z == id)
                .unwrap_or(0)
        });
        for &id in ids.iter().rev() {
            self.document.bring_forward(id);
        }
    }

    pub fn send_backward_selected(&mut self) {
        if self.selection.is_empty() {
            return;
        }
        self.document.push_undo();
        let mut ids = self.selection.clone();
        ids.sort_by_key(|id| {
            self.document
                .z_order
                .iter()
                .position(|z| z == id)
                .unwrap_or(0)
        });
        for &id in &ids {
            self.document.send_backward(id);
        }
    }
}

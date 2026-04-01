use super::core::TextEditState;
use parley::{FontContext, LayoutContext};
use peniko::Brush;

impl TextEditState {
    pub fn handle_mouse_down(
        &mut self,
        local_x: f32,
        local_y: f32,
        shift: bool,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext<Brush>,
    ) {
        self.cursor_reset();
        self.is_dragging = true;

        let mut drv = self.editor.driver(font_cx, layout_cx);
        if shift {
            drv.extend_selection_to_point(local_x, local_y);
        } else {
            drv.move_to_point(local_x, local_y);
        }
    }

    pub fn handle_mouse_drag(
        &mut self,
        local_x: f32,
        local_y: f32,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext<Brush>,
    ) {
        if !self.is_dragging {
            return;
        }

        self.cursor_reset();
        let mut drv = self.editor.driver(font_cx, layout_cx);
        drv.extend_selection_to_point(local_x, local_y);
    }

    pub fn handle_mouse_up(&mut self) {
        self.is_dragging = false;
    }

    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    pub fn handle_double_click(
        &mut self,
        local_x: f32,
        local_y: f32,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext<Brush>,
    ) {
        self.cursor_reset();
        let mut drv = self.editor.driver(font_cx, layout_cx);
        drv.select_word_at_point(local_x, local_y);
    }

    pub fn handle_triple_click(
        &mut self,
        local_x: f32,
        local_y: f32,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext<Brush>,
    ) {
        self.cursor_reset();
        let mut drv = self.editor.driver(font_cx, layout_cx);
        drv.select_hard_line_at_point(local_x, local_y);
    }
}

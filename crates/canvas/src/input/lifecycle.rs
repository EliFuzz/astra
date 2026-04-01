use winit_input_helper::WinitInputHelper;

use super::state::InputState;

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            helper: WinitInputHelper::new(),
            last_click_time: None,
            last_click_position: None,
            click_count: 0,
            double_click_detected: false,
            triple_click_detected: false,
            is_dragging: false,
            drag_start: None,
            touches: [None, None],
            pinch_distance: None,
            pinch_center: None,
            active_modifiers: winit::keyboard::ModifiersState::empty(),
        }
    }

    pub fn step(&mut self) {
        self.helper.step();
        self.double_click_detected = false;
        self.triple_click_detected = false;
    }

    pub fn end_step(&mut self) {
        self.helper.end_step();
    }
}

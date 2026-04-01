use crate::platform::Instant;
use kurbo::{Point, Vec2};
use winit::event::{MouseButton, WindowEvent};
use winit::keyboard::KeyCode;

use super::constants::{DOUBLE_CLICK_DISTANCE, DOUBLE_CLICK_TIME_MS};
use super::state::InputState;

impl InputState {
    pub fn mouse_position(&self) -> Point {
        let (x, y) = self.helper.cursor().unwrap_or((0.0, 0.0));
        Point::new(x as f64, y as f64)
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.helper.mouse_held(button)
    }

    pub fn mouse_just_pressed(&self, button: MouseButton) -> bool {
        self.helper.mouse_pressed(button)
    }

    pub fn mouse_just_released(&self, button: MouseButton) -> bool {
        self.helper.mouse_released(button)
    }

    pub fn scroll_delta(&self) -> Vec2 {
        let (dx, dy) = self.helper.scroll_diff();
        Vec2::new(dx as f64, dy as f64)
    }

    pub fn cursor_diff(&self) -> Vec2 {
        let (dx, dy) = self.helper.cursor_diff();
        Vec2::new(dx as f64, dy as f64)
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.helper.key_held(key)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.helper.key_pressed(key)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.helper.key_released(key)
    }

    pub fn shift(&self) -> bool {
        self.helper.held_shift()
    }

    pub fn ctrl(&self) -> bool {
        self.helper.held_control()
            || self.active_modifiers.control_key()
            || self.active_modifiers.super_key()
    }

    pub fn meta(&self) -> bool {
        self.active_modifiers.super_key()
    }

    pub fn alt(&self) -> bool {
        self.helper.held_alt()
    }

    pub fn is_double_click(&self) -> bool {
        self.double_click_detected
    }

    pub fn is_triple_click(&self) -> bool {
        self.triple_click_detected
    }

    pub fn drag_delta(&self) -> Option<Vec2> {
        self.drag_start.map(|start| {
            let pos = self.mouse_position();
            Vec2::new(pos.x - start.x, pos.y - start.y)
        })
    }

    pub fn close_requested(&self) -> bool {
        self.helper.close_requested()
    }

    pub fn process_window_event(&mut self, event: &WindowEvent) -> bool {
        if let WindowEvent::ModifiersChanged(modifiers) = event {
            self.active_modifiers = modifiers.state();
        }
        let result = self.helper.process_window_event(event);

        if self.mouse_just_pressed(MouseButton::Left) {
            let current_pos = self.mouse_position();
            let now = Instant::now();

            if let (Some(last_time), Some(last_pos)) =
                (self.last_click_time, self.last_click_position)
            {
                let elapsed = now.duration_since(last_time).as_millis();
                let distance = current_pos.distance(last_pos);

                if elapsed < DOUBLE_CLICK_TIME_MS && distance < DOUBLE_CLICK_DISTANCE {
                    self.click_count += 1;
                    if self.click_count == 2 {
                        self.double_click_detected = true;
                        self.last_click_time = Some(now);
                    } else if self.click_count >= 3 {
                        self.triple_click_detected = true;
                        self.click_count = 0;
                        self.last_click_time = None;
                        self.last_click_position = None;
                    }
                } else {
                    self.click_count = 1;
                    self.last_click_time = Some(now);
                    self.last_click_position = Some(current_pos);
                }
            } else {
                self.click_count = 1;
                self.last_click_time = Some(now);
                self.last_click_position = Some(current_pos);
            }

            if !self.is_dragging {
                self.is_dragging = true;
                self.drag_start = Some(current_pos);
            }
        }

        if self.mouse_just_released(MouseButton::Left) {
            self.is_dragging = false;
            self.drag_start = None;
        }

        result
    }

    pub fn process_device_event(&mut self, event: &winit::event::DeviceEvent) {
        self.helper.process_device_event(event);
    }
}

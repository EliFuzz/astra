use super::maps;
use super::text_edit;
use super::super::ui_actions::dispatch::dispatch_action_simple;
use super::super::App;
use crate::ui::UiAction;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{Key, NamedKey};

pub fn handle_keyboard_input(app: &mut App, event: KeyEvent, egui_wants_input: bool) {
    {
        let Some(state) = app.state.as_mut() else {
            return;
        };
        if egui_wants_input {
            state.needs_redraw = true;
            state.window.request_redraw();
            return;
        }
        if text_edit::handle_text_editing(state, &event) {
            return;
        }
    }

    let key_str = match &event.logical_key {
        Key::Named(named) => match named {
            NamedKey::Escape => "Escape",
            NamedKey::Delete => "Delete",
            NamedKey::Backspace => "Backspace",
            NamedKey::ArrowUp => "ArrowUp",
            NamedKey::ArrowDown => "ArrowDown",
            NamedKey::ArrowLeft => "ArrowLeft",
            NamedKey::ArrowRight => "ArrowRight",
            _ => return,
        },
        Key::Character(c) => c.as_str(),
        _ => return,
    };

    if event.state == ElementState::Pressed {
        let (has_modifier, has_shift) = app
            .state
            .as_ref()
            .map(|s| (s.input.ctrl(), s.input.shift()))
            .unwrap_or((false, false));

        if has_modifier {
            if let Some(action) = maps::shortcut_map(key_str, has_shift) {
                let Some(state) = app.state.as_mut() else {
                    return;
                };
                match action {
                    UiAction::ExportPng | UiAction::CopyPng => {
                        state.deferred_action = Some(action);
                    }
                    _ => dispatch_action_simple(state, &action),
                }
                state.needs_redraw = true;
                state.window.request_redraw();
                return;
            }
        } else if let Some(state) = app.state.as_mut() {
            if let Some(action) = maps::navigation_map(state, key_str) {
                dispatch_action_simple(state, &action);
            }
        }
    }

    if let Some(state) = app.state.as_mut() {
        state.needs_redraw = true;
        state.window.request_redraw();
    }
}

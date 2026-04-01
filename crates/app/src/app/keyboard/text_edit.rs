use super::super::AppState;
use crate::app::platform::{self, ClipboardResult};
use astra_canvas::shapes::Shape;
use astra_render::{TextEditResult, TextKey, TextModifiers};
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{Key, NamedKey};

pub(super) fn handle_text_editing(state: &mut AppState, event: &KeyEvent) -> bool {
    let Some(text_id) = state.event_handler.editing_text else {
        return false;
    };

    if event.state == ElementState::Pressed {
        if state.text_edit_state.is_none() {
            if let Some(Shape::Text(text)) = state.canvas.document.get_shape_deep(text_id) {
                let mut edit_state =
                    astra_render::TextEditState::new(&text.content, text.font_size as f32);
                edit_state.cursor_reset();
                state.text_edit_state = Some(edit_state);
            }
        }

        let has_ctrl = state.input.ctrl();
        let text_key = if has_ctrl {
            match &event.logical_key {
                Key::Character(c) if c == "c" || c == "C" => Some(TextKey::Copy),
                Key::Character(c) if c == "x" || c == "X" => Some(TextKey::Cut),
                Key::Character(c) if c == "v" || c == "V" => match platform::paste_text() {
                    ClipboardResult::Text(text) => Some(TextKey::Paste(text)),
                    ClipboardResult::Pending => {
                        state.needs_redraw = true;
                        state.window.request_redraw();
                        return true;
                    }
                    ClipboardResult::Empty => None,
                },
                Key::Character(c) if c == "a" || c == "A" => None,
                Key::Character(_) => return true,
                _ => None,
            }
        } else {
            None
        };

        let text_key = text_key.or_else(|| match &event.logical_key {
            Key::Named(NamedKey::Escape) => Some(TextKey::Escape),
            Key::Named(NamedKey::Backspace) => Some(TextKey::Backspace),
            Key::Named(NamedKey::Delete) => Some(TextKey::Delete),
            Key::Named(NamedKey::Enter) => Some(TextKey::Enter),
            Key::Named(NamedKey::ArrowLeft) => Some(TextKey::Left),
            Key::Named(NamedKey::ArrowRight) => Some(TextKey::Right),
            Key::Named(NamedKey::ArrowUp) => Some(TextKey::Up),
            Key::Named(NamedKey::ArrowDown) => Some(TextKey::Down),
            Key::Named(NamedKey::Home) => Some(TextKey::Home),
            Key::Named(NamedKey::End) => Some(TextKey::End),
            Key::Named(NamedKey::Space) => Some(TextKey::Character(" ".to_string())),
            Key::Character(c) => Some(TextKey::Character(c.to_string())),
            _ => None,
        });

        if let Some(key) = text_key {
            let modifiers = TextModifiers {
                shift: state.input.shift(),
                ctrl: state.input.ctrl(),
                alt: state.input.alt(),
                meta: state.input.meta(),
            };

            let (font_cx, layout_cx) = state.shape_renderer.contexts_mut();

            if let Some(edit_state) = &mut state.text_edit_state {
                let old_text = edit_state.text();
                let old_char_count = old_text.chars().count();
                let cursor_byte = edit_state.cursor_byte_offset();
                let edit_char_pos = old_text[..cursor_byte.min(old_text.len())].chars().count();

                let result = edit_state.handle_key(key, modifiers, font_cx, layout_cx);

                match result {
                    TextEditResult::ExitEdit => {
                        let new_text = edit_state.text();
                        update_text_content(
                            &mut state.canvas.document,
                            text_id,
                            new_text,
                            edit_char_pos,
                            old_char_count,
                        );
                        state.event_handler.exit_text_edit(&mut state.canvas);
                        state.text_edit_state = None;
                    }
                    TextEditResult::Handled => {
                        let new_text = edit_state.text();
                        update_text_content(
                            &mut state.canvas.document,
                            text_id,
                            new_text,
                            edit_char_pos,
                            old_char_count,
                        );
                    }
                    TextEditResult::Copy(text_to_copy) => {
                        platform::copy_text(&text_to_copy);
                        let new_text = edit_state.text();
                        update_text_content(
                            &mut state.canvas.document,
                            text_id,
                            new_text,
                            edit_char_pos,
                            old_char_count,
                        );
                    }
                    TextEditResult::NotHandled => {}
                }
            }
        }
    }
    state.needs_redraw = true;
    state.window.request_redraw();
    true
}

fn update_text_content(
    doc: &mut astra_canvas::canvas::CanvasDocument,
    text_id: astra_canvas::shapes::ShapeId,
    new_text: String,
    edit_char_pos: usize,
    old_char_count: usize,
) {
    if let Some(mut guard) = doc.get_shape_mut(text_id) {
        if let Shape::Text(text) = &mut *guard {
            text.content = new_text;
            text.sync_char_colors_after_edit(edit_char_pos, old_char_count);
        }
    } else {
        doc.with_group_child_mut(text_id, |shape| {
            if let Shape::Text(text) = shape {
                text.content = new_text;
                text.sync_char_colors_after_edit(edit_char_pos, old_char_count);
            }
        });
    }
}

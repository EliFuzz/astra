use super::super::AppState;
use super::super::platform;
use astra_canvas::shapes::Shape;

pub(super) fn apply_pending_items(state: &mut AppState) {
    platform::apply_platform_pending(state);

    if let Some(mut image_shape) = platform::take_pending_insert_image() {
        if let Shape::Image(ref mut img) = image_shape {
            let center = state
                .canvas
                .camera
                .viewport_center_world(state.canvas.viewport_size);
            *img = img.clone().centered_at(center);
        }
        state.canvas.insert_shape_and_select(image_shape);
        state.canvas.set_tool(astra_canvas::tools::ToolKind::Select);
        state.needs_redraw = true;
    }

    if let Some(clipboard_text) = platform::take_pending_clipboard_text() {
        apply_pending_clipboard_text(state, clipboard_text);
    }

    if let Some(clipboard_text) = platform::take_pending_math_clipboard() {
        if let Some((_, ref mut latex)) = state.ui_state.math_editor {
            *latex = clipboard_text;
        }
    }
}

fn apply_pending_clipboard_text(state: &mut AppState, clipboard_text: String) {
    let Some(text_id) = state.event_handler.editing_text else {
        return;
    };
    let Some(edit_state) = &mut state.text_edit_state else {
        return;
    };

    let old_text = edit_state.text();
    let old_char_count = old_text.chars().count();
    let cursor_byte = edit_state.cursor_byte_offset();
    let edit_char_pos = old_text[..cursor_byte.min(old_text.len())].chars().count();

    let (font_cx, layout_cx) = state.shape_renderer.contexts_mut();
    let _ = edit_state.handle_key(
        astra_render::TextKey::Paste(clipboard_text),
        astra_render::TextModifiers::default(),
        font_cx,
        layout_cx,
    );
    let new_text = edit_state.text();
    if let Some(mut guard) = state.canvas.document.get_shape_mut(text_id) {
        if let Shape::Text(text) = &mut *guard {
            text.content = new_text;
            text.sync_char_colors_after_edit(edit_char_pos, old_char_count);
        }
    } else {
        state
            .canvas
            .document
            .with_group_child_mut(text_id, |shape| {
                if let Shape::Text(text) = shape {
                    text.content = new_text;
                    text.sync_char_colors_after_edit(edit_char_pos, old_char_count);
                }
            });
    }
}

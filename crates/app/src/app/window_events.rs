use super::App;
use super::platform;
use super::{keyboard, mouse, presentation};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;

pub fn handle_window_event(app: &mut App, event_loop: &ActiveEventLoop, event: WindowEvent) {
    if platform::try_complete_pending_init(app, &event) {
        return;
    }

    let (egui_wants_pointer, egui_wants_keyboard) = {
        let Some(state) = app.state.as_mut() else {
            return;
        };
        state.input.process_window_event(&event);
        let egui_response = state.egui_state.on_window_event(&state.window, &event);
        let pointer = egui_response.consumed || state.egui_ctx.is_pointer_over_egui();
        let keyboard = egui_response.consumed || state.egui_ctx.egui_wants_keyboard_input();
        (pointer, keyboard)
    };

    match event {
        WindowEvent::CloseRequested => {
            event_loop.exit();
        }

        WindowEvent::Resized(size) => {
            handle_resized(app, size);
        }

        WindowEvent::RedrawRequested => {
            presentation::handle_redraw(app);
        }

        WindowEvent::CursorMoved { position, .. } => {
            if let Some(state) = app.state.as_mut() {
                mouse::handle_cursor_moved(state, position, egui_wants_pointer);
            }
        }

        WindowEvent::MouseInput {
            state: btn_state,
            button,
            ..
        } => {
            if let Some(state) = app.state.as_mut() {
                mouse::handle_mouse_input(state, btn_state, button, egui_wants_pointer);
            }
        }

        WindowEvent::MouseWheel { delta, .. } => {
            if let Some(state) = app.state.as_mut() {
                mouse::handle_mouse_wheel(state, delta, egui_wants_pointer);
            }
        }

        WindowEvent::Touch(touch) => {
            if let Some(state) = app.state.as_mut() {
                mouse::handle_touch(state, touch, egui_wants_pointer);
            }
        }

        WindowEvent::PinchGesture { delta, .. } => {
            if let Some(state) = app.state.as_mut() {
                mouse::handle_pinch_gesture(state, delta, egui_wants_pointer);
            }
        }

        WindowEvent::KeyboardInput { event, .. } => {
            keyboard::handle_keyboard_input(app, event, egui_wants_keyboard);
        }

        WindowEvent::ModifiersChanged(_) => {}

        WindowEvent::DroppedFile(path) => {
            if let Some(state) = app.state.as_mut() {
                platform::handle_dropped_file(state, path);
            }
        }

        _ => {}
    }
}

fn handle_resized(app: &mut App, size: PhysicalSize<u32>) {
    if size.width == 0 || size.height == 0 {
        return;
    }
    let Some(state) = app.state.as_mut() else {
        return;
    };
    state
        .canvas
        .set_viewport_size(size.width as f64, size.height as f64);
    if let Some(render_cx) = app.render_cx.as_mut() {
        render_cx.resize_surface(&mut state.surface, size.width, size.height);
    }
    state.needs_redraw = true;
    state.window.request_redraw();
}

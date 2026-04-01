use super::App;
use super::platform;
use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;

pub fn handle_resumed(app: &mut App, event_loop: &ActiveEventLoop) {
    if app.state.is_some() || app.pending_window.is_some() {
        return;
    }

    log::info!("Creating window...");

    let window_attrs = platform::create_window_attrs(&app.config);
    let window = Arc::new(
        event_loop
            .create_window(window_attrs)
            .expect("Failed to create window"),
    );

    log::info!("Window created, initializing renderer...");

    let size = window.inner_size();
    let (width, height) = if size.width == 0 || size.height == 0 {
        (app.config.width, app.config.height)
    } else {
        (size.width, size.height)
    };

    log::info!("Surface size: {}x{}", width, height);

    platform::create_surface(app, window, width, height);
}

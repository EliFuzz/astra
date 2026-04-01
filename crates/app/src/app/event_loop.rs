use super::App;
use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        super::resume::handle_resumed(self, event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        super::window_events::handle_window_event(self, event_loop, event);
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
        super::presentation::handle_new_events(self);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        super::presentation::handle_about_to_wait(self);
    }
}

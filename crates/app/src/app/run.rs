use super::{App, AppConfig, AppState};
use super::platform::{self, PlatformAppFields, PlatformStateFields};
use crate::ui::UiState;
use astra_canvas::EventHandler;
use astra_canvas::canvas::Canvas;
use astra_canvas::input::InputState;
use astra_render::VelloRenderer;
use std::sync::Arc;
use vello::RendererOptions;
use vello::util::RenderSurface;
use winit::event_loop::EventLoop;
use winit::window::Window;

impl App {
    pub fn new() -> Self {
        Self::with_config(AppConfig::default())
    }

    pub fn with_config(config: AppConfig) -> Self {
        Self {
            config,
            state: None,
            render_cx: None,
            pending_window: None,
            platform: PlatformAppFields::new(),
        }
    }

    pub async fn run() {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        let app = App::new();
        platform::run_event_loop(event_loop, app);
    }

    pub(crate) fn finish_init(&mut self, window: Arc<Window>, surface: RenderSurface<'static>) {
        let render_cx = self
            .render_cx
            .as_ref()
            .expect("RenderContext not initialized");
        let device = &render_cx.devices[surface.dev_id].device;

        let vello_renderer = vello::Renderer::new(device, RendererOptions::default())
            .expect("Failed to create Vello renderer");

        let texture_blitter =
            vello::wgpu::util::TextureBlitter::new(device, surface.config.format);

        let egui_ctx = egui::Context::default();
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );
        let egui_renderer = egui_wgpu::Renderer::new(
            device,
            surface.config.format,
            egui_wgpu::RendererOptions::default(),
        );

        let mut canvas = Canvas::new();
        canvas.set_viewport_size(surface.config.width as f64, surface.config.height as f64);

        log::info!(
            "Astra initialized - {}x{}",
            surface.config.width,
            surface.config.height
        );
        log::info!(
            "Keyboard shortcuts: V=Select, H=Pan, R=Rectangle, E=Ellipse, L=Line, A=Arrow, P=Pen"
        );

        self.state = Some(AppState {
            window: window.clone(),
            surface,
            vello_renderer,
            shape_renderer: VelloRenderer::new(),
            texture_blitter,
            egui_ctx,
            egui_state,
            egui_renderer,
            ui_state: UiState::default(),
            canvas,
            input: InputState::new(),
            config: self.config.clone(),
            event_handler: EventHandler::new(),
            text_edit_state: None,
            needs_redraw: true,
            deferred_action: None,
            render_texture: None,
            platform: PlatformStateFields::new(),
        });

        platform::on_finish_init(self);
        self.pending_window = None;
        window.request_redraw();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

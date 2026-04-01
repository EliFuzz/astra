use astra_canvas::EventHandler;
use astra_canvas::canvas::Canvas;
use astra_canvas::input::InputState;
use astra_render::{TextEditState, VelloRenderer};
use std::sync::Arc;
use vello::util::RenderSurface;
use winit::window::Window;

use crate::app::config::AppConfig;
use crate::app::platform::PlatformStateFields;
use crate::ui::{UiAction, UiState};

pub(crate) struct AppState {
    pub(crate) window: Arc<Window>,
    pub(crate) surface: RenderSurface<'static>,
    pub(crate) vello_renderer: vello::Renderer,
    pub(crate) shape_renderer: VelloRenderer,
    pub(crate) texture_blitter: vello::wgpu::util::TextureBlitter,
    pub(crate) egui_ctx: egui::Context,
    pub(crate) egui_state: egui_winit::State,
    pub(crate) egui_renderer: egui_wgpu::Renderer,
    pub(crate) ui_state: UiState,
    pub(crate) canvas: Canvas,
    pub(crate) input: InputState,
    pub(crate) config: AppConfig,
    pub(crate) event_handler: EventHandler,
    pub(crate) text_edit_state: Option<TextEditState>,
    pub(crate) needs_redraw: bool,
    pub(crate) deferred_action: Option<UiAction>,
    pub(crate) render_texture: Option<(vello::wgpu::Texture, u32, u32)>,
    #[allow(dead_code)]
    pub(crate) platform: PlatformStateFields,
}

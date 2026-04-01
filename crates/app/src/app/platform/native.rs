use super::ClipboardResult;
use crate::app::config::AppConfig;
use crate::app::state::AppState;
use crate::app::App;
use astra_canvas::canvas::CanvasDocument;
use astra_canvas::shapes::Shape;
use astra_storage::native_io::{self, DroppedFile};
use astra_storage::preferences::UserPreferences;
use std::sync::Arc;
use vello::util::RenderSurface;
use vello::wgpu::PresentMode;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::Window;

pub(crate) struct PlatformAppFields;

impl PlatformAppFields {
    pub fn new() -> Self {
        Self
    }
}

pub(crate) struct PlatformStateFields;

impl PlatformStateFields {
    pub fn new() -> Self {
        Self
    }
}

pub(crate) fn run_event_loop(event_loop: EventLoop<()>, mut app: App) {
    event_loop.run_app(&mut app).expect("Event loop error");
}

pub(crate) fn create_window_attrs(config: &AppConfig) -> winit::window::WindowAttributes {
    Window::default_attributes()
        .with_title(&config.title)
        .with_inner_size(LogicalSize::new(config.width, config.height))
}

pub(crate) fn create_surface(app: &mut App, window: Arc<Window>, width: u32, height: u32) {
    let render_cx = app
        .render_cx
        .get_or_insert_with(vello::util::RenderContext::new);
    let surface = pollster::block_on(render_cx.create_surface(
        window.clone(),
        width,
        height,
        PresentMode::AutoVsync,
    ))
    .expect("Failed to create surface");
    let surface: RenderSurface<'static> = unsafe { std::mem::transmute(surface) };
    app.finish_init(window, surface);
}

pub(crate) fn on_finish_init(app: &mut App) {
    if let Some(ref mut state) = app.state {
        let prefs = astra_storage::preferences::load_from_file().unwrap_or_default();
        crate::app::preferences::apply(state, &prefs);
    }
}

pub(crate) fn try_complete_pending_init(_app: &mut App, _event: &WindowEvent) -> bool {
    false
}

pub(crate) fn handle_dropped_file(state: &mut AppState, path: std::path::PathBuf) {
    match native_io::load_dropped_file(&path, &state.canvas) {
        Ok(Some(DroppedFile::Document(document))) => {
            log::info!("Found embedded scene in PNG, loading as document");
            state.canvas.replace_document_and_reset_camera(document);
            state.needs_redraw = true;
            state.window.request_redraw();
        }
        Ok(Some(DroppedFile::Shape(shape))) => {
            state.canvas.insert_shape_and_select(shape);
            state.needs_redraw = true;
            state.window.request_redraw();
        }
        Ok(None) => {}
        Err(error) => {
            log::error!("Failed to process dropped file {:?}: {}", path, error);
        }
    }
}

pub(crate) fn apply_platform_pending(state: &mut AppState) {
    if let Some(doc) = native_io::take_pending_document() {
        state.canvas.replace_document(doc);
        state.canvas.fit_to_content();
        state.needs_redraw = true;
    }
}

pub(crate) fn check_autosave(_state: &mut AppState) {}

pub(crate) fn take_pending_insert_image() -> Option<Shape> {
    native_io::take_pending_insert_image()
}

pub(crate) fn take_pending_clipboard_text() -> Option<String> {
    None
}

pub(crate) fn take_pending_math_clipboard() -> Option<String> {
    None
}

pub(crate) fn save_document(doc: &CanvasDocument, name: &str) {
    native_io::save_document(doc, name);
}

pub(crate) fn load_document() {
    native_io::load_document();
}

pub(crate) fn load_document_by_name(name: &str) {
    native_io::load_document_by_name(name);
}

pub(crate) fn list_documents() {}

pub(crate) fn save_document_with_name(doc: &mut CanvasDocument, name: &str) {
    native_io::save_document(doc, name);
}

pub(crate) fn download_document(doc: &CanvasDocument, name: &str) {
    native_io::save_document(doc, name);
}

pub(crate) fn upload_document() {
    native_io::load_document();
}

pub(crate) fn insert_image() {
    native_io::insert_image_async();
}

pub(crate) fn copy_text(text: &str) {
    if let Ok(mut cb) = arboard::Clipboard::new() {
        let _ = cb.set_text(text);
    }
}

pub(crate) fn paste_text() -> ClipboardResult {
    arboard::Clipboard::new()
        .ok()
        .and_then(|mut cb| cb.get_text().ok())
        .map(ClipboardResult::Text)
        .unwrap_or(ClipboardResult::Empty)
}

pub(crate) fn request_clipboard_for_math() {}

pub(crate) fn paste_from_external_clipboard(state: &mut AppState) {
    if let Ok(mut cb) = arboard::Clipboard::new() {
        if let Ok(text) = cb.get_text() {
            if let Some(shapes) = CanvasDocument::shapes_from_clipboard(&text) {
                let cursor_world = state
                    .canvas
                    .camera
                    .screen_to_world(state.input.mouse_position());
                state.canvas.insert_shapes_centered_at(shapes, cursor_world);
                return;
            }
        }
    }
    if let Some(image_shape) = native_io::paste_image_from_clipboard(&state.canvas) {
        state.canvas.insert_shape_and_select(image_shape);
    }
}

pub(crate) fn export_png(
    state: &mut AppState,
    render_cx: &vello::util::RenderContext,
    scene: vello::Scene,
    width: u32,
    height: u32,
) {
    let device_handle = &render_cx.devices[state.surface.dev_id];
    let device = &device_handle.device;
    let queue = &device_handle.queue;

    if let Some(result) = astra_render::render_scene_to_png(
        device,
        queue,
        &mut state.vello_renderer,
        &scene,
        width,
        height,
    ) {
        native_io::copy_png_to_clipboard(&result.rgba_data, result.width, result.height);
    }
}

pub(crate) fn export_png_to_file(
    state: &mut AppState,
    render_cx: &vello::util::RenderContext,
    width: u32,
    height: u32,
    padded_bounds: kurbo::Rect,
    export_scale: f64,
    selection: Option<Vec<astra_canvas::shapes::ShapeId>>,
) {
    let device_handle = &render_cx.devices[state.surface.dev_id];
    let device = device_handle.device.clone();
    let queue = device_handle.queue.clone();
    let document = state.canvas.document.clone_for_export();
    let doc_name = document.name.clone();

    std::thread::spawn(move || {
        let Some(path) = native_io::pick_png_save_path(&doc_name) else {
            return;
        };
        let export = astra_render::TiledExport {
            document,
            padded_bounds,
            scale: export_scale,
            selection,
        };
        if let Err(e) =
            astra_render::export_scene_to_png_file(&device, &queue, &export, width, height, &path)
        {
            log::error!("PNG export failed: {}", e);
        }
    });
}

pub(crate) fn save_preferences(prefs: &UserPreferences) {
    if let Err(e) = astra_storage::preferences::save_to_file(prefs) {
        log::warn!("Failed to save preferences: {:?}", e);
    }
}

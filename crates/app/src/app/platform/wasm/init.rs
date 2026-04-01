use crate::app::config::AppConfig;
use crate::app::App;
use astra_web::file_ops;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::platform::web::WindowAttributesExtWebSys;
use winit::window::Window;

pub(crate) struct PlatformAppFields {
    pub(crate) init_in_progress: std::cell::Cell<bool>,
}

impl PlatformAppFields {
    pub fn new() -> Self {
        Self {
            init_in_progress: std::cell::Cell::new(false),
        }
    }
}

pub(crate) struct PlatformStateFields {
    pub(crate) last_autosave: web_time::Instant,
    pub(crate) last_doc_version: u64,
}

impl PlatformStateFields {
    pub fn new() -> Self {
        Self {
            last_autosave: web_time::Instant::now(),
            last_doc_version: 0,
        }
    }
}

pub(crate) fn run_event_loop(event_loop: EventLoop<()>, app: App) {
    use winit::platform::web::EventLoopExtWebSys;
    event_loop.spawn_app(app);
}

pub(crate) fn create_window_attrs(config: &AppConfig) -> winit::window::WindowAttributes {
    let web_window = web_sys::window().expect("No window");
    let document = web_window.document().expect("No document");

    let viewport_width = web_window
        .inner_width()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(config.width as f64);
    let viewport_height = web_window
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(config.height as f64);

    if let Some(loading) = document.get_element_by_id("loading") {
        let _ = loading.remove();
    }

    let canvas = document
        .get_element_by_id("astra-canvas")
        .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        .or_else(|| {
            let app_div = document.get_element_by_id("app")?;
            let canvas = document.create_element("canvas").ok()?;
            canvas.set_id("astra-canvas");
            app_div.append_child(&canvas).ok()?;
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok()
        })
        .expect("Failed to create canvas");

    let dpr = web_window.device_pixel_ratio();
    let physical_width = (viewport_width * dpr) as u32;
    let physical_height = (viewport_height * dpr) as u32;

    canvas.set_width(physical_width);
    canvas.set_height(physical_height);
    let style = canvas.style();
    let _ = style.set_property("width", "100%");
    let _ = style.set_property("height", "100%");
    let _ = style.set_property("display", "block");
    let _ = style.set_property("position", "fixed");
    let _ = style.set_property("top", "0");
    let _ = style.set_property("left", "0");

    log::info!(
        "Canvas created: {}x{} (physical: {}x{}, dpr: {})",
        viewport_width,
        viewport_height,
        physical_width,
        physical_height,
        dpr
    );

    Window::default_attributes()
        .with_title(&config.title)
        .with_canvas(Some(canvas))
}

pub(crate) fn create_surface(app: &mut App, window: Arc<Window>, _width: u32, _height: u32) {
    app.pending_window = Some(window);
}

pub(crate) fn on_finish_init(app: &mut App) {
    file_ops::try_load_last_document();
    file_ops::try_load_preferences();

    if let Some(ref state) = app.state {
        let vw = state.canvas.viewport_size.width;
        let vh = state.canvas.viewport_size.height;
        let cox = state.canvas.camera.offset.x;
        let coy = state.canvas.camera.offset.y;
        let cz = state.canvas.camera.zoom;
        file_ops::setup_drag_drop_handlers(vw, vh, cox, coy, cz);
    }
}

pub(crate) fn try_complete_pending_init(app: &mut App, _event: &WindowEvent) -> bool {
    if app.state.is_some() {
        return false;
    }

    let Some(window) = app.pending_window.clone() else {
        return false;
    };

    if app.platform.init_in_progress.get() {
        window.request_redraw();
        return true;
    }

    app.platform.init_in_progress.set(true);

    let web_window = web_sys::window().expect("No window");
    let dpr = web_window.device_pixel_ratio();
    let viewport_width = web_window
        .inner_width()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(app.config.width as f64);
    let viewport_height = web_window
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(app.config.height as f64);

    let width = (viewport_width * dpr) as u32;
    let height = (viewport_height * dpr) as u32;

    let self_ptr = app as *mut App;
    let window_clone = window.clone();

    wasm_bindgen_futures::spawn_local(async move {
        log::info!("Creating surface asynchronously...");
        let mut render_cx = vello::util::RenderContext::new();

        match render_cx
            .create_surface(
                window_clone.clone(),
                width,
                height,
                vello::wgpu::PresentMode::AutoVsync,
            )
            .await
        {
            Ok(surface) => {
                log::info!("Surface created successfully");
                let surface: vello::util::RenderSurface<'static> =
                    unsafe { std::mem::transmute(surface) };
                let app = unsafe { &mut *self_ptr };
                app.render_cx = Some(render_cx);
                app.finish_init(window_clone, surface);
            }
            Err(e) => {
                log::error!("Failed to create surface: {:?}", e);
                let app = unsafe { &mut *self_ptr };
                app.platform.init_in_progress.set(false);
            }
        }
    });

    window.request_redraw();
    true
}

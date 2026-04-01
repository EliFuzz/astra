use crate::app::platform::ClipboardResult;
use crate::app::state::AppState;
use astra_canvas::canvas::CanvasDocument;
use astra_canvas::shapes::Shape;
use astra_storage::preferences::UserPreferences;
use astra_web::file_ops;

pub(crate) fn handle_dropped_file(_state: &mut AppState, _path: std::path::PathBuf) {}

pub(crate) fn apply_platform_pending(state: &mut AppState) {
    if let Some(doc) = file_ops::take_pending_document() {
        state.canvas.replace_document(doc);
        state.canvas.fit_to_content();
        state.needs_redraw = true;
    }

    if let Some(prefs) = file_ops::take_pending_preferences() {
        crate::app::preferences::apply(state, &prefs);
        state.needs_redraw = true;
    }

    if let Some(docs) = file_ops::take_pending_document_list() {
        state.ui_state.set_recent_documents(docs);
    }

    if let Some(image_shape) = file_ops::take_pending_image() {
        state.canvas.insert_shape_and_select(image_shape);
        state.needs_redraw = true;
    }

    if let Some((shapes, cursor_world)) = file_ops::take_pending_shapes() {
        state.canvas.insert_shapes_centered_at(shapes, cursor_world);
        log::info!("Pasted shapes from clipboard");
        state.needs_redraw = true;
    }

    for image_shape in file_ops::take_pending_dropped_images() {
        state.canvas.insert_shape_and_select(image_shape);
        state.needs_redraw = true;
    }

    if let Some(json) = file_ops::take_pending_dropped_document() {
        match CanvasDocument::from_json(&json) {
            Ok(doc) => {
                log::info!("Loaded document from dropped PNG");
                state.canvas.replace_document_and_reset_camera(doc);
                state.needs_redraw = true;
            }
            Err(e) => log::error!("Failed to parse embedded document: {}", e),
        }
    }
}

pub(crate) fn check_autosave(state: &mut AppState) {
    let doc_version =
        state.canvas.document.shapes.len() as u64 + state.canvas.document.z_order.len() as u64;
    if doc_version != state.platform.last_doc_version
        && state.platform.last_autosave.elapsed().as_secs() >= 5
    {
        file_ops::autosave_document(&state.canvas.document);
        state.platform.last_autosave = web_time::Instant::now();
        state.platform.last_doc_version = doc_version;
    }
}

pub(crate) fn take_pending_insert_image() -> Option<Shape> {
    file_ops::take_pending_insert_image()
}

pub(crate) fn take_pending_clipboard_text() -> Option<String> {
    file_ops::take_pending_clipboard_text()
}

pub(crate) fn take_pending_math_clipboard() -> Option<String> {
    file_ops::take_pending_math_clipboard()
}

pub(crate) fn save_document(doc: &CanvasDocument, name: &str) {
    file_ops::save_document(doc, name);
}

pub(crate) fn load_document() {
    file_ops::load_document_async();
}

pub(crate) fn load_document_by_name(name: &str) {
    file_ops::load_document_by_name_async(name);
}

pub(crate) fn list_documents() {
    file_ops::list_documents_async();
}

pub(crate) fn save_document_with_name(doc: &mut CanvasDocument, name: &str) {
    let doc_id = doc.id.clone();
    doc.id = name.to_string();
    file_ops::save_document(doc, name);
    doc.id = doc_id;
}

pub(crate) fn download_document(doc: &CanvasDocument, name: &str) {
    file_ops::download_document(doc, name);
}

pub(crate) fn upload_document() {
    file_ops::upload_document_async();
}

pub(crate) fn insert_image() {
    file_ops::insert_image_async();
}

pub(crate) fn copy_text(text: &str) {
    file_ops::copy_text_to_clipboard(text);
}

pub(crate) fn paste_text() -> ClipboardResult {
    file_ops::request_clipboard_text();
    ClipboardResult::Pending
}

pub(crate) fn request_clipboard_for_math() {
    file_ops::request_clipboard_text_for_math();
}

pub(crate) fn paste_from_external_clipboard(state: &mut AppState) {
    let cursor_world = state
        .canvas
        .camera
        .screen_to_world(state.input.mouse_position());
    file_ops::paste_shapes_from_clipboard_async(cursor_world);
    file_ops::paste_image_from_clipboard_async(
        state.canvas.viewport_size.width,
        state.canvas.viewport_size.height,
        state.canvas.camera.offset.x,
        state.canvas.camera.offset.y,
        state.canvas.camera.zoom,
    );
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
    let filename = format!("{}.png", state.canvas.document.name);
    file_ops::spawn_png_export_async(
        device,
        queue,
        scene,
        width,
        height,
        filename,
        true,
        None,
    );
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
    let device = &device_handle.device;
    let queue = &device_handle.queue;
    let document = state.canvas.document.clone_for_export();
    let filename = format!("{}.png", document.name);

    let export = astra_render::TiledExport {
        document,
        padded_bounds,
        scale: export_scale,
        selection,
    };

    file_ops::spawn_tiled_png_export_async(device, queue, export, width, height, filename, false);
}

pub(crate) fn save_preferences(prefs: &UserPreferences) {
    file_ops::save_preferences(prefs);
}

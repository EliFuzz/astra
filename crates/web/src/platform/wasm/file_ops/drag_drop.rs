use astra_canvas::shapes::{Image, Shape};
use astra_storage::pending;
use kurbo::Point;
use wasm_bindgen::prelude::*;

pub fn setup_drag_drop_handlers(
    viewport_width: f64,
    viewport_height: f64,
    camera_offset_x: f64,
    camera_offset_y: f64,
    camera_zoom: f64,
) {
    use wasm_bindgen::closure::Closure;

    let window = web_sys::window().expect("No window");
    let document = window.document().expect("No document");
    let Some(canvas) = document.query_selector("canvas").ok().flatten() else {
        return;
    };

    let ondragover = Closure::wrap(Box::new(move |event: web_sys::DragEvent| {
        event.prevent_default();
    }) as Box<dyn Fn(_)>);
    canvas
        .add_event_listener_with_callback("dragover", ondragover.as_ref().unchecked_ref())
        .ok();
    ondragover.forget();

    let vw = viewport_width;
    let vh = viewport_height;
    let cox = camera_offset_x;
    let coy = camera_offset_y;
    let cz = camera_zoom;

    let ondrop = Closure::wrap(Box::new(move |event: web_sys::DragEvent| {
        event.prevent_default();
        let Some(dt) = event.data_transfer() else {
            return;
        };
        let Some(files) = dt.files() else {
            return;
        };
        for i in 0..files.length() {
            if let Some(file) = files.get(i) {
                handle_dropped_file(file, vw, vh, cox, coy, cz);
            }
        }
    }) as Box<dyn Fn(_)>);
    canvas
        .add_event_listener_with_callback("drop", ondrop.as_ref().unchecked_ref())
        .ok();
    ondrop.forget();
}

fn handle_dropped_file(file: web_sys::File, vw: f64, vh: f64, cox: f64, coy: f64, cz: f64) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = process_dropped_file(file, vw, vh, cox, coy, cz).await {
            log::error!("Failed to process dropped file: {:?}", e);
        }
    });
}

async fn process_dropped_file(
    file: web_sys::File,
    vw: f64,
    vh: f64,
    cox: f64,
    coy: f64,
    cz: f64,
) -> Result<(), JsValue> {
    let file_name = file.name();
    let file_type = file.type_();
    let array_buffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await?;
    let data = js_sys::Uint8Array::new(&array_buffer).to_vec();

    if file_type == "image/png" || file_name.ends_with(".png") {
        if let Some(json) = astra_core::png::extract_scene_from_png(&data) {
            pending::set_pending_dropped_document(json);
            return Ok(());
        }
    }

    let format = super::format_hint::image_format(&file_type, &file_name);
    let blob: web_sys::Blob = file.into();
    let blob_url = web_sys::Url::create_object_url_with_blob(&blob)?;
    let (width, height) = super::image::decode_image_dimensions(&blob_url).await?;
    web_sys::Url::revoke_object_url(&blob_url)?;

    let center = Point::new((vw / 2.0 - cox) / cz, (vh / 2.0 - coy) / cz);
    let image = Image::new_centered(center, &data, width, height, format);

    log::info!("Dropped image: {} ({}x{})", file_name, width, height);
    pending::push_pending_dropped_image(Shape::Image(image));
    Ok(())
}

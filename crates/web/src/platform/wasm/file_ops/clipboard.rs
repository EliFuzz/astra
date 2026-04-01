use astra_storage::pending;
use wasm_bindgen::prelude::*;

pub fn request_clipboard_text() {
    spawn_clipboard_read(pending::set_pending_clipboard_text);
}

pub fn request_clipboard_text_for_math() {
    spawn_clipboard_read(pending::set_pending_math_clipboard);
}

fn spawn_clipboard_read(apply: fn(String)) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Some(text) = read_clipboard_text_async().await {
            apply(text);
        }
    });
}

pub(super) async fn read_clipboard_text_async() -> Option<String> {
    let window = web_sys::window()?;
    let clipboard = window.navigator().clipboard();
    let promise = clipboard.read_text();
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .ok()?
        .as_string()
}

pub fn copy_text_to_clipboard(text: &str) {
    let text = text.to_string();
    wasm_bindgen_futures::spawn_local(async move {
        if let Some(window) = web_sys::window() {
            let promise = window.navigator().clipboard().write_text(&text);
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
        }
    });
}

pub fn copy_png_to_clipboard(png_data: Vec<u8>) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = copy_png_async(png_data).await {
            log::error!("Failed to copy PNG to clipboard: {:?}", e);
        }
    });
}

async fn copy_png_async(png_data: Vec<u8>) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let clipboard = window.navigator().clipboard();

    let uint8_array = js_sys::Uint8Array::from(png_data.as_slice());
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&uint8_array);

    let options = web_sys::BlobPropertyBag::new();
    options.set_type("image/png");
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &options)?;

    let item_options = js_sys::Object::new();
    js_sys::Reflect::set(&item_options, &"image/png".into(), &blob)?;
    let clipboard_item = web_sys::ClipboardItem::new_with_record_from_str_to_blob_promise(
        &item_options.unchecked_into(),
    )?;

    let items = js_sys::Array::new();
    items.push(&clipboard_item);
    wasm_bindgen_futures::JsFuture::from(clipboard.write(&items)).await?;
    log::info!("PNG copied to clipboard ({} bytes)", png_data.len());
    Ok(())
}

pub fn paste_image_from_clipboard_async(
    viewport_width: f64,
    viewport_height: f64,
    camera_offset_x: f64,
    camera_offset_y: f64,
    camera_zoom: f64,
) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = paste_image_async(
            viewport_width,
            viewport_height,
            camera_offset_x,
            camera_offset_y,
            camera_zoom,
        )
        .await
        {
            log::debug!("No image in clipboard: {:?}", e);
        }
    });
}

async fn paste_image_async(
    viewport_width: f64,
    viewport_height: f64,
    camera_offset_x: f64,
    camera_offset_y: f64,
    camera_zoom: f64,
) -> Result<(), JsValue> {
    use astra_canvas::shapes::{Image, ImageFormat, Shape};
    use kurbo::Point;

    let window = web_sys::window().ok_or("No window")?;
    let clipboard = window.navigator().clipboard();
    let items: js_sys::Array = wasm_bindgen_futures::JsFuture::from(clipboard.read())
        .await?
        .dyn_into()?;

    for i in 0..items.length() {
        let item: web_sys::ClipboardItem = items.get(i).dyn_into()?;
        let types = item.types();
        for j in 0..types.length() {
            let mime_type = match types.get(j).as_string() {
                Some(t) if t.starts_with("image/") => t,
                _ => continue,
            };
            let blob: web_sys::Blob =
                wasm_bindgen_futures::JsFuture::from(item.get_type(&mime_type))
                    .await?
                    .dyn_into()?;
            let array_buffer = wasm_bindgen_futures::JsFuture::from(blob.array_buffer()).await?;
            let data = js_sys::Uint8Array::new(&array_buffer).to_vec();
            let format = match mime_type.as_str() {
                "image/jpeg" => ImageFormat::Jpeg,
                "image/webp" => ImageFormat::WebP,
                _ => ImageFormat::Png,
            };
            let blob_url = web_sys::Url::create_object_url_with_blob(&blob)?;
            let (width, height) = super::image::decode_image_dimensions(&blob_url).await?;
            web_sys::Url::revoke_object_url(&blob_url)?;

            let center = Point::new(
                (viewport_width / 2.0 - camera_offset_x) / camera_zoom,
                (viewport_height / 2.0 - camera_offset_y) / camera_zoom,
            );
            let image = Image::new_centered(center, &data, width, height, format);
            pending::set_pending_image(Shape::Image(image));
            return Ok(());
        }
    }
    Err("No image in clipboard".into())
}

pub fn paste_shapes_from_clipboard_async(cursor_world: kurbo::Point) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Some(text) = read_clipboard_text_async().await {
            if let Some(shapes) =
                astra_canvas::canvas::CanvasDocument::shapes_from_clipboard(&text)
            {
                pending::set_pending_shapes(shapes, cursor_world);
            }
        }
    });
}

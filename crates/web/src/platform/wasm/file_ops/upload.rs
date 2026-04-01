use astra_canvas::canvas::CanvasDocument;
use astra_canvas::shapes::{Image, Shape};
use astra_storage::pending;
use kurbo::Point;
use wasm_bindgen::prelude::*;

pub fn upload_document_async() {
    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = upload_document_impl().await {
            log::error!("Failed to load file: {:?}", e);
        }
    });
}

pub fn insert_image_async() {
    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = insert_image_impl().await {
            log::error!("Failed to insert image: {:?}", e);
        }
    });
}

async fn upload_document_impl() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    let input: web_sys::HtmlInputElement = document.create_element("input")?.dyn_into()?;
    input.set_type("file");
    input.set_accept(".json,.excalidraw");
    input.style().set_property("display", "none").ok();
    document.body().ok_or("No body")?.append_child(&input)?;

    let file = super::file_input::wait_for_file_selection(&input).await;
    input.remove();
    let file = file?;

    let filename = file.name();
    let is_excalidraw = filename.to_lowercase().ends_with(".excalidraw");
    let text: String = wasm_bindgen_futures::JsFuture::from(file.text())
        .await?
        .as_string()
        .ok_or("Failed to read file")?;

    let doc = if is_excalidraw {
        CanvasDocument::shapes_from_json(&text).map_err(|e| JsValue::from_str(&e.to_string()))?
    } else {
        CanvasDocument::from_json(&text).map_err(|e| JsValue::from_str(&e.to_string()))?
    };

    log::info!("Document loaded: {}", doc.name);
    pending::set_pending_document(doc);
    Ok(())
}

async fn insert_image_impl() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;

    let input: web_sys::HtmlInputElement = document.create_element("input")?.dyn_into()?;
    input.set_type("file");
    input.set_accept("image/png,image/jpeg,image/webp");
    input.style().set_property("display", "none").ok();
    document.body().ok_or("No body")?.append_child(&input)?;

    let file = super::file_input::wait_for_file_selection(&input).await;
    input.remove();
    let file = file?;

    let file_name = file.name();
    let file_type = file.type_();
    let format = super::format_hint::image_format(&file_type, &file_name);

    let array_buffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await?;
    let data = js_sys::Uint8Array::new(&array_buffer).to_vec();

    let blob_url = super::file_input::blob_url_from_array_buffer(&array_buffer, &file_type)?;
    let (w, h) = super::image::decode_image_dimensions(&blob_url)
        .await
        .unwrap_or((800, 600));
    web_sys::Url::revoke_object_url(&blob_url)?;

    let image = Image::new_centered(Point::new(100.0, 100.0), &data, w, h, format);

    pending::set_pending_insert_image(Shape::Image(image));
    log::info!("Image loaded for insertion: {}x{}", w, h);
    Ok(())
}

use wasm_bindgen::prelude::*;

pub fn download_document(document: &astra_canvas::canvas::CanvasDocument, name: &str) {
    match document.to_json() {
        Ok(json) => download_file(&format!("{}.json", name), &json, "application/json"),
        Err(e) => log::error!("Failed to serialize document: {}", e),
    }
}

pub fn export_png(png_data: &[u8], name: &str) {
    download_binary_file(&format!("{}.png", name), png_data, "image/png");
}

fn download_file(filename: &str, content: &str, mime_type: &str) {
    let parts = js_sys::Array::new();
    parts.push(&JsValue::from_str(content));
    let options = web_sys::BlobPropertyBag::new();
    options.set_type(mime_type);
    let blob = web_sys::Blob::new_with_str_sequence_and_options(&parts, &options)
        .expect("Failed to create blob");
    trigger_blob_download(&blob, filename);
}

fn download_binary_file(filename: &str, data: &[u8], mime_type: &str) {
    let uint8_array = js_sys::Uint8Array::from(data);
    let parts = js_sys::Array::new();
    parts.push(&uint8_array);
    let options = web_sys::BlobPropertyBag::new();
    options.set_type(mime_type);
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &options)
        .expect("Failed to create blob");
    trigger_blob_download(&blob, filename);
}

fn trigger_blob_download(blob: &web_sys::Blob, filename: &str) {
    let window = web_sys::window().expect("No window");
    let document = window.document().expect("No document");
    let url = web_sys::Url::create_object_url_with_blob(blob).expect("Failed to create URL");
    let a = document
        .create_element("a")
        .expect("Failed to create element")
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .expect("Failed to cast to anchor");
    a.set_href(&url);
    a.set_download(filename);
    a.click();
    web_sys::Url::revoke_object_url(&url).ok();
}

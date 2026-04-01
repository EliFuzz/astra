use wasm_bindgen::prelude::*;

pub(super) async fn wait_for_file_selection(
    input: &web_sys::HtmlInputElement,
) -> Result<web_sys::File, JsValue> {
    use wasm_bindgen::closure::Closure;
    let input_clone = input.clone();
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let inner = input_clone.clone();
        let cb = Closure::once(Box::new(move |_: web_sys::Event| {
            resolve
                .call1(&JsValue::NULL, &inner.files().and_then(|f| f.get(0)).into())
                .ok();
        }) as Box<dyn FnOnce(_)>);
        input_clone.set_onchange(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    });
    input.click();
    wasm_bindgen_futures::JsFuture::from(promise)
        .await?
        .dyn_into()
}

pub(super) fn blob_url_from_array_buffer(
    array_buffer: &JsValue,
    mime_type: &str,
) -> Result<String, JsValue> {
    let uint8 = js_sys::Uint8Array::new(array_buffer);
    let parts = js_sys::Array::new();
    parts.push(&uint8);
    let opts = web_sys::BlobPropertyBag::new();
    opts.set_type(mime_type);
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &opts)?;
    web_sys::Url::create_object_url_with_blob(&blob)
}

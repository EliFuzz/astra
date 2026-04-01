use wasm_bindgen::prelude::*;

pub(super) async fn decode_image_dimensions(blob_url: &str) -> Result<(u32, u32), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let img: web_sys::HtmlImageElement = document.create_element("img")?.dyn_into()?;

    let promise = wait_for_image_load(&img);
    img.set_src(blob_url);
    promise.await?;

    Ok((img.natural_width(), img.natural_height()))
}

async fn wait_for_image_load(img: &web_sys::HtmlImageElement) -> Result<(), JsValue> {
    use wasm_bindgen::closure::Closure;

    let img_clone = img.clone();
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let onload = Closure::once(Box::new(move |_: web_sys::Event| {
            resolve.call0(&JsValue::NULL).ok();
        }) as Box<dyn FnOnce(_)>);
        let onerror = Closure::once(Box::new(move |_: web_sys::Event| {
            reject
                .call1(&JsValue::NULL, &"Failed to load image".into())
                .ok();
        }) as Box<dyn FnOnce(_)>);
        img_clone.set_onload(Some(onload.as_ref().unchecked_ref()));
        img_clone.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onload.forget();
        onerror.forget();
    });

    wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(())
}

pub async fn yield_to_browser() {
    use wasm_bindgen::closure::Closure;

    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let window = web_sys::window().expect("no window");
        let closure = Closure::once_into_js(move || {
            let _ = resolve.call0(&JsValue::NULL);
        });
        let _ = window.request_animation_frame(closure.unchecked_ref());
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

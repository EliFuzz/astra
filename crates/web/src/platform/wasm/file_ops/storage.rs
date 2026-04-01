use astra_canvas::canvas::CanvasDocument;
use astra_render::{encode_png, start_png_readback};
use astra_storage::{IndexedDbStorage, Storage, UserPreferences, pending};
use std::future::Future;
use std::rc::Rc;
use std::sync::atomic::Ordering;

thread_local! {
    static STORAGE: Rc<IndexedDbStorage> = Rc::new(IndexedDbStorage::new());
}

fn spawn_storage<F, Fut>(run: F)
where
    F: FnOnce(Rc<IndexedDbStorage>) -> Fut + 'static,
    Fut: Future<Output = ()> + 'static,
{
    STORAGE.with(|s| {
        let storage = s.clone();
        wasm_bindgen_futures::spawn_local(run(storage));
    });
}

pub fn save_document(document: &CanvasDocument, name: &str) {
    let id = name.to_string();
    let doc = document.clone();
    spawn_storage(move |s| async move {
        match s.save(&id, &doc).await {
            Ok(()) => {
                log::info!("Document saved: {}", id);
                let _ = s.save("__last__", &doc).await;
            }
            Err(e) => log::error!("Failed to save document: {:?}", e),
        }
    });
}

pub fn autosave_document(document: &CanvasDocument) {
    let doc = document.clone();
    spawn_storage(move |s| async move {
        if let Err(e) = s.save("__last__", &doc).await {
            log::warn!("Autosave failed: {:?}", e);
        }
    });
}

pub fn load_document_async() {
    spawn_storage(|s| async move {
        match s.load("__last__").await {
            Ok(doc) => {
                log::info!("Document loaded: {}", doc.name);
                pending::set_pending_document(doc);
            }
            Err(_) => log::info!("No saved document, starting fresh"),
        }
    });
}

pub fn try_load_last_document() {
    load_document_async();
}

pub fn load_document_by_name_async(name: &str) {
    let name = name.to_string();
    spawn_storage(move |s| async move {
        match s.load(&name).await {
            Ok(doc) => {
                log::info!("Document loaded: {}", doc.name);
                pending::set_pending_document(doc);
            }
            Err(e) => log::error!("Failed to load '{}': {:?}", name, e),
        }
    });
}

pub fn list_documents_async() {
    spawn_storage(|s| async move {
        match s.list().await {
            Ok(docs) => {
                let filtered: Vec<String> = docs
                    .into_iter()
                    .filter(|id| id != "__last__" && id != astra_storage::preferences::PREFERENCES_KEY)
                    .collect();
                pending::set_pending_document_list(filtered);
            }
            Err(e) => log::error!("Failed to list documents: {:?}", e),
        }
    });
}

pub fn save_preferences(prefs: &UserPreferences) {
    let prefs = prefs.clone();
    spawn_storage(move |s| async move {
        if let Err(e) = s.save_preferences(&prefs).await {
            log::warn!("Failed to save preferences: {:?}", e);
        }
    });
}

pub fn try_load_preferences() {
    spawn_storage(|s| async move {
        match s.load_preferences().await {
            Ok(prefs) => pending::set_pending_preferences(prefs),
            Err(e) => log::warn!("Failed to load preferences: {:?}", e),
        }
    });
}

pub fn spawn_png_export_async(
    device: &vello::wgpu::Device,
    queue: &vello::wgpu::Queue,
    scene: vello::Scene,
    width: u32,
    height: u32,
    filename: String,
    is_copy: bool,
    scene_json: Option<String>,
) {
    if width == 0 || height == 0 {
        log::warn!("Cannot export empty scene");
        return;
    }

    let mut renderer = match vello::Renderer::new(device, vello::RendererOptions::default()) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to create renderer for export: {:?}", e);
            return;
        }
    };

    let (readback, mapped) =
        match start_png_readback(device, queue, &mut renderer, &scene, width, height) {
            Some(r) => r,
            None => return,
        };

    wasm_bindgen_futures::spawn_local(async move {
        let mut attempts = 0u32;
        loop {
            if mapped.load(Ordering::SeqCst) {
                break;
            }
            attempts += 1;
            if attempts >= 600 {
                log::error!("Timeout waiting for buffer mapping");
                return;
            }
            let _ = super::image::yield_to_browser().await;
        }

        let rgba_data = readback.into_rgba();
        let png_data = match encode_png(&rgba_data, width, height, scene_json.as_deref()) {
            Some(d) => d,
            None => {
                log::error!("Failed to encode PNG");
                return;
            }
        };

        if is_copy {
            super::clipboard::copy_png_to_clipboard(png_data);
        } else {
            super::download::export_png(&png_data, &filename.trim_end_matches(".png"));
            log::info!("PNG export complete: {} bytes", png_data.len());
        }
    });
}

pub fn spawn_tiled_png_export_async(
    device: &vello::wgpu::Device,
    queue: &vello::wgpu::Queue,
    export: astra_render::TiledExport,
    width: u32,
    height: u32,
    filename: String,
    is_copy: bool,
) {
    if width == 0 || height == 0 {
        log::warn!("Cannot export empty scene");
        return;
    }

    let device = device.clone();
    let queue = queue.clone();

    wasm_bindgen_futures::spawn_local(async move {
        match tiled_export_async(&device, &queue, &export, width, height).await {
            Some(png_data) => {
                if is_copy {
                    super::clipboard::copy_png_to_clipboard(png_data);
                } else {
                    super::download::export_png(&png_data, filename.trim_end_matches(".png"));
                    log::info!("Tiled PNG export complete: {} bytes", png_data.len());
                }
            }
            None => log::error!("Tiled PNG export failed"),
        }
    });
}

async fn tiled_export_async(
    device: &vello::wgpu::Device,
    queue: &vello::wgpu::Queue,
    export: &astra_render::TiledExport,
    width: u32,
    height: u32,
) -> Option<Vec<u8>> {
    use std::io::Write;

    let mut gpu_renderer =
        vello::Renderer::new(device, vello::RendererOptions::default()).ok()?;
    let mut shape_renderer = astra_render::VelloRenderer::new_for_export(export.scale);

    let max_tex = device.limits().max_texture_dimension_2d;
    let tile_w = width.min(max_tex);
    let tile_h = astra_render::STRIP_HEIGHT.min(max_tex).min(height);
    let row_bytes = (width as usize) * 4;

    log::info!(
        "WASM tiled export: {}x{} tiles ({}x{} each, image {}x{})",
        (width + tile_w - 1) / tile_w,
        (height + tile_h - 1) / tile_h,
        tile_w,
        tile_h,
        width,
        height,
    );

    let mut png_buf: Vec<u8> = Vec::new();
    {
        let mut encoder = png::Encoder::new(std::io::Cursor::new(&mut png_buf), width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Fast);
        encoder.set_filter(png::Filter::Sub);

        if export.should_embed_metadata() {
            if let Ok(json) = export.document.to_json_compact() {
                if let Err(e) = encoder.add_ztxt_chunk(
                    astra_render::PNG_SCENE_METADATA_KEY.to_string(),
                    json,
                ) {
                    log::warn!("Failed to add metadata chunk: {:?}", e);
                }
            }
        }

        let mut png_writer = encoder.write_header().ok()?;
        let mut stream = png_writer.stream_writer_with_size(row_bytes).ok()?;
        let mut row_buf = vec![0u8; row_bytes];

        let mut strip_y = 0u32;
        while strip_y < height {
            let current_strip_h = tile_h.min(height - strip_y);
            let mut tile_buffers: Vec<(Vec<u8>, u32)> = Vec::new();
            let mut tile_x = 0u32;

            while tile_x < width {
                let current_tile_w = tile_w.min(width - tile_x);

                let tile_scene = astra_render::build_tile_scene(
                    &mut shape_renderer,
                    export,
                    tile_x,
                    strip_y,
                    current_tile_w,
                    current_strip_h,
                );

                let readback = astra_render::prepare_readback(
                    device,
                    queue,
                    &mut gpu_renderer,
                    &tile_scene,
                    current_tile_w,
                    current_strip_h,
                )?;

                let mapped = readback.start_map_async();

                let mut attempts = 0u32;
                loop {
                    if mapped.load(Ordering::SeqCst) {
                        break;
                    }
                    attempts += 1;
                    if attempts >= 600 {
                        log::error!("Timeout waiting for tile buffer mapping");
                        return None;
                    }
                    let _ = super::image::yield_to_browser().await;
                }

                let rgba = readback.into_rgba();
                tile_buffers.push((rgba, current_tile_w));
                tile_x += tile_w;
            }

            for row in 0..current_strip_h {
                let mut offset = 0usize;
                for (rgba, tw) in &tile_buffers {
                    let tile_row_bytes = (*tw as usize) * 4;
                    let src_start = (row as usize) * tile_row_bytes;
                    row_buf[offset..offset + tile_row_bytes]
                        .copy_from_slice(&rgba[src_start..src_start + tile_row_bytes]);
                    offset += tile_row_bytes;
                }
                stream.write_all(&row_buf[..row_bytes]).ok()?;
            }

            strip_y += tile_h;
        }

        stream.finish().ok()?;
    }

    Some(png_buf)
}

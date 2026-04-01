use super::gpu::{
    build_tile_scene, prepare_readback, RenderBuffer, TiledExport, STRIP_HEIGHT,
};
use crate::vello_impl::{PngRenderResult, VelloRenderer};
use std::io::Write;
use vello::Scene;

pub fn render_scene_to_png(
    device: &vello::wgpu::Device,
    queue: &vello::wgpu::Queue,
    vello_renderer: &mut vello::Renderer,
    scene: &Scene,
    width: u32,
    height: u32,
) -> Option<PngRenderResult> {
    if width == 0 || height == 0 {
        return None;
    }

    let readback = prepare_readback(device, queue, vello_renderer, scene, width, height)?;
    let rgba_data = map_and_readback(device, readback)?;
    Some(PngRenderResult {
        rgba_data,
        width,
        height,
    })
}

fn map_and_readback(device: &vello::wgpu::Device, readback: RenderBuffer) -> Option<Vec<u8>> {
    let (tx, rx) = std::sync::mpsc::channel();
    readback
        .buffer
        .slice(..)
        .map_async(vello::wgpu::MapMode::Read, move |result| {
            tx.send(result).ok();
        });

    let _ = device.poll(vello::wgpu::PollType::wait_indefinitely());

    if rx.recv().ok()?.is_err() {
        log::error!("Failed to map buffer for PNG readback");
        return None;
    }

    Some(readback.into_rgba())
}

pub fn export_scene_to_png_file(
    device: &vello::wgpu::Device,
    queue: &vello::wgpu::Queue,
    export: &TiledExport,
    width: u32,
    height: u32,
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if width == 0 || height == 0 {
        return Err("Empty dimensions".into());
    }

    let file = std::fs::File::create(path)?;
    let buf_writer = std::io::BufWriter::with_capacity(1 << 20, file);

    let mut encoder = png::Encoder::new(buf_writer, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Fast);
    encoder.set_filter(png::Filter::Sub);

    if export.should_embed_metadata() {
        if let Ok(json) = export.document.to_json_compact() {
            if let Err(e) = encoder.add_ztxt_chunk(
                astra_core::png::PNG_SCENE_METADATA_KEY.to_string(),
                json,
            ) {
                log::warn!("Failed to add metadata chunk: {:?}", e);
            }
        }
    }

    let mut png_writer = encoder.write_header()?;
    let row_bytes = (width as usize) * 4;
    let mut stream = png_writer.stream_writer_with_size(row_bytes)?;

    let mut gpu_renderer = vello::Renderer::new(device, vello::RendererOptions::default())
        .map_err(|e| format!("Failed to create renderer: {:?}", e))?;

    let mut shape_renderer = VelloRenderer::new_for_export(export.scale);

    let max_tex = device.limits().max_texture_dimension_2d;
    let tile_w = width.min(max_tex);
    let tile_h = STRIP_HEIGHT.min(max_tex).min(height);
    let mut row_buf = vec![0u8; row_bytes];

    log::info!(
        "Tiled export: {}x{} tiles ({}x{} each, max_tex={})",
        (width + tile_w - 1) / tile_w,
        (height + tile_h - 1) / tile_h,
        tile_w,
        tile_h,
        max_tex,
    );

    let mut strip_y = 0u32;
    while strip_y < height {
        let current_strip_h = tile_h.min(height - strip_y);
        let mut tile_buffers: Vec<(Vec<u8>, u32)> = Vec::new();
        let mut tile_x = 0u32;

        while tile_x < width {
            let current_tile_w = tile_w.min(width - tile_x);

            let tile_scene = build_tile_scene(
                &mut shape_renderer,
                export,
                tile_x,
                strip_y,
                current_tile_w,
                current_strip_h,
            );

            let readback = prepare_readback(
                device,
                queue,
                &mut gpu_renderer,
                &tile_scene,
                current_tile_w,
                current_strip_h,
            )
            .ok_or_else(|| format!("Failed to render tile at ({}, {})", tile_x, strip_y))?;

            let rgba = map_and_readback(device, readback)
                .ok_or_else(|| format!("Failed to read back tile at ({}, {})", tile_x, strip_y))?;

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
            stream.write_all(&row_buf[..row_bytes])?;
        }

        strip_y += tile_h;
    }

    stream.finish()?;
    log::info!("Exported PNG to {:?} ({}x{})", path, width, height);
    Ok(())
}

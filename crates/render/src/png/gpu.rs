use crate::renderer::ShapeRenderer;
use crate::vello_impl::VelloRenderer;
use astra_canvas::canvas::CanvasDocument;
use astra_canvas::shapes::ShapeId;
use kurbo::{Affine, Rect};
use peniko::Color;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use vello::{AaConfig, RenderParams, Scene};

pub const STRIP_HEIGHT: u32 = 512;
const CULL_MARGIN: f64 = 100.0;
const MAX_SHAPES_FOR_METADATA: usize = 50_000;

pub struct RenderBuffer {
    pub(crate) buffer: vello::wgpu::Buffer,
    pub(crate) bytes_per_row: u32,
    pub width: u32,
    pub height: u32,
}

impl RenderBuffer {
    pub fn into_rgba(self) -> Vec<u8> {
        let slice = self.buffer.slice(..);
        let data = slice.get_mapped_range();
        let mut rgba = Vec::with_capacity((self.width * self.height * 4) as usize);
        for row in 0..self.height {
            let start = (row * self.bytes_per_row) as usize;
            let end = start + (self.width * 4) as usize;
            rgba.extend_from_slice(&data[start..end]);
        }
        drop(data);
        self.buffer.unmap();
        rgba
    }

    pub fn start_map_async(&self) -> Arc<AtomicBool> {
        let mapped = Arc::new(AtomicBool::new(false));
        let mapped_clone = mapped.clone();
        self.buffer
            .slice(..)
            .map_async(vello::wgpu::MapMode::Read, move |result| {
                if result.is_ok() {
                    mapped_clone.store(true, Ordering::SeqCst);
                } else {
                    log::error!("Buffer mapping failed");
                }
            });
        mapped
    }
}

pub fn prepare_readback(
    device: &vello::wgpu::Device,
    queue: &vello::wgpu::Queue,
    renderer: &mut vello::Renderer,
    scene: &Scene,
    width: u32,
    height: u32,
) -> Option<RenderBuffer> {
    let texture = device.create_texture(&vello::wgpu::TextureDescriptor {
        label: Some("png export texture"),
        size: vello::wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: vello::wgpu::TextureDimension::D2,
        format: vello::wgpu::TextureFormat::Rgba8Unorm,
        usage: vello::wgpu::TextureUsages::STORAGE_BINDING
            | vello::wgpu::TextureUsages::COPY_SRC
            | vello::wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let view = texture.create_view(&vello::wgpu::TextureViewDescriptor::default());

    let params = RenderParams {
        base_color: Color::WHITE,
        width,
        height,
        antialiasing_method: AaConfig::Area,
    };

    if let Err(e) = renderer.render_to_texture(device, queue, scene, &view, &params) {
        log::error!("Failed to render scene for PNG export: {:?}", e);
        return None;
    }

    let bytes_per_row = (width * 4).next_multiple_of(256);

    let buffer = device.create_buffer(&vello::wgpu::BufferDescriptor {
        label: Some("png readback buffer"),
        size: (bytes_per_row * height) as u64,
        usage: vello::wgpu::BufferUsages::COPY_DST | vello::wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&vello::wgpu::CommandEncoderDescriptor {
        label: Some("png copy encoder"),
    });

    encoder.copy_texture_to_buffer(
        vello::wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: vello::wgpu::Origin3d::ZERO,
            aspect: vello::wgpu::TextureAspect::All,
        },
        vello::wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: vello::wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        vello::wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    queue.submit(std::iter::once(encoder.finish()));

    Some(RenderBuffer {
        buffer,
        bytes_per_row,
        width,
        height,
    })
}

pub struct TiledExport {
    pub document: CanvasDocument,
    pub padded_bounds: Rect,
    pub scale: f64,
    pub selection: Option<Vec<ShapeId>>,
}

impl TiledExport {
    pub fn should_embed_metadata(&self) -> bool {
        self.document.shapes.len() <= MAX_SHAPES_FOR_METADATA
    }
}

pub fn build_tile_scene(
    renderer: &mut VelloRenderer,
    export: &TiledExport,
    tile_x: u32,
    tile_y: u32,
    tile_w: u32,
    tile_h: u32,
) -> Scene {
    renderer.scene.reset();

    let bg = Rect::new(0.0, 0.0, tile_w as f64, tile_h as f64);
    renderer.scene.fill(
        peniko::Fill::NonZero,
        Affine::IDENTITY,
        Color::WHITE,
        None,
        &bg,
    );

    let transform = Affine::translate((-f64::from(tile_x), -f64::from(tile_y)))
        * Affine::scale(export.scale)
        * Affine::translate((-export.padded_bounds.x0, -export.padded_bounds.y0));

    let inv_scale = 1.0 / export.scale;
    let doc_viewport = Rect::new(
        export.padded_bounds.x0 + f64::from(tile_x) * inv_scale,
        export.padded_bounds.y0 + f64::from(tile_y) * inv_scale,
        export.padded_bounds.x0 + f64::from(tile_x + tile_w) * inv_scale,
        export.padded_bounds.y0 + f64::from(tile_y + tile_h) * inv_scale,
    )
    .inflate(CULL_MARGIN, CULL_MARGIN);

    match export.selection.as_deref() {
        None => {
            for shape in export.document.visible_shapes_ordered(doc_viewport) {
                ShapeRenderer::render_shape(renderer, shape, transform, false);
            }
        }
        Some(ids) => {
            let visible: HashSet<ShapeId> = export
                .document
                .spatial_index
                .query_rect(doc_viewport)
                .into_iter()
                .collect();
            for &id in ids {
                if !visible.contains(&id) {
                    continue;
                }
                if let Some(shape) = export.document.get_shape(id) {
                    ShapeRenderer::render_shape(renderer, shape, transform, false);
                }
            }
        }
    }

    std::mem::take(&mut renderer.scene)
}

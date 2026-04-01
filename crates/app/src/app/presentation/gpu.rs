use super::super::AppState;
use vello::{AaConfig, RenderParams};

pub fn present_frame(
    state: &mut AppState,
    render_cx: &vello::util::RenderContext,
    scene: vello::Scene,
    primitives: Vec<egui::ClippedPrimitive>,
    textures_delta: egui::TexturesDelta,
    pixels_per_point: f32,
    action_taken: bool,
) {
    let device_handle = &render_cx.devices[state.surface.dev_id];
    let device = &device_handle.device;
    let queue = &device_handle.queue;

    let surface_texture = match state.surface.surface.get_current_texture() {
        Ok(t) => t,
        Err(e) => {
            log::warn!("Failed to get surface texture: {:?}", e);
            return;
        }
    };

    let width = state.surface.config.width;
    let height = state.surface.config.height;

    let params = RenderParams {
        base_color: state.config.background_color,
        width,
        height,
        antialiasing_method: AaConfig::Area,
    };

    let render_texture = match state.render_texture.as_ref() {
        Some((tex, w, h)) if *w == width && *h == height => tex,
        _ => {
            let new_tex = device.create_texture(&vello::wgpu::TextureDescriptor {
                label: Some("render texture"),
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
            state.render_texture = Some((new_tex, width, height));
            state.render_texture.as_ref().map(|(t, _, _)| t).unwrap()
        }
    };

    let render_texture_view =
        render_texture.create_view(&vello::wgpu::TextureViewDescriptor::default());

    if let Err(e) =
        state
            .vello_renderer
            .render_to_texture(device, queue, &scene, &render_texture_view, &params)
    {
        log::error!("Failed to render: {:?}", e);
        return;
    }

    let surface_view = surface_texture
        .texture
        .create_view(&vello::wgpu::TextureViewDescriptor::default());

    {
        let mut blit_encoder =
            device.create_command_encoder(&vello::wgpu::CommandEncoderDescriptor {
                label: Some("blit encoder"),
            });

        state.texture_blitter.copy(
            device,
            &mut blit_encoder,
            &render_texture_view,
            &surface_view,
        );

        queue.submit(std::iter::once(blit_encoder.finish()));
    }

    super::egui_pass::render(
        state,
        super::egui_pass::EguiPassSurface {
            device,
            queue,
            surface_texture,
            surface_view,
            width,
            height,
        },
        &primitives,
        &textures_delta,
        pixels_per_point,
        action_taken,
    );
}

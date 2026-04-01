use super::super::AppState;

pub struct EguiPassSurface<'a> {
    pub device: &'a vello::wgpu::Device,
    pub queue: &'a vello::wgpu::Queue,
    pub surface_texture: vello::wgpu::SurfaceTexture,
    pub surface_view: vello::wgpu::TextureView,
    pub width: u32,
    pub height: u32,
}

pub fn render(
    state: &mut AppState,
    surface: EguiPassSurface<'_>,
    primitives: &[egui::ClippedPrimitive],
    textures_delta: &egui::TexturesDelta,
    pixels_per_point: f32,
    action_taken: bool,
) {
    let EguiPassSurface {
        device,
        queue,
        surface_texture,
        surface_view,
        width,
        height,
    } = surface;

    for (id, image_delta) in &textures_delta.set {
        state
            .egui_renderer
            .update_texture(device, queue, *id, image_delta);
    }

    let screen_descriptor = egui_wgpu::ScreenDescriptor {
        size_in_pixels: [width, height],
        pixels_per_point,
    };

    {
        let mut egui_encoder =
            device.create_command_encoder(&vello::wgpu::CommandEncoderDescriptor {
                label: Some("egui encoder"),
            });

        state.egui_renderer.update_buffers(
            device,
            queue,
            &mut egui_encoder,
            primitives,
            &screen_descriptor,
        );

        let render_pass = egui_encoder.begin_render_pass(&vello::wgpu::RenderPassDescriptor {
            label: Some("egui render pass"),
            color_attachments: &[Some(vello::wgpu::RenderPassColorAttachment {
                view: &surface_view,
                resolve_target: None,
                ops: vello::wgpu::Operations {
                    load: vello::wgpu::LoadOp::Load,
                    store: vello::wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        let mut render_pass = render_pass.forget_lifetime();
        state
            .egui_renderer
            .render(&mut render_pass, primitives, &screen_descriptor);
        drop(render_pass);

        queue.submit(std::iter::once(egui_encoder.finish()));
    }

    for id in &textures_delta.free {
        state.egui_renderer.free_texture(id);
    }
    surface_texture.present();

    if state.needs_redraw || action_taken || state.egui_ctx.has_requested_repaint() {
        state.needs_redraw = false;
        state.window.request_redraw();
    }
}

use super::gpu::{RenderBuffer, prepare_readback};
use std::sync::{Arc, atomic::AtomicBool};
use vello::Scene;

pub fn start_png_readback(
    device: &vello::wgpu::Device,
    queue: &vello::wgpu::Queue,
    renderer: &mut vello::Renderer,
    scene: &Scene,
    width: u32,
    height: u32,
) -> Option<(RenderBuffer, Arc<AtomicBool>)> {
    let readback = prepare_readback(device, queue, renderer, scene, width, height)?;
    let mapped = readback.start_map_async();
    Some((readback, mapped))
}

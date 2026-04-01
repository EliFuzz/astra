use crate::renderer::{RenderContext, Renderer};
use crate::vello_impl::VelloRenderer;
use astra_canvas::canvas::Canvas;
use astra_canvas::shapes::Rectangle;
use kurbo::Point;

#[test]
fn renderer_new_has_empty_scene_encoding() {
    let renderer = VelloRenderer::new();
    assert!(renderer.scene().encoding().is_empty());
}

#[test]
fn build_scene_with_empty_canvas_completes() {
    let mut renderer = VelloRenderer::new();
    let canvas = Canvas::new();
    let ctx = RenderContext::new(&canvas, kurbo::Size::new(800.0, 600.0));
    renderer.build_scene(&ctx);
}

#[test]
fn build_scene_with_rectangle_completes() {
    let mut renderer = VelloRenderer::new();
    let mut canvas = Canvas::new();
    let rect = Rectangle::new(Point::new(100.0, 100.0), 200.0, 150.0);
    canvas
        .document
        .add_shape(astra_canvas::shapes::Shape::Rectangle(rect));
    let ctx = RenderContext::new(&canvas, kurbo::Size::new(800.0, 600.0));
    renderer.build_scene(&ctx);
}

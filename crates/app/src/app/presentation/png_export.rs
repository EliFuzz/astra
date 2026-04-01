use super::super::AppState;
use super::super::platform;

const EXPORT_PADDING: f64 = 20.0;

pub(super) fn apply_png_export(
    state: &mut AppState,
    render_cx: &vello::util::RenderContext,
    is_copy: bool,
) {
    if is_copy {
        apply_copy(state, render_cx);
    } else {
        apply_file_export(state, render_cx);
    }
}

fn apply_copy(state: &mut AppState, render_cx: &vello::util::RenderContext) {
    let (scene, bounds) = build_copy_scene(state);
    let Some(bounds) = bounds else {
        log::info!("Nothing to export");
        return;
    };
    let width = bounds.width().ceil() as u32;
    let height = bounds.height().ceil() as u32;

    log::info!(
        "Copying PNG at {}x scale: {}x{}",
        state.ui_state.export_scale,
        width,
        height,
    );

    platform::export_png(state, render_cx, scene, width, height);
}

fn apply_file_export(state: &mut AppState, render_cx: &vello::util::RenderContext) {
    let export_scale = state.ui_state.export_scale as f64;
    let (bounds, selection) = compute_export_bounds(state);
    let Some(bounds) = bounds else {
        log::info!("Nothing to export");
        return;
    };

    let padded_bounds = bounds.inflate(EXPORT_PADDING, EXPORT_PADDING);
    let width = (padded_bounds.width() * export_scale).ceil() as u32;
    let height = (padded_bounds.height() * export_scale).ceil() as u32;

    log::info!(
        "Exporting PNG at {}x scale: {}x{}",
        state.ui_state.export_scale,
        width,
        height,
    );

    platform::export_png_to_file(
        state,
        render_cx,
        width,
        height,
        padded_bounds,
        export_scale,
        selection,
    );
}

fn compute_export_bounds(
    state: &AppState,
) -> (Option<kurbo::Rect>, Option<Vec<astra_canvas::shapes::ShapeId>>) {
    if state.canvas.selection.is_empty() {
        (state.canvas.document.bounds(), None)
    } else {
        let bounds = selection_bounds(&state.canvas.document, &state.canvas.selection);
        (bounds, Some(state.canvas.selection.clone()))
    }
}

fn selection_bounds(
    document: &astra_canvas::canvas::CanvasDocument,
    ids: &[astra_canvas::shapes::ShapeId],
) -> Option<kurbo::Rect> {
    let mut result: Option<kurbo::Rect> = None;
    for &id in ids {
        if let Some(shape) = document.get_shape(id) {
            let b = shape.bounds();
            result = Some(match result {
                Some(r) => r.union(b),
                None => b,
            });
        }
    }
    result
}

fn build_copy_scene(state: &mut AppState) -> (vello::Scene, Option<kurbo::Rect>) {
    let export_scale = state.ui_state.export_scale as f64;
    if state.canvas.selection.is_empty() {
        state
            .shape_renderer
            .build_export_scene(&state.canvas.document, export_scale)
    } else {
        state.shape_renderer.build_export_scene_selection(
            &state.canvas.document,
            &state.canvas.selection,
            export_scale,
        )
    }
}

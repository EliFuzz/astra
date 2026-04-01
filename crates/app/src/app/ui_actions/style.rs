use super::super::AppState;
use crate::ui::UiAction;
use astra_canvas::shapes::{FillPattern, PathStyle, Shape, Sloppiness, StrokeStyle};

pub(super) fn apply(
    state: &mut AppState,
    action: &UiAction,
    text_selection_state: &Option<(astra_canvas::shapes::ShapeId, std::ops::Range<usize>)>,
) {
    match action {
        UiAction::SetStrokeColor(color) => {
            state.ui_state.stroke_color = *color;
            let mut applied_to_text_range = false;
            if let Some((text_id, byte_range)) = text_selection_state {
                if let Some(mut guard) = state.canvas.document.get_shape_mut(*text_id) {
                    if let Shape::Text(text) = &mut *guard {
                        let start_char = text.content[..byte_range.start].chars().count();
                        let end_char = text.content[..byte_range.end].chars().count();
                        let style = state.ui_state.to_shape_style();
                        text.apply_color_to_range(start_char, end_char, style.stroke_color);
                        applied_to_text_range = true;
                    }
                }
            }
            if !applied_to_text_range {
                let stroke_color = state.ui_state.to_shape_style().stroke_color;
                for &id in &state.canvas.selection.clone() {
                    apply_to_shape_or_children(&mut state.canvas.document, id, &|shape| {
                        shape.style_mut().stroke_color = stroke_color;
                        if let Shape::Text(text) = shape {
                            let char_count = text.content.chars().count();
                            text.apply_color_to_range(0, char_count, stroke_color);
                        }
                    });
                }
            }
        }
        UiAction::SetFillColor(color) => {
            state.ui_state.fill_color = *color;
            let fill = state.ui_state.to_shape_style().fill_color;
            for &id in &state.canvas.selection.clone() {
                apply_to_shape_or_children(&mut state.canvas.document, id, &|shape| {
                    if !matches!(shape, Shape::Text(_)) {
                        shape.style_mut().fill_color = fill;
                    }
                });
            }
        }
        UiAction::SetStrokeWidth(width) => {
            state.ui_state.stroke_width = *width;
            let w = *width as f64;
            for &id in &state.canvas.selection.clone() {
                apply_to_shape_or_children(&mut state.canvas.document, id, &|shape| {
                    if !matches!(shape, Shape::Text(_)) {
                        shape.style_mut().stroke_width = w;
                    }
                });
            }
        }
        UiAction::SetCornerRadius(radius) => {
            state.ui_state.corner_radius = *radius;
            let r = *radius as f64;
            for &id in &state.canvas.selection.clone() {
                apply_to_shape_or_children(&mut state.canvas.document, id, &|shape| {
                    match shape {
                        Shape::Rectangle(rect) => rect.corner_radius = r,
                        Shape::Diamond(diamond) => diamond.corner_radius = r,
                        Shape::Image(image) => image.corner_radius = r,
                        _ => {}
                    }
                });
            }
        }
        UiAction::SetSloppiness(level) => {
            let sloppiness = match level {
                0 => Sloppiness::Architect,
                1 => Sloppiness::Artist,
                2 => Sloppiness::Cartoonist,
                _ => Sloppiness::Drunk,
            };
            state.ui_state.sloppiness = sloppiness;
            for &id in &state.canvas.selection.clone() {
                apply_to_shape_or_children(&mut state.canvas.document, id, &|shape| {
                    shape.style_mut().sloppiness = sloppiness;
                });
            }
        }
        UiAction::SetFillPattern(level) => {
            let fill_pattern = match level {
                0 => FillPattern::Solid,
                1 => FillPattern::Hachure,
                2 => FillPattern::ZigZag,
                3 => FillPattern::CrossHatch,
                4 => FillPattern::Dots,
                5 => FillPattern::Dashed,
                _ => FillPattern::ZigZagLine,
            };
            state.ui_state.fill_pattern = fill_pattern;
            for &id in &state.canvas.selection.clone() {
                apply_to_shape_or_children(&mut state.canvas.document, id, &|shape| {
                    shape.style_mut().fill_pattern = fill_pattern;
                });
            }
        }
        UiAction::SetPathStyle(level) => {
            let path_style = match level {
                0 => PathStyle::Direct,
                1 => PathStyle::Flowing,
                _ => PathStyle::Angular,
            };
            state.ui_state.path_style = *level;
            for &id in &state.canvas.selection.clone() {
                apply_to_shape_or_children(&mut state.canvas.document, id, &|shape| {
                    match shape {
                        Shape::Line(line) => {
                            line.path_style = path_style;
                            if path_style == PathStyle::Angular {
                                line.intermediate_points.clear();
                            }
                        }
                        Shape::Arrow(arrow) => {
                            arrow.path_style = path_style;
                            if path_style == PathStyle::Angular {
                                arrow.intermediate_points.clear();
                            }
                        }
                        _ => {}
                    }
                });
            }
        }
        UiAction::SetStrokeStyle(level) => {
            let stroke_style = match level {
                0 => StrokeStyle::Solid,
                1 => StrokeStyle::Dashed,
                _ => StrokeStyle::Dotted,
            };
            for &id in &state.canvas.selection.clone() {
                apply_to_shape_or_children(&mut state.canvas.document, id, &|shape| {
                    match shape {
                        Shape::Rectangle(rect) => rect.stroke_style = stroke_style,
                        Shape::Diamond(diamond) => diamond.stroke_style = stroke_style,
                        Shape::Ellipse(ellipse) => ellipse.stroke_style = stroke_style,
                        Shape::Line(line) => line.stroke_style = stroke_style,
                        Shape::Arrow(arrow) => arrow.stroke_style = stroke_style,
                        _ => {}
                    }
                });
            }
        }
        UiAction::SetOpacity(opacity) => {
            if !state.canvas.selection.is_empty() {
                state.canvas.document.push_undo();
                let op = *opacity as f64;
                for &id in &state.canvas.selection.clone() {
                    apply_to_shape_or_children(&mut state.canvas.document, id, &|shape| {
                        shape.style_mut().opacity = op;
                        if let Shape::Text(text) = shape {
                            text.invalidate_cache();
                        }
                    });
                }
            }
        }
        UiAction::SetFontSize(size) => {
            state.ui_state.font_size = *size;
            let s = *size as f64;
            for &id in &state.canvas.selection.clone() {
                apply_to_shape_or_children(&mut state.canvas.document, id, &|shape| {
                    if let Shape::Text(text) = shape {
                        text.font_size = s;
                        text.invalidate_cache();
                    }
                });
                fit_group_text(&mut state.canvas.document, id);
            }
        }
        _ => {}
    }
}

fn apply_to_shape_or_children(
    doc: &mut astra_canvas::canvas::CanvasDocument,
    id: astra_canvas::shapes::ShapeId,
    f: &dyn Fn(&mut Shape),
) {
    if let Some(mut guard) = doc.get_shape_mut(id) {
        if let Shape::Group(group) = &mut *guard {
            for child in group.children.iter_mut() {
                f(child);
            }
        } else {
            f(&mut guard);
        }
    }
}

fn fit_group_text(
    doc: &mut astra_canvas::canvas::CanvasDocument,
    id: astra_canvas::shapes::ShapeId,
) {
    if let Some(mut guard) = doc.get_shape_mut(id) {
        if let Shape::Group(group) = &mut *guard {
            group.fit_and_center_text_children();
        }
    }
}

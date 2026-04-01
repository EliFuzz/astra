use super::super::{ColorPopover, SelectedShapeProps, UiAction, UiState};
use super::shape_sections::{
    render_align_section, render_edges_section, render_layer_section, render_opacity_section,
    render_path_style_section, render_text_section,
};
use super::style_sections::{
    render_fill_section, render_sloppiness_section, render_stroke_section,
    render_stroke_style_section, render_stroke_width_section,
};
use crate::ColorGrid;
use egui::{Align2, Color32, Context, Margin, Rect, Vec2};

pub fn render_properties_panel(
    ctx: &Context,
    ui_state: &mut UiState,
    selected_props: &SelectedShapeProps,
) -> Option<UiAction> {
    let show_panel = selected_props.has_selection
        || selected_props.is_drawing_tool
        || ui_state.current_tool == astra_canvas::tools::ToolKind::Rectangle
        || ui_state.current_tool == astra_canvas::tools::ToolKind::Diamond
        || ui_state.current_tool == astra_canvas::tools::ToolKind::Ellipse
        || ui_state.current_tool == astra_canvas::tools::ToolKind::Line
        || ui_state.current_tool == astra_canvas::tools::ToolKind::Arrow
        || ui_state.current_tool == astra_canvas::tools::ToolKind::Freehand
        || ui_state.current_tool == astra_canvas::tools::ToolKind::Highlighter
        || ui_state.current_tool == astra_canvas::tools::ToolKind::Text;

    if !show_panel {
        return None;
    }

    let mut action = None;
    let mut stroke_current_rect = Rect::NOTHING;
    let mut fill_current_rect = Rect::NOTHING;

    egui::Area::new(egui::Id::new("properties"))
        .anchor(Align2::LEFT_CENTER, Vec2::new(12.0, 0.0))
        .interactable(true)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            crate::panel_frame().inner_margin(Margin::symmetric(10, 8)).show(ui, |ui| {
                ui.set_width(150.0);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(0.0, 6.0);

                    if selected_props.shows_stroke_color() {
                        render_stroke_section(ui, ui_state, selected_props, &mut action, &mut stroke_current_rect);
                    }

                    if selected_props.shows_fill_color() {
                        ui.add_space(2.0);
                        render_fill_section(ui, ui_state, &mut action, &mut fill_current_rect);
                    }

                    if selected_props.shows_stroke_width() {
                        ui.add_space(2.0);
                        render_stroke_width_section(ui, ui_state, &mut action);
                    }

                    if selected_props.shows_stroke_style() {
                        ui.add_space(2.0);
                        render_stroke_style_section(ui, selected_props, &mut action);
                    }

                    if selected_props.shows_sloppiness() {
                        ui.add_space(2.0);
                        render_sloppiness_section(ui, selected_props, &mut action);
                    }

                    if selected_props.shows_edges() {
                        ui.add_space(2.0);
                        render_edges_section(ui, selected_props, &mut action);
                    }

                    if selected_props.shows_path_style() {
                        ui.add_space(2.0);
                        render_path_style_section(ui, selected_props, &mut action);
                    }

                    if selected_props.is_text {
                        ui.add_space(2.0);
                        render_text_section(ui, ui_state, selected_props, &mut action);
                    }

                    ui.add_space(2.0);
                    render_opacity_section(ui, selected_props, &mut action);

                    if selected_props.has_selection && !selected_props.is_drawing_tool {
                        ui.add_space(2.0);
                        render_layer_section(ui, &mut action);

                        if selected_props.selection_count >= 2 {
                            ui.add_space(2.0);
                            render_align_section(ui, &mut action);
                        }
                    }
                });
            });
        });

    render_color_popovers(ctx, ui_state, &mut action, stroke_current_rect, fill_current_rect);

    action
}

fn render_color_popovers(
    ctx: &Context,
    ui_state: &mut UiState,
    action: &mut Option<UiAction>,
    stroke_current_rect: Rect,
    fill_current_rect: Rect,
) {
    match ui_state.color_popover {
        ColorPopover::StrokeFull => {
            let result = ColorGrid::new(ui_state.stroke_color, "Stroke Color")
                .below()
                .show(ctx, stroke_current_rect);
            if let Some(selected_color) = result {
                *action = Some(UiAction::SetStrokeColor(selected_color));
                ui_state.color_popover = ColorPopover::None;
            } else if ctx.input(|i| i.pointer.any_pressed()) {
                let grid_id = egui::Id::new("color_grid");
                let covers_pointer = ctx.layer_id_at(ctx.input(|i| i.pointer.interact_pos().unwrap_or_default()))
                    .map(|layer| layer.id == grid_id)
                    .unwrap_or(false);
                if !covers_pointer && !stroke_current_rect.contains(ctx.input(|i| i.pointer.interact_pos().unwrap_or_default())) {
                    ui_state.color_popover = ColorPopover::None;
                }
            }
        }
        ColorPopover::FillFull => {
            let result = ColorGrid::new(
                ui_state.fill_color.unwrap_or(Color32::TRANSPARENT),
                "Fill Color",
            )
            .below()
            .show(ctx, fill_current_rect);
            if let Some(selected_color) = result {
                *action = Some(UiAction::SetFillColor(Some(selected_color)));
                ui_state.color_popover = ColorPopover::None;
            } else if ctx.input(|i| i.pointer.any_pressed()) {
                let grid_id = egui::Id::new("color_grid");
                let covers_pointer = ctx.layer_id_at(ctx.input(|i| i.pointer.interact_pos().unwrap_or_default()))
                    .map(|layer| layer.id == grid_id)
                    .unwrap_or(false);
                if !covers_pointer && !fill_current_rect.contains(ctx.input(|i| i.pointer.interact_pos().unwrap_or_default())) {
                    ui_state.color_popover = ColorPopover::None;
                }
            }
        }
        ColorPopover::None | ColorPopover::BgFull => {}
    }
}

use super::grid_style::grid_style_button;
use super::super::color_helpers::color_swatch_current;
use super::super::{ColorPopover, UiAction, UiState};
use crate::{ColorGrid, IconButton};
use crate::theme;
use astra_render::GridStyle;
use crate::icon;
use egui::{Context, CursorIcon, Rect, Sense, Stroke, Vec2, vec2};

pub(super) fn render_history_section(ui: &mut egui::Ui) -> Option<UiAction> {
    let mut action = None;

    if IconButton::new(
        icon!("undo.png"),
        "Undo (Ctrl+Z)",
    )
    .small()
    .show(ui)
    {
        action = Some(UiAction::Undo);
    }

    ui.add_space(2.0);

    if IconButton::new(
        icon!("redo.png"),
        "Redo (Ctrl+Shift+Z)",
    )
    .small()
    .show(ui)
    {
        action = Some(UiAction::Redo);
    }

    action
}

pub(super) fn render_grid_and_bg_section(
    ui: &mut egui::Ui,
    state: &mut UiState,
) -> (Option<UiAction>, Rect) {
    let mut action = None;

    let grid_tooltip = match state.grid_style {
        GridStyle::None => "No grid",
        GridStyle::Lines => "Grid",
        GridStyle::HorizontalLines => "Lines",
        GridStyle::CrossPlus => "Crosses",
        GridStyle::Dots => "Dots",
    };

    if grid_style_button(ui, state.grid_style, grid_tooltip) {
        action = Some(UiAction::ToggleGrid);
    }

    ui.add_space(4.0);

    let (clicked, rect) = color_swatch_current(ui, state.bg_color, "Background color");
    if clicked {
        state.color_popover = if state.color_popover == ColorPopover::BgFull {
            ColorPopover::None
        } else {
            ColorPopover::BgFull
        };
    }

    ui.add_space(4.0);

    if render_theme_toggle(ui) {
        action = Some(UiAction::ToggleTheme);
    }

    (action, rect)
}

fn render_theme_toggle(ui: &mut egui::Ui) -> bool {
    let is_dark = theme::is_dark();
    let size = vec2(24.0, 24.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if ui.is_rect_visible(rect) {
        let icon_color = if response.hovered() {
            theme::icon_hover_color()
        } else {
            theme::icon_color()
        };

        let center = rect.center();
        let r = 6.0;

        if is_dark {
            ui.painter().circle_stroke(center, r, Stroke::new(1.5, icon_color));
            let inner_r = r * 0.65;
            let offset = Vec2::new(r * 0.35, -r * 0.35);
            ui.painter().circle_filled(center + offset, inner_r, theme::panel_bg());
        } else {
            ui.painter().circle_filled(center, r * 0.55, icon_color);
            let ray_len = 3.0;
            let ray_dist = r * 0.85;
            for i in 0..8 {
                let angle = std::f32::consts::TAU * (i as f32) / 8.0;
                let dir = Vec2::new(angle.cos(), angle.sin());
                let start = center + dir * ray_dist;
                let end = center + dir * (ray_dist + ray_len);
                ui.painter().line_segment([start, end], Stroke::new(1.5, icon_color));
            }
        }
    }

    let tooltip = if is_dark { "Light Theme" } else { "Dark Theme" };
    let clicked = response.clicked();
    response.on_hover_text(tooltip).on_hover_cursor(CursorIcon::PointingHand);
    clicked
}

pub(super) fn render_snap_section(ui: &mut egui::Ui, state: &UiState) -> Option<UiAction> {
    let mut action = None;

    let grid_snap_tooltip = if state.grid_snap_enabled {
        "Grid Snap: On"
    } else {
        "Grid Snap: Off"
    };
    if IconButton::new(
        icon!("snap-grid.png"),
        grid_snap_tooltip,
    )
    .small()
    .selected(state.grid_snap_enabled)
    .show(ui)
    {
        action = Some(UiAction::ToggleGridSnap);
    }

    ui.add_space(4.0);

    let smart_snap_tooltip = if state.smart_snap_enabled {
        "Smart Guides: On"
    } else {
        "Smart Guides: Off"
    };
    if IconButton::new(
        icon!("snap-shapes.png"),
        smart_snap_tooltip,
    )
    .small()
    .selected(state.smart_snap_enabled)
    .show(ui)
    {
        action = Some(UiAction::ToggleSmartSnap);
    }

    ui.add_space(4.0);

    let angle_snap_tooltip = if state.angle_snap_enabled {
        "Angle Snap: On (15°)"
    } else {
        "Angle Snap: Off"
    };
    if IconButton::new(
        icon!("angle.png"),
        angle_snap_tooltip,
    )
    .small()
    .selected(state.angle_snap_enabled)
    .show(ui)
    {
        action = Some(UiAction::ToggleAngleSnap);
    }

    action
}

pub(super) fn render_bg_color_popover(
    ctx: &Context,
    state: &mut UiState,
    bg_rect: Rect,
) -> Option<UiAction> {
    if state.color_popover != ColorPopover::BgFull {
        return None;
    }

    let result = ColorGrid::new(state.bg_color, "")
        .above()
        .show(ctx, bg_rect);
    if let Some(selected_color) = result {
        state.color_popover = ColorPopover::None;
        return Some(UiAction::SetBgColor(selected_color));
    }

    if ctx.input(|i| i.pointer.any_pressed()) {
        let grid_id = egui::Id::new("color_grid");
        let covers_pointer = ctx.layer_id_at(ctx.input(|i| i.pointer.interact_pos().unwrap_or_default()))
            .map(|layer| layer.id == grid_id)
            .unwrap_or(false);
        if !covers_pointer && !bg_rect.contains(ctx.input(|i| i.pointer.interact_pos().unwrap_or_default())) {
            state.color_popover = ColorPopover::None;
        }
    }

    None
}

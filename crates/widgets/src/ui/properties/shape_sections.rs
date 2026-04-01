use super::super::{SelectedShapeProps, UiAction, UiState};
use crate::{IconButton, ToggleButton, section_label as widgets_section_label};
use crate::theme;
use crate::icon;
use egui::Vec2;

pub(super) fn render_edges_section(
    ui: &mut egui::Ui,
    props: &SelectedShapeProps,
    action: &mut Option<UiAction>,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 4.0);
        widgets_section_label(ui, "Edges");
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);

            let is_sharp = props.corner_radius < 1.0;
            if IconButton::new(
                icon!("edge-sharp.png"),
                "Sharp",
            ).small().selected(is_sharp).show(ui) && !is_sharp {
                *action = Some(UiAction::SetCornerRadius(0.0));
            }

            let is_rounded = props.corner_radius >= 1.0;
            if IconButton::new(
                icon!("edge-round.png"),
                "Round",
            ).small().selected(is_rounded).show(ui) && !is_rounded {
                *action = Some(UiAction::SetCornerRadius(32.0));
            }
        });
    });
}

pub(super) fn render_path_style_section(
    ui: &mut egui::Ui,
    props: &SelectedShapeProps,
    action: &mut Option<UiAction>,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 4.0);
        widgets_section_label(ui, "Arrow type");
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);

            let styles = [
                (0u8, "Direct", icon!("arrow.png")),
                (1, "Flowing", icon!("arrow-curved.png")),
                (2, "Angular", icon!("arrow-elbow.png")),
            ];
            for (idx, name, icon) in styles {
                let is_selected = props.path_style == idx;
                if IconButton::new(icon, name).small().selected(is_selected).show(ui) && !is_selected {
                    *action = Some(UiAction::SetPathStyle(idx));
                }
            }
        });
    });
}

pub(super) fn render_text_section(
    ui: &mut egui::Ui,
    ui_state: &UiState,
    props: &SelectedShapeProps,
    action: &mut Option<UiAction>,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 4.0);
        widgets_section_label(ui, "Font size");
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
            let sizes: &[(f32, &str)] = &[
                (16.0, "S"),
                (24.0, "M"),
                (36.0, "L"),
                (48.0, "XL"),
            ];
            let current_size = if props.has_selection { props.font_size } else { ui_state.font_size };
            for &(size, name) in sizes {
                let is_selected = (current_size - size).abs() < 0.5;
                if ToggleButton::new(name, is_selected).show(ui) && !is_selected {
                    *action = Some(UiAction::SetFontSize(size));
                }
            }
        });
    });
}

pub(super) fn render_opacity_section(
    ui: &mut egui::Ui,
    props: &SelectedShapeProps,
    action: &mut Option<UiAction>,
) {
    let label_color = theme::section_label_color();

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 4.0);
        ui.label(
            egui::RichText::new("Opacity")
                .size(10.0)
                .color(label_color),
        );
        ui.horizontal(|ui| {
            let mut opacity = props.opacity;
            let slider = egui::Slider::new(&mut opacity, 0.0..=1.0)
                .show_value(false)
                .custom_formatter(|v, _| format!("{}%", (v * 100.0) as i32));
            if ui.add(slider).changed() {
                *action = Some(UiAction::SetOpacity(opacity));
            }
            ui.label(
                egui::RichText::new(format!("{}%", (props.opacity * 100.0) as i32))
                    .size(11.0)
                    .color(label_color),
            );
        });
    });
}

pub(super) fn render_layer_section(
    ui: &mut egui::Ui,
    action: &mut Option<UiAction>,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 4.0);
        widgets_section_label(ui, "Layers");
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);

            if IconButton::new(
                icon!("layer-front.png"),
                "Bring to Front",
            )
            .small()
            .show(ui)
            {
                *action = Some(UiAction::BringToFront);
            }
            if IconButton::new(
                icon!("layer-back.png"),
                "Send to Back",
            )
            .small()
            .show(ui)
            {
                *action = Some(UiAction::SendToBack);
            }
            if IconButton::new(
                icon!("layer-forward.png"),
                "Bring Forward",
            )
            .small()
            .show(ui)
            {
                *action = Some(UiAction::BringForward);
            }
            if IconButton::new(
                icon!("layer-backward.png"),
                "Send Backward",
            )
            .small()
            .show(ui)
            {
                *action = Some(UiAction::SendBackward);
            }
        });
    });
}

pub(super) fn render_align_section(
    ui: &mut egui::Ui,
    action: &mut Option<UiAction>,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 4.0);
        widgets_section_label(ui, "Align");
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);

            if IconButton::new(
                icon!("align-left.png"),
                "Align Left",
            )
            .small()
            .show(ui)
            {
                *action = Some(UiAction::AlignLeft);
            }
            if IconButton::new(
                icon!("align-center-v.png"),
                "Align Center (V)",
            )
            .small()
            .show(ui)
            {
                *action = Some(UiAction::AlignCenterV);
            }
            if IconButton::new(
                icon!("align-right.png"),
                "Align Right",
            )
            .small()
            .show(ui)
            {
                *action = Some(UiAction::AlignRight);
            }
            if IconButton::new(
                icon!("align-top.png"),
                "Align Top",
            )
            .small()
            .show(ui)
            {
                *action = Some(UiAction::AlignTop);
            }
            if IconButton::new(
                icon!("align-center-h.png"),
                "Align Center (H)",
            )
            .small()
            .show(ui)
            {
                *action = Some(UiAction::AlignCenterH);
            }
            if IconButton::new(
                icon!("align-bottom.png"),
                "Align Bottom",
            )
            .small()
            .show(ui)
            {
                *action = Some(UiAction::AlignBottom);
            }
        });
    });
}

use super::{UiAction, UiState};
use crate::IconButton;
use astra_canvas::tools::ToolKind;
use crate::icon;
use egui::{Align2, Context, ImageSource, Vec2};

struct Tool {
    kind: ToolKind,
    label: &'static str,
    shortcut: &'static str,
    icon: ImageSource<'static>,
}

fn get_tools() -> Vec<Tool> {
    vec![
        Tool {
            kind: ToolKind::Pan,
            label: "Pan",
            shortcut: "H",
            icon: icon!("pan.png"),
        },
        Tool {
            kind: ToolKind::Select,
            label: "Select",
            shortcut: "V / 1",
            icon: icon!("select.png"),
        },
        Tool {
            kind: ToolKind::Rectangle,
            label: "Rectangle",
            shortcut: "R / 2",
            icon: icon!("rectangle.png"),
        },
        Tool {
            kind: ToolKind::Diamond,
            label: "Diamond",
            shortcut: "D / 3",
            icon: icon!("diamond.png"),
        },
        Tool {
            kind: ToolKind::Ellipse,
            label: "Ellipse",
            shortcut: "O / 4",
            icon: icon!("ellipse.png"),
        },
        Tool {
            kind: ToolKind::Arrow,
            label: "Arrow",
            shortcut: "A / 5",
            icon: icon!("arrow.png"),
        },
        Tool {
            kind: ToolKind::Line,
            label: "Line",
            shortcut: "L / 6",
            icon: icon!("line.png"),
        },
        Tool {
            kind: ToolKind::Freehand,
            label: "Draw",
            shortcut: "P / 7",
            icon: icon!("freehand.png"),
        },
        Tool {
            kind: ToolKind::Text,
            label: "Text",
            shortcut: "T / 8",
            icon: icon!("text.png"),
        },
        Tool {
            kind: ToolKind::InsertImage,
            label: "Insert Image",
            shortcut: "9",
            icon: icon!("image.png"),
        },
        Tool {
            kind: ToolKind::LaserPointer,
            label: "Laser",
            shortcut: "Z",
            icon: icon!("laser.png"),
        },
        Tool {
            kind: ToolKind::Eraser,
            label: "Eraser",
            shortcut: "E / 0",
            icon: icon!("eraser.png"),
        },
    ]
}

pub fn render_toolbar(ctx: &Context, ui_state: &UiState) -> Option<UiAction> {
    let mut action = None;
    let tools = get_tools();

    egui::Area::new(egui::Id::new("toolbar"))
        .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 12.0))
        .show(ctx, |ui| {
            crate::panel_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(2.0, 0.0);

                    for tool in &tools {
                        let is_selected = ui_state.current_tool == tool.kind;
                        if IconButton::new(tool.icon.clone(), tool.label)
                            .shortcut(tool.shortcut)
                            .selected(is_selected)
                            .tool()
                            .show(ui)
                        {
                            action = Some(UiAction::SetTool(tool.kind));
                        }
                    }
                });
            });
        });

    action
}

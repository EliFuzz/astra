use super::super::AppState;
use crate::ui::UiAction;
use astra_canvas::GRID_SIZE;
use astra_canvas::tools::ToolKind;

pub(super) fn navigation_map(state: &mut AppState, key_str: &str) -> Option<UiAction> {
    match key_str {
        "v" | "V" | "1" => Some(UiAction::SetTool(ToolKind::Select)),
        "h" | "H" => Some(UiAction::SetTool(ToolKind::Pan)),
        "r" | "R" | "2" => Some(UiAction::SetTool(ToolKind::Rectangle)),
        "d" | "D" | "3" => Some(UiAction::SetTool(ToolKind::Diamond)),
        "o" | "O" | "4" => Some(UiAction::SetTool(ToolKind::Ellipse)),
        "a" | "A" | "5" => Some(UiAction::SetTool(ToolKind::Arrow)),
        "l" | "L" | "6" => Some(UiAction::SetTool(ToolKind::Line)),
        "p" | "P" | "7" => Some(UiAction::SetTool(ToolKind::Freehand)),
        "t" | "T" | "8" => Some(UiAction::SetTool(ToolKind::Text)),
        "9" => Some(UiAction::SetTool(ToolKind::InsertImage)),
        "e" | "E" | "0" => Some(UiAction::SetTool(ToolKind::Eraser)),
        "z" | "Z" => Some(UiAction::SetTool(ToolKind::LaserPointer)),
        "Delete" | "Backspace" if !state.canvas.selection.is_empty() => {
            Some(UiAction::DeleteSelected)
        }
        "ArrowUp" if !state.canvas.selection.is_empty() => {
            Some(UiAction::NudgeSelection(0.0, -GRID_SIZE))
        }
        "ArrowDown" if !state.canvas.selection.is_empty() => {
            Some(UiAction::NudgeSelection(0.0, GRID_SIZE))
        }
        "ArrowLeft" if !state.canvas.selection.is_empty() => {
            Some(UiAction::NudgeSelection(-GRID_SIZE, 0.0))
        }
        "ArrowRight" if !state.canvas.selection.is_empty() => {
            Some(UiAction::NudgeSelection(GRID_SIZE, 0.0))
        }
        "Escape" => {
            handle_escape(state);
            None
        }
        _ => None,
    }
}

fn handle_escape(state: &mut AppState) {
    if state.ui_state.has_open_overlay() {
        state.ui_state.close_overlays();
    } else {
        state.canvas.tool_manager.cancel();
        state.canvas.clear_selection();
        state.canvas.set_tool(ToolKind::Select);
        state.ui_state.current_tool = ToolKind::Select;
    }
}

pub(super) fn shortcut_map(key: &str, has_shift: bool) -> Option<UiAction> {
    match key {
        "a" | "A" => Some(UiAction::SelectAll),
        "s" | "S" => Some(UiAction::SaveLocal),
        "o" | "O" => Some(UiAction::LoadDocument),
        "e" | "E" if has_shift => Some(UiAction::CopyPng),
        "e" | "E" => Some(UiAction::ExportPng),
        "z" | "Z" if has_shift => Some(UiAction::Redo),
        "z" | "Z" => Some(UiAction::Undo),
        "y" | "Y" => Some(UiAction::Redo),
        "g" | "G" if has_shift => Some(UiAction::UngroupSelected),
        "g" | "G" => Some(UiAction::GroupSelected),
        "c" | "C" if has_shift => Some(UiAction::CopyPng),
        "c" | "C" => Some(UiAction::CopyShapes),
        "x" | "X" => Some(UiAction::CutShapes),
        "v" | "V" => Some(UiAction::PasteShapes),
        "d" | "D" => Some(UiAction::Duplicate),
        _ => None,
    }
}

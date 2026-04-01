#[derive(Debug, Clone, PartialEq, Default)]
pub enum WidgetState {
    #[default]
    Normal,
    Hovered,
    Selected,
    Editing(EditingKind),
}

impl WidgetState {
    pub fn is_selected(&self) -> bool {
        matches!(self, Self::Selected | Self::Editing(_))
    }

    pub fn is_editing(&self) -> bool {
        matches!(self, Self::Editing(_))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EditingKind {
    Text,
    Path,
}

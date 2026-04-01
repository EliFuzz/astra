#[derive(Debug, Clone, PartialEq)]
pub enum TextKey {
    Character(String),
    Backspace,
    Delete,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    Escape,
    Copy,
    Cut,
    Paste(String),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TextModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl TextModifiers {
    pub fn action_mod(&self) -> bool {
        if super::platform::is_action_key_meta() {
            self.meta
        } else {
            self.ctrl
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextEditResult {
    Handled,
    ExitEdit,
    NotHandled,
    Copy(String),
}

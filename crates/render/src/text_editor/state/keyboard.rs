use super::core::TextEditState;
use crate::text_editor::keys::{TextEditResult, TextKey, TextModifiers};
use parley::{FontContext, LayoutContext};
use peniko::Brush;

impl TextEditState {
    pub fn handle_key(
        &mut self,
        key: TextKey,
        modifiers: TextModifiers,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext<Brush>,
    ) -> TextEditResult {
        if self.editor.is_composing() {
            return TextEditResult::NotHandled;
        }

        self.cursor_reset();
        let action_mod = modifiers.action_mod();
        let shift = modifiers.shift;

        match key {
            TextKey::Escape => TextEditResult::ExitEdit,
            TextKey::Copy => {
                if let Some(text) = self.editor.selected_text() {
                    TextEditResult::Copy(text.to_string())
                } else {
                    TextEditResult::Handled
                }
            }
            TextKey::Cut => {
                if let Some(text) = self.editor.selected_text().map(|s| s.to_string()) {
                    {
                        let mut drv = self.editor.driver(font_cx, layout_cx);
                        drv.delete();
                    }
                    self.update_layout_cache(font_cx, layout_cx);
                    TextEditResult::Copy(text)
                } else {
                    TextEditResult::Handled
                }
            }
            key => {
                let mut drv = self.editor.driver(font_cx, layout_cx);
                match key {
                    TextKey::Backspace => {
                        if action_mod {
                            drv.backdelete_word();
                        } else {
                            drv.backdelete();
                        }
                    }
                    TextKey::Delete => {
                        if action_mod {
                            drv.delete_word();
                        } else {
                            drv.delete();
                        }
                    }
                    TextKey::Enter => {
                        drv.insert_or_replace_selection("\n");
                    }
                    TextKey::Left => {
                        if action_mod {
                            if shift {
                                drv.select_word_left();
                            } else {
                                drv.move_word_left();
                            }
                        } else if shift {
                            drv.select_left();
                        } else {
                            drv.move_left();
                        }
                    }
                    TextKey::Right => {
                        if action_mod {
                            if shift {
                                drv.select_word_right();
                            } else {
                                drv.move_word_right();
                            }
                        } else if shift {
                            drv.select_right();
                        } else {
                            drv.move_right();
                        }
                    }
                    TextKey::Up => {
                        if shift {
                            drv.select_up();
                        } else {
                            drv.move_up();
                        }
                    }
                    TextKey::Down => {
                        if shift {
                            drv.select_down();
                        } else {
                            drv.move_down();
                        }
                    }
                    TextKey::Home => {
                        if action_mod {
                            if shift {
                                drv.select_to_text_start();
                            } else {
                                drv.move_to_text_start();
                            }
                        } else if shift {
                            drv.select_to_line_start();
                        } else {
                            drv.move_to_line_start();
                        }
                    }
                    TextKey::End => {
                        if action_mod {
                            if shift {
                                drv.select_to_text_end();
                            } else {
                                drv.move_to_text_end();
                            }
                        } else if shift {
                            drv.select_to_line_end();
                        } else {
                            drv.move_to_line_end();
                        }
                    }
                    TextKey::Paste(text) => {
                        drv.insert_or_replace_selection(&text);
                    }
                    TextKey::Character(c) => {
                        if action_mod && (c == "a" || c == "A") {
                            if shift {
                                drv.collapse_selection();
                            } else {
                                drv.select_all();
                            }
                        } else if !action_mod {
                            drv.insert_or_replace_selection(&c);
                        }
                    }
                    TextKey::Escape | TextKey::Copy | TextKey::Cut => {}
                }
                self.update_layout_cache(font_cx, layout_cx);
                TextEditResult::Handled
            }
        }
    }
}

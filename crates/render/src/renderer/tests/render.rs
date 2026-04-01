use crate::{GridStyle, RendererError, TextEditResult, TextKey, TextModifiers};

#[test]
fn grid_style_next_cycles_through_all_variants() {
    assert_eq!(GridStyle::None.next(), GridStyle::Lines);
    assert_eq!(GridStyle::Lines.next(), GridStyle::HorizontalLines);
    assert_eq!(GridStyle::HorizontalLines.next(), GridStyle::CrossPlus);
    assert_eq!(GridStyle::CrossPlus.next(), GridStyle::Dots);
    assert_eq!(GridStyle::Dots.next(), GridStyle::None);
}

#[test]
fn grid_style_name_returns_nonempty_string() {
    for style in [
        GridStyle::None,
        GridStyle::Lines,
        GridStyle::HorizontalLines,
        GridStyle::CrossPlus,
        GridStyle::Dots,
    ] {
        assert!(!style.name().is_empty());
    }
}

#[test]
fn grid_style_default_is_none() {
    assert_eq!(GridStyle::default(), GridStyle::None);
}

#[test]
fn text_modifiers_default_all_false() {
    let m = TextModifiers::default();
    assert!(!m.shift);
    assert!(!m.ctrl);
    assert!(!m.alt);
    assert!(!m.meta);
}

#[test]
fn text_key_character_stores_string() {
    let key = TextKey::Character("a".to_string());
    match key {
        TextKey::Character(s) => assert_eq!(s, "a"),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn text_key_paste_stores_content() {
    let key = TextKey::Paste("pasted text".to_string());
    match key {
        TextKey::Paste(s) => assert_eq!(s, "pasted text"),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn text_edit_result_variants_exist() {
    let _ = TextEditResult::Handled;
    let _ = TextEditResult::ExitEdit;
    let _ = TextEditResult::NotHandled;
    let _ = TextEditResult::Copy("hello".to_string());
}

#[test]
fn renderer_error_init_failed_display() {
    let err = RendererError::InitFailed("GPU unavailable".to_string());
    let msg = err.to_string();
    assert!(msg.contains("GPU unavailable"));
}

#[test]
fn renderer_error_render_failed_display() {
    let err = RendererError::RenderFailed("out of memory".to_string());
    let msg = err.to_string();
    assert!(msg.contains("out of memory"));
}

#[test]
fn renderer_error_surface_display() {
    let err = RendererError::Surface("lost".to_string());
    let msg = err.to_string();
    assert!(msg.contains("lost"));
}

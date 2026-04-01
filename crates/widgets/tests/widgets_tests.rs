use astra_widgets::{
    UiState,
    colors::{colors_match, parse_css_color},
    sizing, theme,
};

#[test]
fn sizing_small_is_18() {
    assert!((sizing::SMALL - 18.0_f32).abs() < f32::EPSILON);
}

#[test]
fn sizing_medium_is_26() {
    assert!((sizing::MEDIUM - 26.0_f32).abs() < f32::EPSILON);
}

#[test]
fn sizing_large_is_32() {
    assert!((sizing::LARGE - 32.0_f32).abs() < f32::EPSILON);
}

#[test]
fn sizing_small_lt_medium_lt_large() {
    assert!(sizing::SMALL < sizing::MEDIUM);
    assert!(sizing::MEDIUM < sizing::LARGE);
}

#[test]
fn theme_accent_has_expected_rgb() {
    let accent = theme::accent();
    assert_eq!(accent.r(), 115);
    assert_eq!(accent.g(), 125);
    assert_eq!(accent.b(), 150);
}

#[test]
fn colors_match_same_color_is_true() {
    assert!(colors_match(theme::accent(), theme::accent()));
}

#[test]
fn colors_match_different_colors_is_false() {
    assert!(!colors_match(theme::accent(), theme::border()));
}

#[test]
fn parse_css_color_valid_hex_returns_correct_rgb() {
    let color = parse_css_color("#3b82f6");
    assert_eq!(color.r(), 0x3b);
    assert_eq!(color.g(), 0x82);
    assert_eq!(color.b(), 0xf6);
}

#[test]
fn parse_css_color_invalid_returns_grey_fallback() {
    let color = parse_css_color("not-a-color");
    assert_eq!(color.r(), 128);
    assert_eq!(color.g(), 128);
    assert_eq!(color.b(), 128);
}

#[test]
fn open_document_dialog_only_sets_requested_dialog() {
    let mut state = UiState::default();

    state.open_document_dialog(true);
    assert!(state.open_recent_dialog_open);
    assert!(!state.open_dialog_open);

    state.close_overlays();
    state.open_document_dialog(false);
    assert!(state.open_dialog_open);
    assert!(!state.open_recent_dialog_open);
}

#[test]
fn open_save_dialog_copies_name_into_input() {
    let mut state = UiState::default();

    state.open_save_dialog("Board");

    assert!(state.save_dialog_open);
    assert_eq!(state.save_name_input, "Board");
}

#[test]
fn remember_recent_document_keeps_first_insert_and_limit() {
    let mut state = UiState::default();

    for index in 0..12 {
        state.remember_recent_document(&format!("doc-{index}"));
    }
    state.remember_recent_document("doc-5");

    assert_eq!(state.recent_documents.len(), 10);
    assert_eq!(state.recent_documents[0], "doc-11");
    assert_eq!(
        state
            .recent_documents
            .iter()
            .filter(|name| *name == "doc-5")
            .count(),
        1
    );
}

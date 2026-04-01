use super::AppState;
use super::platform;
use astra_core::SerializableColor;
use astra_storage::preferences::UserPreferences;

pub(crate) fn apply(state: &mut AppState, prefs: &UserPreferences) {
    astra_widgets::theme::set_dark(prefs.dark_theme);

    state.config.grid_style = prefs.grid_style.into();
    state.ui_state.grid_style = state.config.grid_style;

    let bg = prefs.background_color;
    state.config.background_color = peniko::Color::from_rgba8(bg.r, bg.g, bg.b, bg.a);
    state.ui_state.bg_color = egui::Color32::from_rgba_unmultiplied(bg.r, bg.g, bg.b, bg.a);

    state.ui_state.grid_snap_enabled = prefs.grid_snap;
    state.ui_state.smart_snap_enabled = prefs.smart_guides;
    state.ui_state.angle_snap_enabled = prefs.angle_snap;
}

pub(crate) fn collect(state: &AppState) -> UserPreferences {
    UserPreferences {
        grid_style: state.config.grid_style.into(),
        background_color: SerializableColor::from(state.config.background_color),
        dark_theme: astra_widgets::theme::is_dark(),
        grid_snap: state.ui_state.grid_snap_enabled,
        smart_guides: state.ui_state.smart_snap_enabled,
        angle_snap: state.ui_state.angle_snap_enabled,
    }
}

pub(crate) fn save(state: &AppState) {
    let prefs = collect(state);
    platform::save_preferences(&prefs);
}

use crate::app::AppConfig;

#[test]
fn default_title_is_astra() {
    assert_eq!(AppConfig::default().title, "Astra");
}

#[test]
fn default_width_is_1280() {
    assert_eq!(AppConfig::default().width, 1280);
}

#[test]
fn default_height_is_800() {
    assert_eq!(AppConfig::default().height, 800);
}

#[test]
fn clone_preserves_title() {
    let config = AppConfig::default();
    assert_eq!(config.clone().title, config.title);
}

#[test]
fn clone_preserves_dimensions() {
    let config = AppConfig::default();
    let cloned = config.clone();
    assert_eq!(cloned.width, config.width);
    assert_eq!(cloned.height, config.height);
}

#[test]
fn custom_title_overrides_default() {
    let config = AppConfig {
        title: "My Board".to_string(),
        ..AppConfig::default()
    };
    assert_eq!(config.title, "My Board");
}

#[test]
fn custom_dimensions_override_default() {
    let config = AppConfig {
        width: 1920,
        height: 1080,
        ..AppConfig::default()
    };
    assert_eq!(config.width, 1920);
    assert_eq!(config.height, 1080);
}

#[test]
fn title_is_non_empty_by_default() {
    assert!(!AppConfig::default().title.is_empty());
}

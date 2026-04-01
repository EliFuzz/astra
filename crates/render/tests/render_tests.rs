use astra_render::GridStyle;

#[test]
fn integration_grid_style_default_matches_unit_tests() {
    assert_eq!(GridStyle::default(), GridStyle::None);
}

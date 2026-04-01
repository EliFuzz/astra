use astra_render::GridStyle;
use peniko::Color;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub grid_style: GridStyle,
    pub background_color: Color,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "Astra".to_string(),
            width: 1280,
            height: 800,
            grid_style: GridStyle::None,
            background_color: Color::from_rgba8(18, 18, 24, 255),
        }
    }
}

mod app;
mod ui;

pub use app::{App, AppConfig};
pub use astra_widgets::{UiAction, UiState, render_ui};

#[cfg(target_arch = "wasm32")]
mod wasm_entry {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn register_canvas_font(data: Vec<u8>) {
        astra_render::register_canvas_font(data);
    }

    #[wasm_bindgen]
    pub fn register_math_font(data: Vec<u8>) {
        astra_render::register_math_font(data);
    }

    #[wasm_bindgen]
    pub fn commit_fonts() {
        astra_render::commit_fonts();
    }

    #[wasm_bindgen(start)]
    pub async fn run_wasm() {
        console_error_panic_hook::set_once();
        console_log::init_with_level(log::Level::Info).ok();
        super::App::run().await;
    }
}

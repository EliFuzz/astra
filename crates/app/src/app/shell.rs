use std::sync::Arc;
use winit::window::Window;

use crate::app::config::AppConfig;
use crate::app::platform::PlatformAppFields;

pub struct App {
    pub(crate) config: AppConfig,
    pub(crate) state: Option<crate::app::state::AppState>,
    pub(crate) render_cx: Option<vello::util::RenderContext>,
    pub(crate) pending_window: Option<Arc<Window>>,
    #[allow(dead_code)]
    pub(crate) platform: PlatformAppFields,
}

mod config;
mod event_loop;
mod keyboard;
mod mouse;
pub(crate) mod platform;
mod preferences;
mod presentation;
mod resume;
mod run;
mod shell;
mod state;
mod ui_actions;
mod window_events;

#[cfg(test)]
mod tests;

pub use config::AppConfig;
pub use shell::App;
pub(crate) use state::AppState;

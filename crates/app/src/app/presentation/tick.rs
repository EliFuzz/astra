use super::super::App;

pub fn handle_new_events(app: &mut App) {
    if let Some(state) = app.state.as_mut() {
        state.input.step();
    }
}

pub fn handle_about_to_wait(app: &mut App) {
    if let Some(state) = app.state.as_mut() {
        state.input.end_step();
    }
}

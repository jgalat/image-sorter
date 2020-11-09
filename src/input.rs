use crate::app::{Action, App, TabId};

pub fn handle_app_key(key: char, app: &mut App) {
    match key {
        's' => app.push_action(Action::Skip(app.current_image())),
        'z' => app.pop_action(),
        _ => {}
    }
}

pub fn handle_mapping_key(key: char, app: &mut App) {
    if app.current_tab() != TabId::Main {
        return;
    }

    if let Some(path) = app.key_mapping.get(&key).cloned() {
        app.push_action(Action::Move(app.current_image(), path));
    }
}

use crate::app::{Action, App, TabId};

pub fn handle_app_key(key: char, app: &mut App) {
    match key {
        's' => {
            if let Some(image_path) = app.current_image() {
                app.push_action(Action::Skip(image_path));
            }
        }
        'z' => app.pop_action(),
        _ => {}
    }
}

pub fn handle_mapping_key(key: char, app: &mut App) {
    if app.current_tab() != TabId::Main {
        return;
    }

    if let Some(path) = app.key_mapping.get(&key).cloned() {
        if let Some(image_path) = app.current_image() {
            app.push_action(Action::Move(image_path, path));
        }
    }
}

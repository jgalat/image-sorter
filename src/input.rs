use crate::app::App;

pub fn handle_app_key(key: char, app: &mut App) {
    match key {
        's' => app.add_action(None),
        'z' => app.revert_action(),
        _ => {}
    }
}

pub fn handle_mapping_key(key: char, app: &mut App) {
    if let Some(path) = &mut app.key_mapping.get(&key).cloned() {
        app.add_action(Some(path.clone()));
    }
}

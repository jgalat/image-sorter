use termion::event::Key;

use crate::app::{Action, App};

pub fn handle_key_main(key: Key, app: &mut App) {
    match key {
        Key::Ctrl(key) => handle_app_key(key, app),
        Key::Char(key) => handle_mapping_key(key, app),
        _ => {}
    }
}

fn handle_app_key(key: char, app: &mut App) {
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

fn handle_mapping_key(key: char, app: &mut App) {
    if let Some(path) = app.key_mapping.get(&key).cloned() {
        if let Some(image_path) = app.current_image() {
            app.push_action(Action::Move(image_path, path));
        }
    }
}

pub fn handle_key_script(key: Key, app: &mut App) {
    match key {
        Key::Up | Key::Char('k') => app.scroll_up(),
        Key::Down | Key::Char('j') => app.scroll_down(),
        Key::Left | Key::Char('h') => app.scroll_left(),
        Key::Right | Key::Char('l') => app.scroll_right(),
        _ => {}
    }
}

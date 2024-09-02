use termion::event::Key;

use crate::app::{Action, App};

pub fn handle_key_main(key: Key, app: &mut App) {
    match key {
        Key::Backspace => {
            if let Some(i) = app.current_image() {
                app.push_action(Action::Delete(i));
            }
        }
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
    if let Some(mut path) = app.key_mapping.get_mut(&key).cloned() {
        if let Some(image_path) = app.current_image() {
            if let Some(Action::Rename(name)) = app.actions.last() {
                path.push(name);
            }
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

pub fn handle_key_input(key: Key, app: &mut App) {
    match key {
        Key::Ctrl('k') => {
            app.input.drain(app.input_idx..app.input.len());
        }
        Key::Ctrl('u') => {
            app.input.drain(..app.input_idx);
            app.input_idx = 0;
        }
        Key::Ctrl('l') => {
            app.input = vec![];
            app.input_idx = 0;
        }
        Key::Ctrl('w') => {
            if app.input_idx == 0 {
                return;
            }
            let word_end = match app.input[..app.input_idx].iter().rposition(|&x| x != ' ') {
                Some(index) => index + 1,
                None => 0,
            };
            let word_start = match app.input[..word_end].iter().rposition(|&x| x == ' ') {
                Some(index) => index + 1,
                None => 0,
            };
            app.input.drain(word_start..app.input_idx);
            app.input_idx = word_start;
        }
        Key::End | Key::Ctrl('e') => {
            app.input_idx = app.input.len();
        }
        Key::Home | Key::Ctrl('a') => {
            app.input_idx = 0;
        }
        Key::Left | Key::Ctrl('b') => {
            if app.input_idx > 0 {
                app.input_idx -= 1;
            }
        }
        Key::Right | Key::Ctrl('f') => {
            if app.input_idx < app.input.len() {
                app.input_idx += 1;
            }
        }
        Key::Esc => {
            app.enable_input = false;
        }
        Key::Char('\n') => {
            let input_str: String = app.input.iter().collect();
            app.push_action(Action::Rename(input_str));
            app.enable_input = false;
        }
        Key::Backspace | Key::Ctrl('h') => {
            if !app.input.is_empty() && app.input_idx > 0 {
                app.input.remove(app.input_idx - 1);
                app.input_idx -= 1;
            }
        }
        Key::Delete | Key::Ctrl('d') => {
            if !app.input.is_empty() && app.input_idx < app.input.len() {
                app.input.remove(app.input_idx);
            }
        }
        Key::Char(c) => {
            app.input.insert(app.input_idx, c);
            app.input_idx += 1;
        }
        _ => {}
    }
}

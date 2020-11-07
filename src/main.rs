mod app;
mod event;
mod image_display;
mod input;
mod render;

use anyhow::Result;
use std::io;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

use crate::app::{App, TabId};
use crate::event::{Event, EventsListener};
use crate::image_display::ImageDisplay;
use crate::input::{handle_app_key, handle_mapping_key};
use crate::render::{render_layout, render_main, render_script};

fn main() -> Result<()> {
    let mut app = App::new();

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events_listener = EventsListener::default();
    let mut image_display = ImageDisplay::new()?;

    loop {
        terminal.draw(|mut f| {
            let window = render_layout(&mut f, &app);
            if let Err(err) = match app.current_tab() {
                TabId::Main => render_main(&mut f, &app, &mut image_display, window),
                TabId::Script => render_script(&mut f, &app, window),
            } {
                eprintln!("ERROR: {}", err);
                panic!(err);
            }
        })?;

        match events_listener.next()? {
            Event::Tick => continue,
            Event::Input(Key::Ctrl('c')) => break,
            Event::Input(Key::BackTab) => app.switch_tab(),
            Event::Input(Key::Ctrl(key)) => handle_app_key(key, &mut app),
            Event::Input(Key::Char(key)) => handle_mapping_key(key, &mut app),
            _ => {}
        }
    }

    Ok(())
}

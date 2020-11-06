mod app;
mod event;
mod image_display;
mod render;

use anyhow::Result;
use std::io;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

use crate::app::{App, RouteId};
use crate::event::{Event, EventsListener};
use crate::image_display::ImageDisplay;
use crate::render::{render_layout, render_main};

fn main() -> Result<()> {
    let mut app = App::new();

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events_listener = EventsListener::default();
    let mut image_display = ImageDisplay::new()?;

    let mut render = true;

    loop {
        let terminal_size = terminal.size()?;
        if app.size != terminal_size || render {
            render = false;
            app.size = terminal_size;
            terminal.draw(|mut f| {
                let window = render_layout(&mut f, &mut app);
                if let Err(err) = match app.route {
                    RouteId::Main => render_main(&mut f, &mut app, &mut image_display, window),
                    RouteId::Bindings => Ok(()),
                    RouteId::ResultScript => Ok(()),
                } {
                    eprintln!("ERROR: {}", err);
                    panic!(err);
                }
            })?;
        }

        match events_listener.next()? {
            Event::Tick => continue,
            Event::Input(Key::Ctrl('c')) => break,
            Event::Input(Key::Char('r')) => render = true,
            _ => {}
        }
    }

    Ok(())
}

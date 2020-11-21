mod app;
mod event;
mod image_display;
mod input;
mod render;

use anyhow::{anyhow, Result};
use std::{io, path::PathBuf};
use structopt::StructOpt;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

use crate::app::{App, TabId};
use crate::event::{Event, EventsListener};
use crate::image_display::ImageDisplay;
use crate::input::{handle_key_main, handle_key_script};
use crate::render::{render_layout, render_main, render_script};

fn parse_key_val(s: &str) -> Result<(char, PathBuf)> {
    let pos = s
        .find('=')
        .ok_or_else(|| anyhow!(format!("invalid KEY=value: no `=` found in `{}`", s)))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    author = env!("CARGO_PKG_AUTHORS") ,
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct Opt {
    #[structopt(
        short,
        long,
        help = "Bind a char to a folder, CHAR=FOLDER",
        parse(try_from_str = parse_key_val),
    )]
    bind: Vec<(char, PathBuf)>,

    #[structopt(
        help = "Images or folders containing images to sort",
        parse(from_os_str)
    )]
    input: Vec<PathBuf>,

    #[structopt(
        short,
        long,
        help = "Name the output script",
        default_value = "sort.sh"
    )]
    output: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut app = App::new(opt)?;

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
                eprintln!("ERROR: {:?}", err);
                panic!(err);
            }
        })?;

        match events_listener.next()? {
            Event::Tick => continue,
            Event::Input(Key::Ctrl('c')) => break,
            Event::Input(Key::Ctrl('w')) => app.write()?,
            Event::Input(Key::BackTab) => app.switch_tab(),
            Event::Input(key) => match app.current_tab() {
                TabId::Main => handle_key_main(key, &mut app),
                TabId::Script => handle_key_script(key, &mut app),
            },
        }
    }

    Ok(())
}

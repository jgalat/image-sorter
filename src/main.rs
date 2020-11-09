mod app;
mod event;
mod image_display;
mod input;
mod render;

use anyhow::Result;
use clap::{App as ClapApp, Arg};
use std::io;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

use crate::app::{App, TabId};
use crate::event::{Event, EventsListener};
use crate::image_display::ImageDisplay;
use crate::input::{handle_key_main, handle_key_script};
use crate::render::{render_layout, render_main, render_script};

fn main() -> Result<()> {
    let matches = ClapApp::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("bind")
                .help("Bind a char to a folder")
                .short("b")
                .long("bind")
                .takes_value(true)
                .number_of_values(2)
                .value_names(&["char", "folder"])
                .multiple(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Name the output script")
                .short("o")
                .long("output")
                .takes_value(true)
                .number_of_values(1)
                .value_names(&["file"]),
        )
        .arg(
            Arg::with_name("input")
                .help("Input images or folders to sort")
                .takes_value(true)
                .last(true),
        )
        .get_matches();

    let mut app = App::default();

    if let Some(bind_args) = matches.values_of("bind") {
        app.parse_key_mapping(bind_args.collect())?;
    }
    if let Some(input_args) = matches.values_of("input") {
        app.parse_input_files(input_args.collect())?;
    }
    if let Some(output_arg) = matches.value_of("output") {
        app.output = output_arg.to_string();
    }

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
            Event::Input(Key::BackTab) => app.switch_tab(),
            Event::Input(key) => match app.current_tab() {
                TabId::Main => handle_key_main(key, &mut app),
                TabId::Script => handle_key_script(key, &mut app),
            },
        }
    }

    Ok(())
}

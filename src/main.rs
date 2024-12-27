mod app;
mod event;
mod image_display;
mod input;
mod render;

use anyhow::{anyhow, Result};
use expanduser::expanduser;
use ratatui::{backend::TermionBackend, Terminal};
use std::{io, path::PathBuf, time::Duration};
use structopt::StructOpt;
use termion::{cursor::Goto, event::Key, raw::IntoRawMode, screen::IntoAlternateScreen};

use crate::app::{App, TabId};
use crate::event::{Event, EventsListener};
use crate::image_display::ImageDisplay;
use crate::input::{handle_key_input, handle_key_main, handle_key_script};
use crate::render::{render_layout, render_main, render_script};

fn parse_key_val(s: &str) -> Result<(char, PathBuf)> {
    let pos = s
        .find('=')
        .ok_or_else(|| anyhow!(format!("invalid KEY=value: no `=` found in `{}`", s)))?;
    let dir: String = s[pos + 1..].parse()?;
    Ok((s[..pos].parse()?, expanduser(dir)?))
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
        help = "Search for images recursively in the input folders"
    )]
    recurse: bool,

    #[structopt(
        short,
        long,
        help = "Name the output script",
        default_value = "sort.sh"
    )]
    output: String,

    #[structopt(short, long, help = "App tick rate (ms)", default_value = "1000")]
    tick_rate: u64,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let events_listener = EventsListener::new(Duration::from_millis(opt.tick_rate));
    let mut app = App::new(opt)?;

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = stdout.into_alternate_screen()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let image_display = ImageDisplay::new()?;

    loop {
        terminal.draw(|f| {
            let window = render_layout(f, &app);
            if let Err(err) = match app.current_tab() {
                TabId::Main => render_main(f, &app, &image_display, window),
                TabId::Script => render_script(f, &app, window),
            } {
                eprintln!("ERROR: {:?}", err);
                panic!("{}", err);
            }
        })?;

        if app.enable_input {
            terminal.show_cursor()?;
            let size = terminal.size()?;
            print!("{}", Goto(app.input_idx as u16 + 2, size.height - 1))
        } else {
            terminal.hide_cursor()?;
        }

        match events_listener.next()? {
            Event::Tick => continue,
            Event::Input(key) => {
                if key == Key::Ctrl('c') {
                    break;
                }

                if app.enable_input {
                    handle_key_input(key, &mut app);
                } else {
                    // App controls
                    match key {
                        Key::Ctrl('w') => app.write()?,
                        Key::Ctrl('r') => app.rename_current_image(),
                        Key::Char('\t') => app.switch_tab(),
                        _ => match app.current_tab() {
                            TabId::Main => handle_key_main(key, &mut app),
                            TabId::Script => handle_key_script(key, &mut app),
                        },
                    }
                }
            }
        }
    }

    Ok(())
}

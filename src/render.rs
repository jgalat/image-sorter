use anyhow::Result;
use std::io;
use termion::{raw::RawTerminal, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    terminal::Frame,
    text::{Spans, Text},
    widgets::{Block, Borders, Paragraph, Row, Table, Tabs},
};

use crate::app::App;
use crate::image_display::ImageDisplay;

pub fn render_layout(
    f: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>,
    app: &App,
) -> Rect {
    let window = f.size();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)].as_ref())
        .split(window);

    let titles = ["Main", "Script"]
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().title("image-sorter").borders(Borders::ALL))
        .select(app.tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Red));

    f.render_widget(tabs, layout[0]);
    layout[1]
}

pub fn render_main(
    f: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>,
    app: &App,
    image_display: &mut ImageDisplay,
    window: Rect,
) -> Result<()> {
    let image_block = Block::default()
        .borders(Borders::ALL)
        .title(app.current_image());
    let next_up_block = Block::default().borders(Borders::ALL).title("Next up");
    let status_block = Block::default().borders(Borders::ALL).title("Status");
    let key_bindings_block = Block::default().borders(Borders::ALL).title("Key bindings");
    let controls_block = Block::default().borders(Borders::ALL).title("Controls");

    let window_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Length(30)].as_ref())
        .split(window);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(9)].as_ref())
        .split(window_layout[0]);

    let sidebar_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(9),
            ]
            .as_ref(),
        )
        .split(window_layout[1]);

    let status_container = status_block.inner(sidebar_layout[0]);
    render_status(f, app, status_container);

    let key_bindings_container = key_bindings_block.inner(sidebar_layout[1]);
    render_key_bindings(f, app, key_bindings_container);

    let controls_container = controls_block.inner(sidebar_layout[2]);
    render_controls(f, controls_container);

    let terminal_size = f.size();
    let image_container = image_block.inner(main_layout[0]);
    image_display.render_image(app.current_image(), image_container, terminal_size)?;

    f.render_widget(image_block, main_layout[0]);
    f.render_widget(next_up_block, main_layout[1]);
    f.render_widget(status_block, sidebar_layout[0]);
    f.render_widget(key_bindings_block, sidebar_layout[1]);
    f.render_widget(controls_block, sidebar_layout[2]);

    Ok(())
}

fn render_status(
    f: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>,
    app: &App,
    window: Rect,
) {
    let status = Text::from(format!("Sorted: {}/{}", app.current, app.images.len()));
    let paragraph = Paragraph::new(status).alignment(Alignment::Center);
    f.render_widget(paragraph, window);
}

fn render_key_bindings(
    f: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>,
    app: &App,
    window: Rect,
) {
    let keys = app
        .key_bindings
        .iter()
        .map(|(key, path)| Row::Data(vec![key.to_string(), path.clone()].into_iter()));
    let key_bindings = Table::new(["Key", "Path"].iter(), keys.into_iter())
        .widths([Constraint::Length(3), Constraint::Length(20)].as_ref())
        .header_gap(0)
        .header_style(Style::default().fg(Color::Red));

    f.render_widget(key_bindings, window);
}

fn render_controls(
    f: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>,
    window: Rect,
) {
    let controls = Table::new(
        ["Key", "Action"].iter(),
        vec![
            Row::Data(["Ctrl-C", "Exit"].iter()),
            Row::Data(["Shift-Tab", "Switch tabs"].iter()),
            Row::Data(["", ""].iter()),
            Row::Data(["Ctrl-S", "Skip image"].iter()),
            Row::Data(["Ctrl-Z", "Undo action"].iter()),
            Row::Data(["Ctrl-W", "Commit actions"].iter()),
        ]
        .into_iter(),
    )
    .widths([Constraint::Length(10), Constraint::Length(20)].as_ref())
    .header_gap(0)
    .header_style(Style::default().fg(Color::Red));

    f.render_widget(controls, window);
}

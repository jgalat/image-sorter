use anyhow::Result;
use std::io;
use termion::{raw::RawTerminal, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    terminal::Frame,
    text::{Spans, Text},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
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
    let current_image = app.current_image();
    let image_block = Block::default().borders(Borders::ALL).title(current_image);

    let controls_block = Block::default().borders(Borders::ALL).title("Controls");
    let status_block = Block::default().borders(Borders::ALL).title("Status");

    let window_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(window);

    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)].as_ref())
        .split(window_layout[1]);

    let status_container = status_block.inner(sidebar[0]);
    render_status(f, app, status_container);

    let controls_container = controls_block.inner(sidebar[1]);
    render_controls(f, controls_container);

    let terminal_size = f.size();
    let image_container = image_block.inner(window_layout[0]);
    image_display.render_image(current_image, image_container, terminal_size)?;

    f.render_widget(image_block, window_layout[0]);
    f.render_widget(status_block, sidebar[0]);
    f.render_widget(controls_block, sidebar[1]);

    Ok(())
}

fn render_status(
    f: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>,
    app: &App,
    window: Rect,
) {
    let status = Text::from(format!("{}/{}", app.current + 1, app.images.len()));
    let paragraph = Paragraph::new(status).alignment(Alignment::Center);
    f.render_widget(paragraph, window);
}

fn render_controls(
    f: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>,
    window: Rect,
) {
    let controls = Text::from(
        r#"Ctrl-C - Exit
        Shift-Tab - Change tabs

        Ctrl-S - Skip image
        Ctrl-Z - Undo action
        Ctrl-W - Commit actions
        "#,
    );

    let paragraph = Paragraph::new(controls).wrap(Wrap { trim: true });

    f.render_widget(paragraph, window);
}

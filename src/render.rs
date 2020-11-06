use anyhow::Result;
use std::io;
use termion::{raw::RawTerminal, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    terminal::Frame,
    widgets::{Block, Borders, Tabs},
};

use crate::app::{App, RouteId};
use crate::image_display::ImageDisplay;

pub fn render_layout(
    f: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>,
    app: &mut App,
) -> Rect {
    let window = f.size();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Max(0)].as_ref())
        .split(window);

    let selected = match app.route {
        RouteId::Main => 0,
        RouteId::Bindings => 1,
        RouteId::ResultScript => 2,
    };

    let tabs = Tabs::default()
        .block(Block::default().title("image-sorter").borders(Borders::ALL))
        .titles(&["Main", "Bindings", "Result script"])
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Red));

    f.render_widget(tabs, layout[0]);
    layout[1]
}

pub fn render_main(
    f: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>,
    app: &mut App,
    image_display: &mut ImageDisplay,
    window: Rect,
) -> Result<()> {
    let current_image = app.images[app.current];
    let image_block = Block::default().borders(Borders::ALL).title(current_image);

    let controls_block = Block::default().borders(Borders::ALL).title("Controls");
    let status_block = Block::default().borders(Borders::ALL).title("Status");

    let window_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(window);

    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(window_layout[1]);

    f.render_widget(image_block, window_layout[0]);
    f.render_widget(status_block, sidebar[0]);
    f.render_widget(controls_block, sidebar[1]);

    let image_block = image_block.inner(window_layout[0]);
    let terminal_size = f.size();
    image_display.render_image(current_image, image_block, terminal_size)?;
    Ok(())
}

use anyhow::Result;
use std::io;
use termion::{raw::RawTerminal, screen::AlternateScreen};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    terminal::Frame,
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Row, Table, Tabs},
};

use crate::app::{Action, App};
use crate::image_display::ImageDisplay;

pub fn render_layout<B>(f: &mut Frame<B>, app: &App) -> Rect
where
    B: Backend,
{
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

pub fn render_main<B>(
    f: &mut Frame<B>,
    app: &App,
    image_display: &mut ImageDisplay,
    window: Rect,
) -> Result<()>
where
    B: Backend,
{
    let image_title = match app.current_image() {
        None => "No more images left to sort".to_string(),
        Some(image_path) => image_path,
    };
    let image_block = Block::default().borders(Borders::ALL).title(image_title);
    let status_block = Block::default().borders(Borders::ALL).title("Status");
    let key_mapping_block = Block::default().borders(Borders::ALL).title("Key mapping");
    let controls_block = Block::default().borders(Borders::ALL).title("Controls");

    let window_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Length(30)].as_ref())
        .split(window);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10)].as_ref())
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

    let key_mapping_container = key_mapping_block.inner(sidebar_layout[1]);
    render_key_mapping(f, app, key_mapping_container);

    let controls_container = controls_block.inner(sidebar_layout[2]);
    render_controls(f, controls_container);

    if let Some(image_path) = app.current_image() {
        let terminal_size = f.size();
        let image_container = image_block.inner(main_layout[0]);
        image_display.render_image(image_path, image_container, terminal_size)?;
    }

    f.render_widget(image_block, main_layout[0]);
    f.render_widget(status_block, sidebar_layout[0]);
    f.render_widget(key_mapping_block, sidebar_layout[1]);
    f.render_widget(controls_block, sidebar_layout[2]);

    Ok(())
}

fn render_status<B>(f: &mut Frame<B>, app: &App, window: Rect)
where
    B: Backend,
{
    let status = Text::from(format!("Sorted: {}/{}", app.current, app.images.len()));
    let paragraph = Paragraph::new(status).alignment(Alignment::Center);
    f.render_widget(paragraph, window);
}

fn render_key_mapping<B>(f: &mut Frame<B>, app: &App, window: Rect)
where
    B: Backend,
{
    let keys = app
        .key_mapping
        .iter()
        .map(|(key, path)| Row::Data(vec![key.to_string(), path.clone()].into_iter()));
    let key_mapping = Table::new(["Key", "Path"].iter(), keys)
        .widths([Constraint::Length(3), Constraint::Length(25)].as_ref())
        .header_gap(0)
        .header_style(Style::default().fg(Color::Red));

    f.render_widget(key_mapping, window);
}

fn render_controls<B>(f: &mut Frame<B>, window: Rect)
where
    B: Backend,
{
    let controls = Table::new(
        ["Key", "Action"].iter(),
        vec![
            Row::Data(["Ctrl-C", "Exit"].iter()),
            Row::Data(["Shift-Tab", "Switch tabs"].iter()),
            Row::Data(["", ""].iter()),
            Row::Data(["Ctrl-S", "Skip image"].iter()),
            Row::Data(["Ctrl-Z", "Undo action"].iter()),
            Row::Data(["Ctrl-W", "Save script"].iter()),
        ]
        .into_iter(),
    )
    .widths([Constraint::Length(10), Constraint::Length(20)].as_ref())
    .header_gap(0)
    .header_style(Style::default().fg(Color::Red));

    f.render_widget(controls, window);
}

pub fn render_script<B>(f: &mut Frame<B>, app: &App, window: Rect) -> Result<()>
where
    B: Backend,
{
    let comment_style = Style::default().fg(Color::Yellow);
    let mut lines = vec![
        Span::styled("#!/bin/sh", comment_style),
        Span::styled(
            format!(
                "# Press Ctrl+W to save the following script to {}",
                app.output
            ),
            comment_style,
        ),
        Span::styled(
            "# Use the arrows keys (or h j k l) to scroll",
            comment_style,
        ),
    ];

    for action in app.actions.iter() {
        match action {
            Action::Skip(image) => {
                lines.push(Span::styled(format!("# Skipped {}", image), comment_style))
            }
            Action::Move(image, path) => lines.push(Span::from(format!("mv {} {}", image, path))),
            Action::MkDir(path) => lines.push(Span::from(format!("mkdir -p {}", path))),
        }
    }

    let lines: Vec<Spans> = lines.into_iter().map(Spans::from).collect();

    let script_block = Block::default().borders(Borders::ALL);

    let paragraph = Paragraph::new(lines)
        .block(script_block)
        .scroll(app.script_offset);

    f.render_widget(paragraph, window);

    Ok(())
}

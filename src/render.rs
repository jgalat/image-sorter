use anyhow::Result;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    terminal::Frame,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Row, Table, Tabs},
};
use std::time::Duration;

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

    let titles = ["Main", "Script"].iter().cloned().map(Line::from).collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().title("image-sorter").borders(Borders::ALL))
        .select(app.tab)
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
        Some(image_path) => {
            let image_path = image_path.display().to_string();
            if let Some(Action::Rename(name)) = app.actions.last() {
                format!("{} - Renamed to {}", image_path, name)
            } else {
                image_path
            }
        }
    };
    let image_block = Block::default().borders(Borders::ALL).title(image_title);

    let window_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Length(30)].as_ref())
        .split(window);

    let main_layout_constraints = if app.enable_input {
        vec![Constraint::Min(10), Constraint::Length(3)]
    } else {
        vec![Constraint::Min(10)]
    };

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(main_layout_constraints)
        .split(window_layout[0]);

    let sidebar_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(10),
            ]
            .as_ref(),
        )
        .split(window_layout[1]);

    render_status(f, app, sidebar_layout[0]);
    render_key_mapping(f, app, sidebar_layout[1]);
    render_controls(f, sidebar_layout[2]);

    if let Some(image_path) = app.current_image() {
        let terminal_size = f.size();
        let image_container = image_block.inner(main_layout[0]);
        image_display.render_image(image_path, image_container, terminal_size)?;
    }

    f.render_widget(image_block, main_layout[0]);
    if app.enable_input {
        render_rename_input(f, app, main_layout[1]);
    }

    Ok(())
}

fn render_rename_input<B>(f: &mut Frame<B>, app: &App, window: Rect)
where
    B: Backend,
{
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .title("Rename");
    let text: String = app.input.iter().collect();
    let text = Text::from(text);
    let paragraph = Paragraph::new(text).block(input_block);

    f.render_widget(paragraph, window);
}

const STATUS_DURATION: Duration = Duration::from_secs(2);

fn render_status<B>(f: &mut Frame<B>, app: &App, window: Rect)
where
    B: Backend,
{
    let status_block = Block::default().borders(Borders::ALL).title("Status");
    let mut status = format!("Sorted: {}/{}", app.current, app.images.len());
    if let Some(last_save) = app.last_save {
        if last_save.elapsed() < STATUS_DURATION {
            status = "Script saved!".to_string();
        }
    }
    let paragraph = Paragraph::new(Text::from(status))
        .alignment(Alignment::Center)
        .block(status_block);
    f.render_widget(paragraph, window);
}

fn render_key_mapping<B>(f: &mut Frame<B>, app: &App, window: Rect)
where
    B: Backend,
{
    let key_mapping_block = Block::default().borders(Borders::ALL).title("Key mapping");
    let keys = app
        .key_mapping
        .iter()
        .map(|(key, path)| Row::new(vec![key.to_string(), path.display().to_string()]));

    let key_mapping = Table::new(keys)
        .widths([Constraint::Length(3), Constraint::Length(25)].as_ref())
        .header(Row::new(["Key", "Path"]).style(Style::default().fg(Color::Red)))
        .block(key_mapping_block);

    f.render_widget(key_mapping, window);
}

fn render_controls<B>(f: &mut Frame<B>, window: Rect)
where
    B: Backend,
{
    let controls_block = Block::default().borders(Borders::ALL).title("Controls");
    let controls = Table::new(vec![
        Row::new(["Ctrl-C", "Exit"]),
        Row::new(["Tab", "Switch tabs"]),
        Row::new(["", ""]),
        Row::new(["Ctrl-R", "Rename image"]),
        Row::new(["Ctrl-S", "Skip image"]),
        Row::new(["Ctrl-Z", "Undo action"]),
        Row::new(["Ctrl-W", "Save script"]),
    ])
    .widths([Constraint::Length(10), Constraint::Length(20)].as_ref())
    .header(Row::new(["Key", "Action"]).style(Style::default().fg(Color::Red)))
    .block(controls_block);

    f.render_widget(controls, window);
}

pub fn render_script<B>(f: &mut Frame<B>, app: &App, window: Rect) -> Result<()>
where
    B: Backend,
{
    let comment_style = Style::default().fg(Color::Yellow);
    let mut lines = vec![
        Line::styled("#!/bin/sh", comment_style),
        Line::styled(
            format!(
                "# Press Ctrl+W to save the following script to {}",
                app.output
            ),
            comment_style,
        ),
        Line::styled(
            "# Use the arrows keys (or h j k l) to scroll",
            comment_style,
        ),
    ];

    for action in app.actions.iter() {
        match action {
            Action::Skip(image) => lines.push(Line::styled(
                format!("# Skipped {}", image.display()),
                comment_style,
            )),
            Action::MkDir(path) => {
                lines.push(Line::from(format!("mkdir -p \"{}\"", path.display())))
            }
            Action::Move(image, path) => lines.push(Line::from(format!(
                "mv \"{}\" \"{}\"",
                image.display(),
                path.display()
            ))),
            _ => {}
        }
    }

    let lines: Vec<Line> = lines.into_iter().map(Line::from).collect();
    let script_block = Block::default().borders(Borders::ALL);
    let paragraph = Paragraph::new(lines)
        .block(script_block)
        .scroll(app.script_offset);

    f.render_widget(paragraph, window);
    Ok(())
}

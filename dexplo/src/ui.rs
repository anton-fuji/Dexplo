use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Wrap},
};

pub fn ui(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // Main layout: header, content, footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(size);

    // Split content area into left (file list) and right (preview)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_chunks[1]);

    // Header
    let path_display = app
        .current_path
        .strip_prefix(std::env::current_dir().unwrap_or_default())
        .unwrap_or(&app.current_path)
        .display()
        .to_string();
    let path_display = if path_display.is_empty() {
        ".".to_string()
    } else {
        path_display
    };

    let header = Paragraph::new(format!("  {}", path_display))
        .style(
            Style::default()
                .fg(Color::Rgb(100, 200, 255))
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(70, 130, 200)))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .padding(Padding::horizontal(1)),
        );

    // File list (left side)
    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|(name, is_dir)| {
            let style = if *is_dir {
                Style::default()
                    .fg(Color::Rgb(120, 180, 255))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(180, 200, 220))
            };
            ListItem::new(format!(" {}", name)).style(style)
        })
        .collect();

    let file_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(70, 130, 200)))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .padding(Padding::horizontal(1))
                .title(" Files "),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(40, 80, 140))
                .fg(Color::Rgb(255, 255, 255))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    // Preview (right side)
    let preview = Paragraph::new(app.preview_content.as_str())
        .style(Style::default().fg(Color::Rgb(180, 200, 220)))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(70, 130, 200)))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .padding(Padding::horizontal(1))
                .title(" Preview "),
        )
        .wrap(Wrap { trim: false });

    // Footer
    let help_text = " ^j/^k: move | l/Enter: open | h: back | q: quit ";
    let help = Paragraph::new(help_text)
        .style(
            Style::default()
                .fg(Color::Rgb(150, 170, 200))
                .bg(Color::Rgb(25, 35, 55)),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::Rgb(70, 130, 200))),
        );

    f.render_widget(header, main_chunks[0]);
    f.render_stateful_widget(file_list, content_chunks[0], &mut app.state);
    f.render_widget(preview, content_chunks[1]);
    f.render_widget(help, main_chunks[2]);
}

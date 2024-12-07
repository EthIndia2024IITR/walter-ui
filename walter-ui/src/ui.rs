use ratatui::layout::Alignment;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn render_ui(frame: &mut Frame, app: &mut App) {
    let centered_rect = centered_rect(98, 98, frame.area());
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().green());
    frame.render_widget(main_block, centered_rect);

    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(4),
    ])
    .split(centered_rect);

    match app.current_screen {
        CurrentScreen::Dashboard => render_dashboard(frame, centered_rect),
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}


pub fn render_dashboard(frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(area);

    let title_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("██╗    ██╗ █████╗ ██╗  ████████╗███████╗██████╗ 
██║    ██║██╔══██╗██║  ╚══██╔══╝██╔════╝██╔══██╗
██║ █╗ ██║███████║██║     ██║   █████╗  ██████╔╝
██║███╗██║██╔══██║██║     ██║   ██╔══╝  ██╔══██╗
╚███╔███╔╝██║  ██║███████╗██║   ███████╗██║  ██║
 ╚══╝╚══╝ ╚═╝  ╚═╝╚══════╝╚═╝   ╚══════╝╚═╝  ╚═╝",
        Style::default().fg(Color::Green),
    ))
    .block(title_block)
    .alignment(Alignment::Center);

    let instructions_block = Block::default().style(Style::default());

    let instructions = Paragraph::new(Text::styled(
        "Press 'c' to start a new query\n\nPress 'q' to quit",
        Style::default().fg(Color::Yellow),
    ))
    .block(instructions_block)
    .alignment(Alignment::Center);

    frame.render_widget(title, chunks[1]);
    frame.render_widget(instructions, chunks[2]);
}


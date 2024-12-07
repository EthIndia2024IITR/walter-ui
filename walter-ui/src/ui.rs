use std::fmt::format;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{self, Line, Span, Text},
    widgets::{
        Bar, BarChart, BarGroup, Block, BorderType, Borders, Cell, Clear, HighlightSpacing, List,
        ListItem, Padding, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table, Tabs, Wrap,
    },
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn render_ui(frame: &mut Frame, app: &mut App) {
    let centered_rect = centered_rect(95, 95, frame.area());
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().green());
    frame.render_widget(main_block, centered_rect);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(4),
        ])
        .split(centered_rect);

    match app.current_screen {
        CurrentScreen::Splash => render_splash_screen(frame, app, chunks[1]),
        CurrentScreen::Dashboard => {
            frame.render_widget(
                Paragraph::new("").block(Block::bordered().title("~ [ Dashboard ] ~").title_alignment(Alignment::Center)),
                frame.area(),
            );
            render_dashboard(frame, app, chunks[1])
        }
        _ => {}
    }

    if app.should_quit {
        render_exit_popup(frame, centered_rect);
    }
}

fn render_splash_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(50),
            Constraint::Percentage(40),
        ])
        .split(area);

    let title_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "██╗    ██╗ █████╗ ██╗  ████████╗███████╗██████╗ 
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

    let center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(chunks[2]);

    let loaded_details = format!(
        "Active Sui Account \t{}\nActive Sui Env \t{}",
        app.sui_active_address, app.sui_active_env
    );
    let instructions = Paragraph::new(Text::styled(
        loaded_details,
        Style::default().fg(Color::Yellow),
    ))
    .block(instructions_block)
    .alignment(Alignment::Left)
    .style(Style::default().fg(Color::Cyan))
    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));

    frame.render_widget(title, chunks[1]);
    frame.render_widget(instructions, center[1]);
}

fn render_user_blobs(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let header_style = Style::default().fg(Color::LightGreen);
    let selected_style = Style::default().fg(Color::DarkGray).bg(Color::Yellow);

    if !app.user_blobs.is_empty() {
        let header = [
            "Blob ID",
            "Unencoded size",
            "Certified",
            "Deletable",
            "Expiry epoch",
            "Object ID",
        ]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(2);
        let rows = app.user_blobs.iter().enumerate().map(|(_i, data)| {
            let item = [
                data.blob_id.as_str(),
                &data.unencoded_size.to_string(),
                &data.is_certified.to_string(),
                &data.is_deletable.to_string(),
                &data.expiration_epoch.to_string(),
                data.object_id.as_str(),
            ];
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("{}", truncate(content)))))
                .collect::<Row>()
                .style(Style::new().fg(Color::Yellow))
                .height(1)
        });

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(40),
                Constraint::Percentage(12),
                Constraint::Percentage(10),
                Constraint::Percentage(8),
                Constraint::Percentage(10),
                Constraint::Percentage(20),
            ],
        )
        .header(header)
        .block(
            Block::bordered()
                .border_style(Style::new().green())
                .padding(Padding::horizontal(2)),
        )
        .row_highlight_style(selected_style)
        .highlight_spacing(HighlightSpacing::Always);

        let title = Text::styled(
            format!("~ [ Uploads by user {} ] ~", truncate(&app.sui_active_address)),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
        let title = Paragraph::new(title)
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center);


        let bottom_left = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);
        
        let system_info = Text::styled(
            &app.walrus_system_info,
            Style::default().fg(Color::Yellow),
        );
        let system_info = Paragraph::new(system_info)
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Left);


        frame.render_widget(system_info, bottom_left[0]);
        frame.render_stateful_widget(table, chunks[0], &mut app.table_state);
        frame.render_widget(title, chunks[0]);
        render_scrollbar(frame, app, chunks[0]);
    } else {
        let text = Text::from(format!("\n\n\nNo blobs found."));
        let paragraph = Paragraph::new(text).alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
    }
}

fn render_scrollbar(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .style(Style::new().green())
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 3,
            horizontal: 1,
        }),
        &mut app.scrollbar_state,
    );
}

fn render_dashboard(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(area);

    render_user_blobs(frame, app, chunks[0]);
    render_footer(frame, app, chunks[1]);
}

fn render_exit_popup(frame: &mut Frame, area: Rect) {
    let outer_rect = centered_rect(42, 32, area);
    let inner_rect = centered_rect(40, 30, area);
    frame.render_widget(Clear, outer_rect);

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Green).fg(Color::Black))
        .border_type(BorderType::Rounded)
        .title("Confirm Exit");

    let exit_text = Text::styled(
        "\n\nDo you really want to exit? \n\n [Y]es / [N]o",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );

    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    frame.render_widget(exit_paragraph, inner_rect);
}

fn render_footer(frame: &mut Frame, app: &mut App, area: Rect) {
    let instructions_block = Block::default().padding(Padding::vertical(1));
    let mut content = "";

    match app.current_screen {
        CurrentScreen::Splash => content = "Press 'Enter' to continue",
        CurrentScreen::Dashboard => content = "[Q]uit | [D]ownload a File | [U]pload a File (soon)",
        _ => {}
    }

    let instructions = Paragraph::new(Text::styled(content, Style::default().fg(Color::Green)))
        .block(instructions_block)
        .alignment(Alignment::Center);

    frame.render_widget(instructions, area);
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

fn truncate(content: &str) -> String {
    if content.len() >= 2 && content[..2] == *"0x" {
        format!(
            "{}...{}",
            &content[..8],
            &content[content.len() - 8..content.len()]
        )
    } else {
        content.to_string()
    }
}

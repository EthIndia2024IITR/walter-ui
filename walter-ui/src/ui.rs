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
                Paragraph::new("").block(
                    Block::bordered()
                        .title("~ [ Dashboard ] ~")
                        .title_alignment(Alignment::Center),
                ),
                frame.area(),
            );
            render_dashboard(frame, app, chunks[1])
        }
        CurrentScreen::Uploader => {
            frame.render_widget(
                Paragraph::new("").block(
                    Block::bordered()
                        .title("~ [ Uploader ] ~")
                        .title_alignment(Alignment::Center),
                ),
                frame.area(),
            );
            render_uploader(frame, app, chunks[1]);
        }
        CurrentScreen::Migrator => {
            frame.render_widget(
                Paragraph::new("").block(
                    Block::bordered()
                        .title("~ [ Migrate from IPFS ] ~")
                        .title_alignment(Alignment::Center),
                ),
                frame.area(),
            );

            render_migrator(frame, app, chunks[1]);
        }
        CurrentScreen::SharderAndEpochExtender => {
            frame.render_widget(
                Paragraph::new("").block(
                    Block::bordered()
                        .title("~ [ Sharder & Epoch Extender ] ~")
                        .title_alignment(Alignment::Center),
                ),
                frame.area(),
            );
            render_sharder_and_extender(frame, app, chunks[1]);
        }
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

    let title = Paragraph::new(
        "██╗    ██╗ █████╗ ██╗  ████████╗███████╗██████╗ 
██║    ██║██╔══██╗██║  ╚══██╔══╝██╔════╝██╔══██╗
██║ █╗ ██║███████║██║     ██║   █████╗  ██████╔╝
██║███╗██║██╔══██║██║     ██║   ██╔══╝  ██╔══██╗
╚███╔███╔╝██║  ██║███████╗██║   ███████╗██║  ██║
 ╚══╝╚══╝ ╚═╝  ╚═╝╚══════╝╚═╝   ╚══════╝╚═╝  ╚═╝",
    )
    .style(Style::default().fg(Color::Cyan))
    .alignment(Alignment::Center);

    let loaded_details = vec![
        Line::from(Span::styled(
            format!("Active Sui Account: {}\n", app.sui_active_address),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("Active Sui Env: {}", app.sui_active_env),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let details_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Session Details")
        .title_alignment(Alignment::Center)
        .padding(Padding::new(1, 1, 2, 2));

    let details_paragraph = Paragraph::new(loaded_details)
        .block(details_block)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left);

    let details_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(chunks[2])[1];

    frame.render_widget(title, chunks[1]);
    frame.render_widget(details_paragraph, details_area);
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
            format!(
                "~ [ Uploads by user {} ] ~",
                truncate(&app.sui_active_address)
            ),
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

        let system_info = Text::styled(&app.walrus_system_info, Style::default().fg(Color::Yellow));
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
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(area);

    render_user_blobs(frame, app, chunks[0]);
    render_footer(frame, app, chunks[1]);
}

fn render_updater(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(area);

    render_footer(frame, app, chunks[1]);
}

fn render_uploader(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(area);

    let screen = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let left = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(50),
        ])
        .split(screen[0]);

    let filename = Text::from(
        Line::from(format!("File path: {}", app.filename.clone()))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center),
    );

    frame.render_widget(filename, left[1]);

    let file_exists = std::path::Path::new(&app.filename).exists();
    if file_exists && !app.filename.is_empty() && !app.is_editing {
        let info = Text::from(Line::from("File exists").style(Style::default().fg(Color::Green)));
        frame.render_widget(info, left[1]);
    } else {
        let info =
            Text::from(Line::from("File does not exist").style(Style::default().fg(Color::Red)));
        frame.render_widget(info, left[1]);
    };

    render_footer(frame, app, chunks[1]);
}

fn render_migrator(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(area);

    let screen = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(chunks[0]);

    let text_field = Text::from(
        Line::from(format!(
            "Enter Pinata API Key: {}",
            app.pinata_api_key.clone()
        ))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center),
    );

    frame.render_widget(text_field, screen[0]);
    render_footer(frame, app, chunks[1]);
}

fn render_sharder_and_extender(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(area);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let sharder_area = content_chunks[0].inner(Margin {
        horizontal: 2,
        vertical: 2,
    });

    let extender_area = content_chunks[1].inner(Margin {
        horizontal: 2,
        vertical: 2,
    });

    let sharder_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(sharder_area);

    let extender_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(extender_area);

    let sharder_title = "Sharder";
    let sharder_content = format!("File to shard: {}", app.filename);
    let sharder_status = if app.sharder_status == "success" {
        Paragraph::new("Sharding succeeded").style(Style::default().fg(Color::Green))
    } else if app.sharder_status == "failure" {
        Paragraph::new("Sharding failed").style(Style::default().fg(Color::Red))
    } else {
        Paragraph::new("").style(Style::default().fg(Color::Yellow))
    };
    let sharder_block = Block::default()
        .borders(Borders::ALL)
        .title(sharder_title)
        .title_alignment(Alignment::Center);
    let sharder_paragraph = Paragraph::new(sharder_content).block(sharder_block);

    frame.render_widget(sharder_paragraph, sharder_chunks[0]);
    frame.render_widget(sharder_status, sharder_chunks[1]);

    let extender_title = "Epoch Extender";
    let extender_content = format!("BlobID to epoch extend: {}", app.extender_blob_id);
    let extender_status = if app.extender_status == "success" {
        Paragraph::new("Extension succeeded").style(Style::default().fg(Color::Green))
    } else if app.extender_status == "failure" {
        Paragraph::new("Extension failed").style(Style::default().fg(Color::Red))
    } else {
        Paragraph::new("").style(Style::default().fg(Color::Yellow))
    };
    let extender_block = Block::default()
        .borders(Borders::ALL)
        .title(extender_title)
        .title_alignment(Alignment::Center);
    let extender_paragraph = Paragraph::new(extender_content).block(extender_block);

    frame.render_widget(extender_paragraph, extender_chunks[0]);
    frame.render_widget(extender_status, extender_chunks[1]);

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
        CurrentScreen::Dashboard => {
            content = "[2] Uploader | [3] Migrate | [4] Sharder & Epoch Extender | [Q]uit"
        }
        CurrentScreen::Uploader => content =
            "[1] Dashboard | [Enter] Upload | [3] Migrator | [4] Sharder & Epoch Extender | [Q]uit",
        CurrentScreen::Migrator => {
            content =
                "[1] Dashboard | [2] Uploader | [M]igrate | [4] Sharder & Epoch Extender | [Q]uit"
        }
        CurrentScreen::SharderAndEpochExtender => content =
            "[1] Dashboard | [2] Uploader | [3] Migrator | [K] Shard | [E]ncrypt? | Epoch Ex[T]end | [Q]uit",
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

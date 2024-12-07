use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, Screen};

pub fn render(frame: &mut Frame, app: &mut App) {
    // Create a layout with a sidebar and main content area
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Sidebar width
            Constraint::Percentage(80)  // Main content width
        ])
        .split(frame.size());

    // Render sidebar with menu
    render_sidebar(frame, app, layout[0]);

    // Render current screen
    match app.current_screen {
        Screen::Home => render_home_screen(frame, layout[1]),
        Screen::Storage => render_storage_screen(frame, layout[1]),
        Screen::Nodes => render_nodes_screen(frame, layout[1]),
        Screen::Settings => render_settings_screen(frame, layout[1]),
    }
}

fn render_sidebar(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    // Create menu items
    let items: Vec<ListItem> = app.menu_items
        .iter()
        .map(|&item| {
            ListItem::new(Span::from(item.to_string()))
                .style(Style::default().fg(Color::White))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Walrus Menu").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Yellow)
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.list_state.clone());
}

fn render_home_screen(frame: &mut Frame, area: ratatui::layout::Rect) {
    let block = Block::default()
        .title("Walrus - Home")
        .borders(Borders::ALL);

    let text = vec![
        Span::styled("Welcome to Walrus Blockchain Storage\n\n", Style::default().fg(Color::Cyan)),
        Span::raw("Use "),
        Span::styled("H", Style::default().fg(Color::Yellow)),
        Span::raw(" and "),
        Span::styled("L", Style::default().fg(Color::Yellow)),
        Span::raw(" to navigate screens\n"),
        Span::raw("Use "),
        Span::styled("Up", Style::default().fg(Color::Yellow)),
        Span::raw(" and "),
        Span::styled("Down", Style::default().fg(Color::Yellow)),
        Span::raw(" to select menu items\n"),
        Span::raw("Press "),
        Span::styled("Enter", Style::default().fg(Color::Yellow)),
        Span::raw(" to select a screen\n"),
        Span::raw("Press "),
        Span::styled("Q", Style::default().fg(Color::Yellow)),
        Span::raw(" to quit"),
    ];

    let paragraph = Paragraph::new(Line::from(text))
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn render_storage_screen(frame: &mut Frame, area: ratatui::layout::Rect) {
    let block = Block::default()
        .title("Walrus - Storage Management")
        .borders(Borders::ALL);

    let text = vec![
        Span::styled("Storage Management Placeholder\n\n", Style::default().fg(Color::Green)),
        Span::raw("Manage blockchain storage operations\n"),
        Span::raw("- View current storage usage\n"),
        Span::raw("- Configure storage parameters\n"),
        Span::raw("- Backup and restore storage"),
    ];

    let paragraph = Paragraph::new(Line::from(text))
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn render_nodes_screen(frame: &mut Frame, area: ratatui::layout::Rect) {
    let block = Block::default()
        .title("Walrus - Node Management")
        .borders(Borders::ALL);

    let text = vec![
        Span::styled("Node Management Placeholder\n\n", Style::default().fg(Color::Magenta)),
        Span::raw("Manage blockchain network nodes\n"),
        Span::raw("- List connected nodes\n"),
        Span::raw("- Add/Remove nodes\n"),
        Span::raw("- View node performance"),
    ];

    let paragraph = Paragraph::new(Line::from(text))
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn render_settings_screen(frame: &mut Frame, area: ratatui::layout::Rect) {
    let block = Block::default()
        .title("Walrus - Settings")
        .borders(Borders::ALL);

    let text = vec![
        Span::styled("Application Settings Placeholder\n\n", Style::default().fg(Color::Blue)),
        Span::raw("Configure Walrus TUI\n"),
        Span::raw("- Theme settings\n"),
        Span::raw("- Network preferences\n"),
        Span::raw("- Logging and debugging"),
    ];

    let paragraph = Paragraph::new(Line::from(text))
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
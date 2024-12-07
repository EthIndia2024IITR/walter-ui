use ratatui::{
    layout::Rect,
    Frame,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Walrus - Home")
        .borders(Borders::ALL);

    let text = vec![
        Span::styled("Welcome to Walrus Blockchain Storage\n\n", Style::default().fg(Color::Cyan)),
        Span::raw("Walrus is a cutting-edge blockchain storage solution designed for:\n"),
        Span::raw("- Efficient data management\n"),
        Span::raw("- Secure distributed storage\n"),
        Span::raw("- High-performance blockchain infrastructure\n\n"),
        Span::styled("Quick Navigation:\n", Style::default().fg(Color::Yellow)),
        Span::raw("• Use "),
        Span::styled("H/L", Style::default().fg(Color::Green)),
        Span::raw(" to switch screens\n"),
        Span::raw("• Use "),
        Span::styled("Up/Down", Style::default().fg(Color::Green)),
        Span::raw(" to navigate menu\n"),
        Span::raw("• Press "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw(" to select\n"),
        Span::raw("• Press "),
        Span::styled("Q", Style::default().fg(Color::Red)),
        Span::raw(" to quit"),
    ];

    let paragraph = Paragraph::new(Line::from(text))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
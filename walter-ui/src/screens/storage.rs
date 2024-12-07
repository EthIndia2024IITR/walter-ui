use ratatui::{
    layout::Rect,
    Frame,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct StorageState {
    pub total_storage: u64,
    pub used_storage: u64,
    pub available_storage: u64,
    pub storage_nodes: Vec<String>,
}

impl Default for StorageState {
    fn default() -> Self {
        Self {
            total_storage: 1024 * 1024 * 1024, // 1TB
            used_storage: 256 * 1024 * 1024,   // 256GB used
            available_storage: 768 * 1024 * 1024, // 768GB available
            storage_nodes: vec![
                "node-01.walrus.network".to_string(),
                "node-02.walrus.network".to_string(),
                "node-03.walrus.network".to_string(),
            ],
        }
    }
}

pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Walrus - Storage Management")
        .borders(Borders::ALL);

    let storage_state = StorageState::default();

    let total_storage = format!("• Total Storage: {:.2} GB\n", (storage_state.total_storage as f64) / (1024.0 * 1024.0 * 1024.0));
    let used_storage = format!("• Used Storage: {:.2} GB\n", storage_state.used_storage as f64 / (1024.0 * 1024.0 * 1024.0));
    let available_storage = format!("• Available Storage: {:.2} GB\n", storage_state.available_storage as f64 / (1024.0 * 1024.0 * 1024.0));
    let storage_nodes = storage_state.storage_nodes.join("\n");

    let text = vec![
        Span::styled("Storage Management\n\n", Style::default().fg(Color::Cyan)),
        Span::raw("Storage Overview:\n"),
        Span::raw(&total_storage),
        Span::raw(&used_storage),
        Span::raw(&available_storage),
        Span::styled("\nStorage Nodes:\n", Style::default().fg(Color::Yellow)),
        Span::raw(&storage_nodes),
    ];

    let paragraph = Paragraph::new(Line::from(text))
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

mod app;
mod ui;
mod utils;

use app::{App, CurrentScreen};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::ScrollbarState,
};
use serde_json;
use std::fs::File;
use std::{
    error::Error,
    io::{self, BufWriter, Stdout, Write},
};
use ui::render_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let sui_active_env = utils::sui_active_env().await?;
    app.sui_active_env = sui_active_env;

    let sui_active_address = utils::sui_active_address().await?;
    app.sui_active_address = sui_active_address;

    let user_blobs = utils::walrus_list_blobs().await?;
    let user_blobs = serde_json::from_str(&user_blobs)?;
    app.user_blobs = user_blobs;

    let _res = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| render_ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            if !app.should_quit {
                match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    _ => {}
                }
            }

            if app.should_quit {
                match key.code {
                    KeyCode::Char('y') => return Ok(true),
                    KeyCode::Char('n') => app.should_quit = false,
                    _ => {}
                }
            }

            match app.current_screen {
                CurrentScreen::Splash => match key.code {
                    KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Dashboard;
                        if !&app.user_blobs.is_empty() {
                            app.scrollbar_state = ScrollbarState::new(&app.user_blobs.len() - 1);
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Dashboard => match key.code {
                    KeyCode::Char('c') => {}
                    KeyCode::Up => {
                        app.prev_row();
                    }
                    KeyCode::Down => {
                        app.next_row();
                    }
                    _ => {}
                },
                CurrentScreen::Update => todo!(),
            }
        }
    }
}

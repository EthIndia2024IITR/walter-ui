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
    let sui_active_env = sui_active_env.trim().to_string();
    app.sui_active_env = sui_active_env;

    let sui_active_address = utils::sui_active_address().await?;
    let sui_active_address = sui_active_address.trim().to_string();
    app.sui_active_address = sui_active_address;

    let user_blobs = utils::walrus_list_blobs().await?;
    let user_blobs = serde_json::from_str(&user_blobs)?;
    app.user_blobs = user_blobs;

    let walrus_system_info = utils::walrus_info_system().await?;
    let walrus_system_info = walrus_system_info.trim().to_string();
    app.walrus_system_info = walrus_system_info;

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
            if key.kind == event::KeyEventKind::Press {
                if key.code == KeyCode::Esc {
                    if app.is_editing {
                        app.is_editing = false;
                    }
                }
                if key.code == KeyCode::Char('e') || key.code == KeyCode::Char('E') {
                    if !app.is_editing {
                        app.is_editing = true;
                        continue;
                    }
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

                if key.code == KeyCode::Char('1') {
                    app.current_screen = CurrentScreen::Dashboard;
                }

                if key.code == KeyCode::Char('2') {
                    app.current_screen = CurrentScreen::Uploader;
                }

                if key.code == KeyCode::Char('3') {
                    app.current_screen = CurrentScreen::Migrator;
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
                CurrentScreen::Uploader => match key.code {
                    KeyCode::Char(value) => {
                        if app.is_editing {
                            app.filename += &value.to_string();
                        }
                    }
                    KeyCode::Backspace => {
                        if app.is_editing {
                            if app.filename.len() > 0 {
                                app.filename.pop();
                            }
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Migrator => match key.code {
                    KeyCode::Char('v') => {
                        // yahan dalna key
                        app.pinata_api_key = "API KEY COMES HERE".into();
                    },
                    KeyCode::Char('x') => {
                        app.pinata_api_key = "".into();
                    },
                    KeyCode::Char('M') => {
                        // yahan karo migration
                    }
                    _ => {},
                },
                CurrentScreen::SharderAndEpochExtender => {
                    
                }
            }
        }
    }
}

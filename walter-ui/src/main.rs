use serde_json::json;
use std::{error::Error, process::exit};

// use ratatui::{
//     backend::Backend,
//     crossterm::event::{self, Event, KeyCode, KeyEventKind},
//     Terminal,
// };

use std::io::Write;
use std::process::{Command, Stdio};

mod app;
mod ui;
mod utils;

use utils::{StoreCommand, StoreDetails, WalrusCommand, WalrusResponse, run_walrus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let store_cmd = WalrusCommand {
        // config: "/path/to/client_config.yaml".to_string(),
        command: StoreCommand {
            store: StoreDetails {
                file: "test.txt".to_string(),
                epochs: 1,
            },
        },
    };

    let json_cmd = serde_json::to_string(&store_cmd)?;

    let walrus_output = run_walrus(json_cmd).await?;
    println!("{}", walrus_output);

    exit(0);

    // enable_raw_mode()?;
    // let mut stdout = io::stdout(); // This is a special case. Normally using stdout is fine
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;

    // // create app and run it
    // let mut app = App::new();
    // let res = run_app(&mut terminal, &mut app).await;

    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;

    // if let Ok(do_print) = res {
    //     if do_print {
    //         app.print_json()?;
    //     }
    // } else if let Err(err) = res {
    //     println!("{err:?}");
    // }

    /*

    { "command: "list-blobs" }

    */

    // Ok(())
}

// async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
//     loop {
//         terminal.draw(|f| ui(f, app))?;
//         if let Event::Key(key) = event::read()? {
//             if key.kind == event::KeyEventKind::Release {
//                 // Skip events that are not KeyEventKind::Press
//                 continue;
//             }
//             match app.current_screen {
//                 CurrentScreen::Main => match key.code {
//                     KeyCode::Char('e') => {
//                         app.current_screen = CurrentScreen::Editing;
//                         app.currently_editing = Some(CurrentlyEditing::Key);
//                     }
//                     KeyCode::Char('q') => {
//                         app.current_screen = CurrentScreen::Exiting;
//                     }
//                     _ => {}
//                 },
//                 CurrentScreen::Exiting => match key.code {
//                     KeyCode::Char('y') => {
//                         return Ok(true);
//                     }
//                     KeyCode::Char('n') | KeyCode::Char('q') => {
//                         return Ok(false);
//                     }
//                     _ => {}
//                 },
//                 CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
//                     KeyCode::Enter => {
//                         if let Some(editing) = &app.currently_editing {
//                             match editing {
//                                 CurrentlyEditing::Key => {
//                                     app.currently_editing = Some(CurrentlyEditing::Value);
//                                 }
//                                 CurrentlyEditing::Value => {
//                                     app.save_key_value();
//                                     app.current_screen = CurrentScreen::Main;
//                                 }
//                             }
//                         }
//                     }
//                     KeyCode::Backspace => {
//                         if let Some(editing) = &app.currently_editing {
//                             match editing {
//                                 CurrentlyEditing::Key => {
//                                     app.key_input.pop();
//                                 }
//                                 CurrentlyEditing::Value => {
//                                     app.value_input.pop();
//                                 }
//                             }
//                         }
//                     }
//                     KeyCode::Esc => {
//                         app.current_screen = CurrentScreen::Main;
//                         app.currently_editing = None;
//                     }
//                     KeyCode::Tab => {
//                         app.toggle_editing();
//                     }
//                     KeyCode::Char(value) => {
//                         if let Some(editing) = &app.currently_editing {
//                             match editing {
//                                 CurrentlyEditing::Key => {
//                                     app.key_input.push(value);
//                                 }
//                                 CurrentlyEditing::Value => {
//                                     app.value_input.push(value);
//                                 }
//                             }
//                         }
//                     }
//                     _ => {}
//                 },
//                 _ => {}
//             }
//         }
//     }
// }

pub mod commands;
pub mod permissions;
pub mod utils;

use commands::execute_instructions;
use permissions::check_and_request_sudo;

pub fn run() {
    if !check_and_request_sudo() {
        eprintln!("Failed to obtain sudo permissions. Exiting...");
        return;
    }

    if let Err(e) = execute_instructions() {
        eprintln!("Error executing instructions: {}", e);
    }
}

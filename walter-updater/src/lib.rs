pub mod commands;  
pub mod permissions;  
pub mod utils;  

use commands::execute_instructions;  
use permissions::check_and_request_sudo;  

/// Main function to execute the library's functionality  
pub fn run() {  
    // Check and request sudo permissions  
    if !check_and_request_sudo() {  
        eprintln!("Failed to obtain sudo permissions. Exiting...");  
        return;  
    }  

    // Execute the instructions  
    if let Err(e) = execute_instructions() {  
        eprintln!("Error executing instructions: {}", e);  
    }  
}  
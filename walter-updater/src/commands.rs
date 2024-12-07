use std::process::Command;  
use std::io::{self, Write};  

/// Function to execute the given instructions  
pub fn execute_instructions() -> Result<(), Box<dyn std::error::Error>> {  
    // Define the system variable  
    let system = get_system_variable()?;  

    // Download the binary using curl  
    let curl_command = format!(  
        "curl https://storage.googleapis.com/mysten-walrus-binaries/walrus-testnet-latest-{} -o walrus",  
        system  
    );  
    println!("Executing: {}", curl_command);  
    run_command(&curl_command)?;  

    // Make the binary executable  
    let chmod_command = "chmod +x walrus";  
    println!("Executing: {}", chmod_command);  
    run_command(chmod_command)?;  

    println!("Instructions executed successfully.");  
    Ok(())  
}  

/// Helper function to determine the system variable  
fn get_system_variable() -> Result<String, io::Error> {  
    print!("Enter your system (e.g., ubuntu-x86_64, ubuntu-x86_64-generic, macos-x86_64, macos-arm64, windows-x86_64.exe): ");  
    io::stdout().flush()?; // Ensure the prompt is displayed  
    let mut system = String::new();  
    io::stdin().read_line(&mut system)?;  
    Ok(system.trim().to_string())  
}  

/// Helper function to run a shell command  
fn run_command(command: &str) -> Result<(), Box<dyn std::error::Error>> {  
    let status = Command::new("sh")  
        .arg("-c")  
        .arg(command)  
        .status()?;  

    if !status.success() {  
        return Err(format!("Command failed: {}", command).into());  
    }  
    Ok(())  
}  
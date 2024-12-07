use sudo::escalate_if_needed;

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Function to execute the given instructions  
fn execute_instructions() -> Result<(), Box<dyn std::error::Error>> {
    // Define the system variable
    let system = get_system_variable()?;

    // Download the binary using curl
    let download_path = "./walrus"; // Temporary download location
    let curl_command = format!(
        "curl https://storage.googleapis.com/mysten-walrus-binaries/walrus-testnet-latest-{} -o {}",
        system, download_path
    );
    println!("Downloading walrus binary...");
    run_command(&curl_command)?;

    // Make the binary executable
    println!("Making the binary executable...");
    run_command(&format!("chmod +x {}", download_path))?;

    // Find the existing installation path using `which walrus`
    let install_path =
        find_existing_walrus_path().unwrap_or_else(|| "/usr/local/bin/walrus".to_string());

    // Replace the existing installation
    if Path::new(&install_path).exists() {
        println!(
            "Existing walrus installation found at {}. Replacing it...",
            install_path
        );
        fs::remove_file(&install_path)?; // Remove the old binary
    } else {
        println!("No existing walrus installation found. Installing new binary...");
    }

    // Move the new binary to the installation path
    fs::rename(download_path, &install_path)?;
    println!("Walrus binary successfully updated at {}", install_path);

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

/// Helper function to find the existing installation path of walrus  
fn find_existing_walrus_path() -> Option<String> {
    let output = Command::new("which").arg("walrus").output().ok()?; // Run `which walrus` and capture the output

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !path.is_empty() {
            return Some(path);
        }
    }
    None
}

/// Helper function to run a shell command  
fn run_command(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("sh").arg("-c").arg(command).status()?;

    if !status.success() {
        return Err(format!("Command failed: {}", command).into());
    }
    Ok(())
}

fn check_and_request_sudo() -> bool {
    match escalate_if_needed() {
        Ok(_) => {
            println!("Sudo permissions granted.");
            true
        }
        Err(e) => {
            eprintln!("Failed to obtain sudo permissions: {}", e);
            false
        }
    }
}

pub fn run() {
    if !check_and_request_sudo() {
        eprintln!("Failed to obtain sudo permissions. Exiting...");
        return;
    }

    if let Err(e) = execute_instructions() {
        eprintln!("Error executing instructions: {}", e);
    }
}

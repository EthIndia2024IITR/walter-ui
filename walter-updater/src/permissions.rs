use sudo::escalate_if_needed;  

/// Function to check and request sudo permissions  
pub fn check_and_request_sudo() -> bool {  
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
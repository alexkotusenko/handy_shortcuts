//! Shortcuts for working with different operating systems

pub fn unless_unix<T: Fn() -> ()>(fallback: T) {
    let os = std::env::consts::OS;
    if os != "macos" && os != "linux" {
        fallback();
    }
}

pub fn is_unix() -> bool {
    let os = std::env::consts::OS;
    if os == "macos" || os == "linux" {
        return true;
    }
    false
}

pub fn get_unix_home_folder() -> Result<String, String> {
    if !is_unix() {
        return Err("The OS is not Unix-based".to_string());
    }
    match std::env::var("HOME") {
        Ok(path) => {
            return Ok(path); 
        }
        Err(_) => {
            return Err("Failed to determine the home directory.".to_string());
        }
    }
}

pub fn get_unix_user() -> Result<String, String> {
    if !is_unix() {
        return Err("The OS is not Unix-based".to_string());
    }
    match std::env::var("USER") {
        Ok(user) => {
            return Ok(user); 
        }
        Err(_) => {
            return Err("Failed to determine the current user.".to_string());
        }
    }
}

pub fn get_unix_shell() -> Result<String, String> {
    if !is_unix() {
        return Err("The OS is not Unix-based".to_string());
    }
    match std::env::var("SHELL") {
        Ok(user) => {
            return Ok(user); 
        }
        Err(_) => {
            return Err("Failed to determine the shell used.".to_string());
        }
    }
}
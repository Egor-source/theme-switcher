use std::env::temp_dir;
use std::fs;
use super::constants::FILES_NAMES;

pub fn get_daemon_pid() -> Option<i32> {
    let pid_file = temp_dir().join(format!("{FILES_NAMES}.pid"));

    let pid_content = match fs::read_to_string(pid_file) {
        Ok(content) => content,
        Err(_) => return None,
    };

    match pid_content.trim().parse::<i32>() {
        Ok(pid)=>Some(pid),
        Err(_) => None
    }
}
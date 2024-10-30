use std::env::temp_dir;
use std::{fs, io};
use std::process::exit;
use libc::{kill, SIGTERM};
use super::constants::FILES_NAMES;
use super::utils::get_daemon_pid;

pub fn stop(){
    let pid = match get_daemon_pid() {
        Some(pid) => pid,
        None => {
            eprintln!("Theme switcher was not launched");
            exit(1);
        }
    };

    match kill_process(pid) {
        Ok(_) => {
            println!("Theme Switcher stopped successfully");
        },
        Err(e) => {
            eprintln!("Error during stop Theme Switcher: {}", e);
            exit(1);
        },
    }

    fs::remove_file(temp_dir().join(format!("{FILES_NAMES}.pid"))).unwrap();
    fs::remove_file(temp_dir().join(format!("{FILES_NAMES}.err"))).unwrap();
}

fn kill_process(pid: i32) -> io::Result<()> {
    unsafe {
        if kill(pid, SIGTERM) != 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}
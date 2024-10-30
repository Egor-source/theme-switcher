use std::env::consts::OS;
use std::{env, fs, io};
use std::path::PathBuf;
use std::process::exit;

const LINUX_AUTOSTART_PATH: &str = ".config/autostart/my_app_start.desktop";
const MACOS_AUTOSTART_PATH: &str = "Library/LaunchAgents/com.example.myapp.plist";
pub fn uninstall(){
    match OS {
        "linux" => uninstall_linux_autostart().unwrap(),
        "macos" => uninstall_macos_autostart().unwrap(),
        _ => {
            eprintln!("{}",io::Error::new(io::ErrorKind::Unsupported, "Unsupported OS"));
            exit(1);
        },
    };
}

fn uninstall_linux_autostart() -> io::Result<()> {
    let autostart_path = PathBuf::from(env::var("HOME").unwrap()).join(LINUX_AUTOSTART_PATH);
    remove_autostart_file(autostart_path)
}

fn uninstall_macos_autostart() -> io::Result<()> {
    let autostart_path = PathBuf::from(env::var("HOME").unwrap()).join(MACOS_AUTOSTART_PATH);
    remove_autostart_file(autostart_path)
}

fn remove_autostart_file(path: PathBuf) -> io::Result<()> {
    if path.exists() {
        fs::remove_file(&path)?;
        println!("Autostart file removed from: {}", path.display());
    } else {
        println!("Autostart file not found at: {}", path.display());
    }
    Ok(())
}
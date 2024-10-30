use std::collections::HashSet;
use std::env::temp_dir;
use std::fs::File;
use std::process::{exit, Command};
use std::sync::{Arc, Mutex};
use rdev::{listen, Event, EventType, Key};
use daemonize::Daemonize;
use super::constants::FILES_NAMES;

fn change_theme(is_light: &mut bool) {
    #[cfg(target_os = "linux")]
    {
        let theme = match is_light  {
            true => "Adwaita",
            false => "Adwaita-dark",
        };

        Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.interface")
            .arg("gtk-theme")
            .arg(theme)
            .spawn()
            .expect("Failed to enable dark mode on GNOME");
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("osascript")
            .arg("-e")
            .arg(format!("tell application \"System Events\" to tell appearance preferences to set dark mode to {}", (!*is_light).to_string()))
            .spawn()
            .expect("Failed to enable dark mode on macOS");
    }

    *is_light = !*is_light;
}

pub fn start() {
    let temp_dir = temp_dir();
    let daemon = Daemonize::new()
        .pid_file(format!("{FILES_NAMES}.pid"))
        .chown_pid_file(true)
        .umask(0o775)
        .working_directory(&temp_dir)
        .stdout(File::open("/dev/null").unwrap())
        .stderr(File::create(temp_dir.join(format!("{FILES_NAMES}.err"))).unwrap());

    println!("Theme Switcher starting...");

    if let Err(e) = daemon.start() {
        eprintln!("Failed to start Theme Switcher: {e}");
        exit(1);
    }

    let mut is_light = true;

    let pressed_keys = Arc::new(Mutex::new(HashSet::new()));

    let pressed_keys_clone = Arc::clone(&pressed_keys);

    let callback = move |event: Event| {
        let mut pressed_keys = pressed_keys_clone.lock().unwrap();
        match event.event_type {
            EventType::KeyPress(key) => {
                pressed_keys.insert(key.clone());
                if pressed_keys.len() == 3 && pressed_keys.contains(&Key::ControlLeft) && pressed_keys.contains(&Key::ShiftLeft) && pressed_keys.contains(&Key::Num1) {
                   change_theme(&mut is_light);
                }
            },
            EventType::KeyRelease(key) => {
                pressed_keys.remove(&key);
            },
            _ => (),
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
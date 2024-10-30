use std::{env, fs, io};
use std::env::consts::OS;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use super::start as run_start;
use crate::utils::get_daemon_pid;

const LINUX_TEMPLATE: &str = include_str!("../autostart/linux.desktop");
const MACOS_TEMPLATE: &str = include_str!("../autostart/macos.plist");
pub fn install(start:Option<&bool>) {
   match OS {
      "linux" => setup_linux_autostart().unwrap(),
      "macos" => setup_macos_autostart().unwrap(),
      _ => {
         eprintln!("{}", io::Error::new(io::ErrorKind::Unsupported, "Unsupported OS"));
         exit(1);
      },
   };

   println!("Theme Switcher installed successfully");

   if let Some(start) = start {
      let pid = get_daemon_pid();
      match pid {
         Some(_) if *start => eprintln!("Theme Switcher has already started"),
         None if *start => run_start(),
         _ => {}
      }
   }
}

fn setup_linux_autostart() -> io::Result<()> {
   let autostart_path = PathBuf::from(format!("{}/.config/autostart/my_app_start.desktop", env::var("HOME").unwrap()));
   let start_command = format!("sudo {} start", env::current_exe()?.display());
   let content = LINUX_TEMPLATE.replace("{{EXEC}}", &start_command);

   write_autostart_file(autostart_path, &content)
}

fn setup_macos_autostart() -> io::Result<()> {
   let autostart_path = PathBuf::from(format!("{}/Library/LaunchAgents/com.example.myapp.plist", env::var("HOME").unwrap()));
   let start_command = format!("sudo {} start", env::current_exe()?.display());
   let content = MACOS_TEMPLATE.replace("{{EXEC}}", &start_command);

   write_autostart_file(autostart_path, &content)
}

fn write_autostart_file(path: PathBuf, content: &str) -> io::Result<()> {
   let already_installed = fs::exists(&path)?;

   if already_installed {
      println!("Theme Switcher already installed");
      exit(1);
   }

   let mut file = File::create(path)?;
   file.write_all(content.as_bytes())?;
   Ok(())
}
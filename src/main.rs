mod commands;
mod constants;
mod utils;

use std::process::exit;
use clap::{Arg, ArgAction, Command};
use commands::{start, stop, install};
use crate::commands::uninstall;

fn main() {
    let matches = Command::new("Theme Switcher")
        .subcommand_required(true)
        .disable_help_subcommand(true)
        .subcommands([
            Command::new("start").about("Start Theme Switcher"),
            Command::new("stop").about("Stop Theme Switcher"),
            Command::new("install").about("Install Theme Switcher").arg(
                Arg::new("start")
                    .short('s')
                    .long("start")
                    .help("Start Theme Switcher after install")
                    .action(ArgAction::SetTrue)
            ),
            Command::new("uninstall").about("Uninstall Theme Switcher"),
        ]).get_matches();

    match matches.subcommand() {
        Some(("start", _)) => {
            start();
        }
        Some(("stop", _)) => {
            stop();
        }
        Some(("install", args)) => {
            let start = args.get_one::<bool>("start");
            install(start);
        }
        Some(("uninstall", _)) => {
            uninstall();
        }
        _ => {
            eprintln!("Subcommand not found");
            exit(1);
        }
    }
}

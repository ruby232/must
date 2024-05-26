use clap::{Arg, ArgAction, ArgMatches};
use symphonia::core::errors::{Result};
use log::{error};
use crate::commands::Command;

mod server;
mod client;
mod play;

pub mod output;
mod commands;

fn main() {
    pretty_env_logger::init();
    let args = clap::Command::new("Must")
        .version("0.1")
        .author("Ruby <ruby232@gmail.com>")
        .about("A simple music player for the terminal.")
        .arg(
            Arg::new("server")
                .short('s')
                .long("server")
                .action(ArgAction::SetTrue)
                .help("Starts the server")
                .conflicts_with_all(&["play_pause"]),
        )
        .arg(
            Arg::new("play_pause")
                .short('p')
                .long("play-pause")
                .action(ArgAction::SetTrue)
                .help("Play/pause the music")
                .conflicts_with_all(&["server"]),
        )
        .get_matches();

    let code = run(&args).unwrap_or_else(|err| {
        error!("{}", err.to_string());
        -1
    });

    std::process::exit(code);
}

fn run(args: &ArgMatches) -> Result<i32> {
    if args.get_flag("server") {
        println!("Starting server ...");
        server::main()?;
        return Ok(0);
    }

    let mut command: Command = Command::None;

    if args.get_flag("play_pause") {
        command = Command::PlayPause;
    }

    if let Command::None = command {
        error!("Command not found");
        return Ok(1);
    }

    client::send(command)?;

    Ok(0)
}
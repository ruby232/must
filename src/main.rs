use std::env;
use clap::{Arg, ArgAction, ArgMatches};
use symphonia::core::errors::{Error, Result};
use log::error;

mod server;
mod client;
mod play;

pub mod output;

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
    if args.contains_id("server") {
        server::main()?;
        return Ok(0)
    }

    if args.contains_id("play_pause") {
        // @Todo: Implement play/pause
        println!("Todo: Playing music");
       // play::play();
    }

    Ok(0)
}
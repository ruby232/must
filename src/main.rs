use clap::{Arg, ArgAction, ArgMatches, Command};
use symphonia::core::errors::{Result};
use log::{error};
use crate::task_manager::{Task, TaskType};

mod server;
mod client;
mod play;

pub mod output;
pub mod task_manager;

fn main() {
    pretty_env_logger::init();
    let args = Command::new("Must")
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
        ).arg(
        Arg::new("play_file")
            .short('f')
            .long("play-file")
            .help("Play the audio file")
            .conflicts_with_all(&["server", "play_pause"]),
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

    let mut task: Task = Task {
        kind: TaskType::NotFound,
        arg: None,
    };


    if args.get_flag("play_pause") {
        task.kind = TaskType::PlayPause;
    }

    if let Some(file) = args.get_one::<String>("play_file") {
        task.kind = TaskType::PlayFile;
        task.arg = Some(file.to_string());
    }

    if task.is_not_found() {
        error!("Command not found");
        return Ok(1);
    }

    client::send(task)?;
    Ok(0)
}
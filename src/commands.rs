pub enum Command {
    StartServer,
    PlayPause,
    Quit,
    None,
}

impl Command {
    pub fn to_string(&self) -> String {
        match self {
            Command::StartServer => String::from("s"),
            Command::PlayPause =>  String::from("p"),
            Command::Quit =>  String::from("q"),
            Command::None =>  String::from("none"),
        }
    }
}

impl Command {
    pub fn from_string(s: &str) -> Command {
        match s {
            "s" => Command::StartServer,
            "p" => Command::PlayPause,
            "q" => Command::Quit,
            _ => Command::None,
        }
    }
}

pub fn execute_command(command_str: String) -> String {
    let command = Command::from_string(command_str.trim());

    match command {
        Command::StartServer => {
            // Por ahora no se va a usar
            return "Starting server ...".to_string();
        },
        Command::PlayPause => {
            //
            return "PlayPause ...".to_string();
        },
        Command::Quit => {
            return "Quit server ...".to_string();
        },
        Command::None => {
            return "Notin to do.".to_string();
        },
    }
}
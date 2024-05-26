use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq)]
pub enum TaskType {
    StartServer,
    PlayPause,
    Quit,
    PlayFile,
    NotFound,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub kind: TaskType,
    pub arg: Option<String>,
}

impl Task {
    pub fn run(&self) -> String {
        match self.kind {
            TaskType::StartServer => {
                "Starting server".to_string()
            }
            TaskType::PlayPause => {
                "Play/Pause".to_string()
            }
            TaskType::Quit => {
                "Quitting".to_string()
            }
            TaskType::PlayFile => {
                format!("Playing file: {}", self.arg.as_ref().unwrap())
            }
            TaskType::NotFound => {
                "Task not found".to_string()
            }
        }
    }
}

impl Task {
    pub(crate) fn is_not_found(&self) -> bool {
        &TaskType::NotFound == &self.kind
    }
}
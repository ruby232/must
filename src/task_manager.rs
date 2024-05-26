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
        return String::from("Task run");
    }
}

impl Task {
    pub(crate) fn is_not_found(&self) -> bool {
        &TaskType::NotFound == &self.kind
    }
}
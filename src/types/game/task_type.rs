#[derive(Debug)]
pub enum TaskType {
    Monsters,
    Items,
}

impl ToString for TaskType {
    fn to_string(&self) -> String {
        match self {
            TaskType::Monsters => "monsters".to_string(),
            TaskType::Items => "items".to_string(),
        }
    }
}

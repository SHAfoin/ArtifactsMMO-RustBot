use core::fmt;

#[derive(Debug)]
pub enum EventType {
    Monster,
    Resource,
    Workshop,
    Bank,
    GrandExchange,
    Tasks,
    Master,
    Npc,
}

impl EventType {
    pub fn as_str(&self) -> &str {
        match self {
            EventType::Monster => "monster",
            EventType::Resource => "resource",
            EventType::Workshop => "workshop",
            EventType::Bank => "bank",
            EventType::GrandExchange => "grand_exchange",
            EventType::Tasks => "tasks",
            EventType::Master => "master",
            EventType::Npc => "npc",
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

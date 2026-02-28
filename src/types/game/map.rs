#[derive(Debug)]
pub enum MapContentType {
    Monster,
    Resource,
    Workshop,
    Bank,
    GrandExchange,
    TasksMaster,
    Npc,
}

impl ToString for MapContentType {
    fn to_string(&self) -> String {
        match self {
            MapContentType::Monster => "monster".to_string(),
            MapContentType::Resource => "resource".to_string(),
            MapContentType::Workshop => "workshop".to_string(),
            MapContentType::Bank => "bank".to_string(),
            MapContentType::GrandExchange => "grand_exchange".to_string(),
            MapContentType::TasksMaster => "tasks_master".to_string(),
            MapContentType::Npc => "npc".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum MapLayerType {
    Interior,
    Overworld,
    Underground,
}

impl ToString for MapLayerType {
    fn to_string(&self) -> String {
        match self {
            MapLayerType::Interior => "interior".to_string(),
            MapLayerType::Overworld => "overworld".to_string(),
            MapLayerType::Underground => "underground".to_string(),
        }
    }
}

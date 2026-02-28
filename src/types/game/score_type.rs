#[derive(Debug)]
pub enum ScoreType {
    AchievementsPoints,
    Gold,
}

impl ToString for ScoreType {
    fn to_string(&self) -> String {
        match self {
            ScoreType::AchievementsPoints => "achievements_points".to_string(),
            ScoreType::Gold => "gold".to_string(),
        }
    }
}

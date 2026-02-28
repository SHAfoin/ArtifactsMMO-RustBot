#[derive(Debug)]
pub enum AchievementType {
    CombatKill,
    CombatDrop,
    CombatLevel,
    Gathering,
    Crafting,
    Recycling,
    Task,
    Other,
    Use,
    NpcBuy,
    NpcSell,
}

impl ToString for AchievementType {
    fn to_string(&self) -> String {
        match self {
            AchievementType::CombatKill => "combat_kill".to_string(),
            AchievementType::CombatDrop => "combat_drop".to_string(),
            AchievementType::CombatLevel => "combat_level".to_string(),
            AchievementType::Gathering => "gathering".to_string(),
            AchievementType::Crafting => "crafting".to_string(),
            AchievementType::Recycling => "recycling".to_string(),
            AchievementType::Task => "task".to_string(),
            AchievementType::Other => "other".to_string(),
            AchievementType::Use => "use".to_string(),
            AchievementType::NpcBuy => "npc_buy".to_string(),
            AchievementType::NpcSell => "npc_sell".to_string(),
        }
    }
}

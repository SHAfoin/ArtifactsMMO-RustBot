#[derive(Debug)]
pub enum Skill {
    Combat,
    Woodcutting,
    Mining,
    Fishing,
    Weaponcrafting,
    Gearcrafting,
    Jewelrycrafting,
    Cooking,
    Alchemy,
}

impl Skill {
    pub fn is_resource_skill(&self) -> bool {
        matches!(
            self,
            Skill::Woodcutting | Skill::Mining | Skill::Fishing | Skill::Alchemy
        )
    }

    pub fn is_task_skill(&self) -> bool {
        matches!(
            self,
            Skill::Weaponcrafting
                | Skill::Gearcrafting
                | Skill::Jewelrycrafting
                | Skill::Cooking
                | Skill::Woodcutting
                | Skill::Mining
                | Skill::Alchemy
                | Skill::Fishing
        )
    }

    pub fn is_crafting_skill(&self) -> bool {
        matches!(
            self,
            Skill::Weaponcrafting | Skill::Gearcrafting | Skill::Jewelrycrafting | Skill::Cooking
        )
    }
}

impl ToString for Skill {
    fn to_string(&self) -> String {
        match self {
            Skill::Combat => "combat".to_string(),
            Skill::Woodcutting => "woodcutting".to_string(),
            Skill::Mining => "mining".to_string(),
            Skill::Fishing => "fishing".to_string(),
            Skill::Weaponcrafting => "weaponcrafting".to_string(),
            Skill::Gearcrafting => "gearcrafting".to_string(),
            Skill::Jewelrycrafting => "jewelrycrafting".to_string(),
            Skill::Cooking => "cooking".to_string(),
            Skill::Alchemy => "alchemy".to_string(),
        }
    }
}

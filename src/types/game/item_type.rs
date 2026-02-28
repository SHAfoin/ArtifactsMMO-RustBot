#[derive(Debug)]
pub enum ItemType {
    Utility,
    BodyArmor,
    Weapon,
    Resource,
    LegArmor,
    Helmet,
    Boots,
    Shield,
    Amulet,
    Ring,
    Artifact,
    Currency,
    Consumable,
    Rune,
    Bag,
}

impl ItemType {
    pub fn as_str(&self) -> &str {
        match self {
            ItemType::Utility => "utility",
            ItemType::BodyArmor => "body_armor",
            ItemType::Weapon => "weapon",
            ItemType::Resource => "resource",
            ItemType::LegArmor => "leg_armor",
            ItemType::Helmet => "helmet",
            ItemType::Boots => "boots",
            ItemType::Shield => "shield",
            ItemType::Amulet => "amulet",
            ItemType::Ring => "ring",
            ItemType::Artifact => "artifact",
            ItemType::Currency => "currency",
            ItemType::Consumable => "consumable",
            ItemType::Rune => "rune",
            ItemType::Bag => "bag",
        }
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    api::characters::get_character,
    types::{
        common::{settings::Settings, validated_string::ValidatedString},
        game::{character, skin_type::SkinType},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryItem {
    pub slot: i32,
    pub code: String,
    pub quantity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Character {
    pub name: ValidatedString,
    pub account: ValidatedString,
    pub skin: SkinType,
    pub level: u64,
    pub xp: u64,
    pub max_xp: u64,
    pub gold: u64,
    pub speed: u64,

    pub mining_level: u64,
    pub mining_xp: u64,
    pub mining_max_xp: u64,

    pub woodcutting_level: u64,
    pub woodcutting_xp: u64,
    pub woodcutting_max_xp: u64,

    pub fishing_level: u64,
    pub fishing_xp: u64,
    pub fishing_max_xp: u64,

    pub weaponcrafting_level: u64,
    pub weaponcrafting_xp: u64,
    pub weaponcrafting_max_xp: u64,

    pub gearcrafting_level: u64,
    pub gearcrafting_xp: u64,
    pub gearcrafting_max_xp: u64,

    pub jewelrycrafting_level: u64,
    pub jewelrycrafting_xp: u64,
    pub jewelrycrafting_max_xp: u64,

    pub cooking_level: u64,
    pub cooking_xp: u64,
    pub cooking_max_xp: u64,

    pub alchemy_level: u64,
    pub alchemy_xp: u64,
    pub alchemy_max_xp: u64,

    pub hp: u64,
    pub max_hp: u64,
    pub haste: u64,
    pub critical_strike: u64,
    pub wisdom: u64,
    pub prospecting: u64,

    pub attack_fire: u64,
    pub attack_earth: u64,
    pub attack_water: u64,
    pub attack_air: u64,

    pub dmg: u64,
    pub dmg_fire: u64,
    pub dmg_earth: u64,
    pub dmg_water: u64,
    pub dmg_air: u64,

    pub res_fire: u64,
    pub res_earth: u64,
    pub res_water: u64,
    pub res_air: u64,

    pub x: i32,
    pub y: i32,

    pub cooldown: u64,
    pub cooldown_expiration: Option<DateTime<Utc>>,
    pub weapon_slot: ValidatedString,
    pub rune_slot: ValidatedString,
    pub shield_slot: ValidatedString,
    pub helmet_slot: ValidatedString,
    pub body_armor_slot: ValidatedString,
    pub leg_armor_slot: ValidatedString,
    pub boots_slot: ValidatedString,
    pub ring1_slot: ValidatedString,
    pub ring2_slot: ValidatedString,
    pub amulet_slot: ValidatedString,
    pub artifact1_slot: ValidatedString,
    pub artifact2_slot: ValidatedString,
    pub artifact3_slot: ValidatedString,

    pub utility1_slot: ValidatedString,
    pub utility1_slot_quantity: u64,
    pub utility2_slot: ValidatedString,
    pub utility2_slot_quantity: u64,

    pub bag_slot: ValidatedString,

    pub task: ValidatedString,
    pub task_type: ValidatedString,
    pub task_progress: u64,
    pub task_total: u64,

    pub inventory_max_items: u64,
    pub inventory: Vec<InventoryItem>,
}

impl Character {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn fetch_character(settings: &Settings, character_name: &ValidatedString) -> Self {
        match get_character(&settings, character_name).await {
            Ok(m) => {
                return Character::from_json(&m["data"]);
            }
            Err(e) => {
                error!(target = "gameplay", "Error fetching character: {}", e);
                return Character::new();
            }
        }
    }

    pub fn from_json(json: &serde_json::Value) -> Self {
        serde_json::from_value(json.clone()).unwrap_or_default()
    }

    pub fn update_from_response(
        &mut self,
        json: &serde_json::Value,
    ) -> Result<(), serde_json::Error> {
        let updated_character: Character = serde_json::from_value(json.clone())?;
        *self = updated_character;
        Ok(())
    }

    pub fn is_on_cooldown(&self) -> bool {
        if let Some(expiration) = self.cooldown_expiration {
            Utc::now() < expiration
        } else {
            false
        }
    }
}

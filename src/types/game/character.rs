use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::game::skin_type::SkinType;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryItem {
    pub slot: i32,
    pub code: String,
    pub quantity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Character {
    pub name: String,
    pub account: String,
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
    pub weapon_slot: String,
    pub rune_slot: String,
    pub shield_slot: String,
    pub helmet_slot: String,
    pub body_armor_slot: String,
    pub leg_armor_slot: String,
    pub boots_slot: String,
    pub ring1_slot: String,
    pub ring2_slot: String,
    pub amulet_slot: String,
    pub artifact1_slot: String,
    pub artifact2_slot: String,
    pub artifact3_slot: String,

    pub utility1_slot: String,
    pub utility1_slot_quantity: u64,
    pub utility2_slot: String,
    pub utility2_slot_quantity: u64,

    pub bag_slot: String,

    pub task: String,
    pub task_type: String,
    pub task_progress: u64,
    pub task_total: u64,

    pub inventory_max_items: u64,
    pub inventory: Vec<InventoryItem>,
}

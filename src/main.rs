use config::Config;
use core::fmt;
use dotenv::dotenv;
use regex::Regex;
use reqwest::{header::HeaderValue, Error};
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use std::path::Display;

/// men1 men2 men3 women1 women2 women3 corrupted1 zombie1
enum SkinType {
    Male1,
    Male2,
    Male3,
    Women1,
    Women2,
    Women3,
    Corrupted1,
    Zombie1,
}

impl fmt::Display for SkinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let skin_str = match self {
            SkinType::Male1 => "men1",
            SkinType::Male2 => "men2",
            SkinType::Male3 => "men3",
            SkinType::Women1 => "women1",
            SkinType::Women2 => "women2",
            SkinType::Women3 => "women3",
            SkinType::Corrupted1 => "corrupted1",
            SkinType::Zombie1 => "zombie1",
        };
        write!(f, "{}", skin_str)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum EquipmentSlot {
    Head,
    Body,
    Legs,
    Feet,
    Ring1,
    Ring2,
    Amulet,
    Artifact1,
    Artifact2,
    Artifact3,
    Utility1,
    Utility2,
    Bag,
    Rune,
}

impl fmt::Display for EquipmentSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let slot_str = match self {
            EquipmentSlot::Head => "head",
            EquipmentSlot::Body => "body",
            EquipmentSlot::Legs => "legs",
            EquipmentSlot::Feet => "feet",
            EquipmentSlot::Ring1 => "ring1",
            EquipmentSlot::Ring2 => "ring2",
            EquipmentSlot::Amulet => "amulet",
            EquipmentSlot::Artifact1 => "artifact1",
            EquipmentSlot::Artifact2 => "artifact2",
            EquipmentSlot::Artifact3 => "artifact3",
            EquipmentSlot::Utility1 => "utility1",
            EquipmentSlot::Utility2 => "utility2",
            EquipmentSlot::Bag => "bag",
            EquipmentSlot::Rune => "rune",
        };
        write!(f, "{}", slot_str)
    }
}

#[derive(Debug, Clone)]
pub struct ValidatedString(String);

impl ValidatedString {
    pub fn new(value: &str) -> Result<Self, String> {
        let regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        if regex.is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(format!(
                "Invalid string: '{}'. Must match ^[a-zA-Z0-9_-]+$",
                value
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidatedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ValidatedString {
    fn from(item: &str) -> Self {
        ValidatedString::new(&item).unwrap_or_default()
    }
}

impl Default for ValidatedString {
    fn default() -> Self {
        Self("".to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ValidatedStringWithSpaces(String);

impl ValidatedStringWithSpaces {
    pub fn new(value: &str) -> Result<Self, String> {
        let regex = Regex::new(r"^[a-zA-Z0-9_-]+(\s[a-zA-Z0-9_-]+)*\s?$").unwrap();
        if regex.is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(format!(
                "Invalid string: '{}'. Must match ^[a-zA-Z0-9_-]+(\\s[a-zA-Z0-9_-]+)*\\s?$",
                value
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidatedStringWithSpaces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ValidatedStringWithSpaces {
    fn from(item: &str) -> Self {
        ValidatedStringWithSpaces::new(&item).unwrap_or_default()
    }
}

impl Default for ValidatedStringWithSpaces {
    fn default() -> Self {
        Self("".to_string())
    }
}

enum Skill {
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
    fn is_resource_skill(&self) -> bool {
        matches!(
            self,
            Skill::Woodcutting | Skill::Mining | Skill::Fishing | Skill::Alchemy
        )
    }

    fn is_task_skill(&self) -> bool {
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

    fn is_crafting_skill(&self) -> bool {
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

enum TaskType {
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

enum NPCType {
    Merchant,
    Trader,
}

impl ToString for NPCType {
    fn to_string(&self) -> String {
        match self {
            NPCType::Merchant => "merchant".to_string(),
            NPCType::Trader => "trader".to_string(),
        }
    }
}

enum MapContentType {
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

enum AchievementType {
    CombatKill,
    CombatDrop,
    CombatLevel,
    Gathering,
    Crafting,
    Recycling,
    Task,
    Other,
    Use,
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
        }
    }
}

enum ScoreType {
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

enum ItemType {
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
    fn as_str(&self) -> &str {
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

enum EventType {
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
    fn as_str(&self) -> &str {
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

#[derive(Deserialize)]
struct Settings {
    api_url: String,
    api_token: SecretBox<String>,
}

#[derive(Debug, Clone)]
struct PaginationParams {
    page: isize,
    size: isize,
}

impl PaginationParams {
    fn new(page: isize, size: isize) -> Result<Self, String> {
        if page < 1 {
            return Err("Page must be >= 1".to_string());
        }
        if size < 1 || size > 100 {
            return Err("Size must be between 1 and 100".to_string());
        }
        Ok(Self { page, size })
    }

    fn default() -> Self {
        Self { page: 1, size: 50 }
    }

    fn to_query_params(&self) -> Vec<(&str, String)> {
        vec![
            ("page", self.page.to_string()),
            ("size", self.size.to_string()),
        ]
    }
}

fn app_configuration() -> Settings {
    dotenv().ok();

    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .add_source(config::Environment::with_prefix("artifactsmmo"))
        .build()
        .unwrap();

    config.try_deserialize().unwrap()
}

async fn post(settings: Settings, path: &str, json: &str) -> Result<(), Error> {
    let url = format!("{}{}", settings.api_url, path);

    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", settings.api_token.expose_secret())).unwrap(),
    );

    let response = client
        .post(url)
        .headers(headers)
        .body(json.to_owned())
        .send()
        .await?;

    println!("Status Code: {}", response.status());

    let response_body = response.text().await?;

    println!("Response body: \n{}", response_body);

    Ok(())
}

async fn get(
    settings: Settings,
    path: &str,
    query_params: Option<Vec<(&str, String)>>,
) -> Result<(), Error> {
    let url = format!("{}{}", settings.api_url, path);

    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", settings.api_token.expose_secret())).unwrap(),
    );

    let response = client
        .get(url)
        .headers(headers)
        .query(&query_params.unwrap_or_default())
        .send()
        .await?;

    println!("Status Code: {}", response.status());

    let response_text = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_text)
        .unwrap_or_else(|_| serde_json::Value::String(response_text.clone()));
    let response_body = serde_json::to_string_pretty(&response_json).unwrap();

    println!("Response body: \n{}", response_body);

    Ok(())
}

/// Return the status of the game server.
async fn get_server_details(settings: Settings) -> Result<(), Error> {
    get(settings, "/", None).await
}

/// Fetch account details.
async fn get_account_details(settings: Settings) -> Result<(), Error> {
    get(settings, "/my/details", None).await
}

/// Retrieve the details of a character.
async fn get_character(settings: Settings, name: ValidatedString) -> Result<(), Error> {
    get(settings, &format!("/characters/{}", name), None).await
    // match name {
    //     Some(n) =>
    //     None => get(settings, "/my/characters", None).await,
    // }
}

/// Fetch bank details.
async fn get_bank_details(settings: Settings) -> Result<(), Error> {
    get(settings, "/my/bank", None).await
}

/// Fetch all items in your bank.
async fn get_bank_items(
    settings: Settings,
    item_code: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(code) = item_code {
        query_params.push(("item_code", code.to_string()));
    }

    get(settings, "/my/bank/items", Some(query_params)).await
}

/// Fetch your sell orders details.
async fn get_my_grandexchange_sell_orders(
    settings: Settings,
    code: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(code) = code {
        query_params.push(("code", code.to_string()));
    }

    get(settings, "/my/grandexchange/orders", Some(query_params)).await
}

/// Fetch your sales history of the last 7 days.
async fn get_my_grandexchange_sell_history(
    settings: Settings,
    code: Option<ValidatedString>,
    id: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(code) = code {
        query_params.push(("code", code.to_string()));
    }

    if let Some(id) = id {
        query_params.push(("id", id.to_string()));
    }

    get(settings, "/my/grandexchange/history", Some(query_params)).await
}

/// Fetch all sell orders.
async fn get_all_grandexchange_orders(
    settings: Settings,
    seller: Option<ValidatedString>,
    code: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(seller) = seller {
        query_params.push(("seller", seller.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(code) = code {
        query_params.push(("code", code.to_string()));
    }

    get(settings, "/grandexchange/orders", Some(query_params)).await
}

/// Retrieve the sell order of a item.
async fn get_grandexchange_order(settings: Settings, id: ValidatedString) -> Result<(), Error> {
    get(settings, &format!("/grandexchange/orders/{}", id), None).await
}

// For a specific item only, print the last 7 days of sell history
async fn get_grandexchange_sell_history(
    settings: Settings,
    code: ValidatedString,
    buyer: Option<ValidatedString>,
    seller: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(buyer) = buyer {
        query_params.push(("buyer", buyer.to_string()));
    }

    if let Some(seller) = seller {
        query_params.push(("seller", seller.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(
        settings,
        &format!("/grandexchange/history/{}", code),
        Some(query_params),
    )
    .await
}

async fn get_characters_logs(
    settings: Settings,
    character: Option<ValidatedString>,
) -> Result<(), Error> {
    get(
        settings,
        &format!("/my/logs/{}", character.unwrap_or_default()),
        None,
    )
    .await
}

/// Retrieve the achievements of a account.
async fn get_account_achievements(
    settings: Settings,
    account: ValidatedString,
    completed: Option<bool>,
    _type: Option<AchievementType>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(completed) = completed {
        query_params.push(("completed", completed.to_string()));
    }

    if let Some(_type) = _type {
        query_params.push(("type", _type.to_string()));
    }

    get(
        settings,
        &format!("/accounts/{}/achievements", account),
        Some(query_params),
    )
    .await
}

/// Fetch account character lists.
async fn get_account_characters(settings: Settings, account: ValidatedString) -> Result<(), Error> {
    get(settings, &format!("/accounts/{}/characters", account), None).await
}

/// Retrieve the details of a character.
async fn get_account(settings: Settings, account: ValidatedString) -> Result<(), Error> {
    get(settings, &format!("/accounts/{}", account), None).await
}

/// List of all achievements.
async fn get_all_achievements(
    settings: Settings,
    _type: Option<AchievementType>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    if let Some(_type) = _type {
        query_params.push(("type", _type.to_string()));
    }

    get(settings, "/achievements", Some(query_params)).await
}

/// Retrieve the details of a achievement.
async fn get_achievement(settings: Settings, code: ValidatedString) -> Result<(), Error> {
    get(settings, &format!("/achievements/{}", code), None).await
}

/// List of all badges.
async fn get_all_badges(
    settings: Settings,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/badges", Some(query_params)).await
}

/// Retrieve the details of a badge.
async fn get_badge(settings: Settings, code: &str) -> Result<(), Error> {
    get(settings, &format!("/badges/{}", code), None).await
}

/// List of all effects. Effects are used by equipment, tools, runes, consumables and monsters. An effect is an action that produces an effect on the game.
async fn get_all_effects(
    settings: Settings,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();
    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }
    get(settings, "/effects", Some(query_params)).await
}

/// Retrieve the details of a badge.
async fn get_effect(settings: Settings, code: &str) -> Result<(), Error> {
    get(settings, &format!("/effects/{}", code), None).await
}

/// Fetch events details.
async fn get_all_events(
    settings: Settings,
    _type: Option<EventType>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();
    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }
    if let Some(_type) = _type {
        query_params.push(("type", _type.as_str().to_string()));
    }
    get(settings, "/events", Some(query_params)).await
}

/// Fetch active events details.
async fn get_all_active_events(
    settings: Settings,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();
    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }
    get(settings, "/events/active", Some(query_params)).await
}

/// Fetch items details.
async fn get_all_items(
    settings: Settings,
    craft_material: Option<ValidatedString>,
    craft_skill: Option<Skill>,
    max_level: Option<isize>,
    min_level: Option<isize>,
    name: Option<ValidatedStringWithSpaces>,
    _type: Option<ItemType>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(craft_skill) = &craft_skill {
        if !craft_skill.is_crafting_skill() {
            panic!("craft_skill must be a crafting skill");
        }
    }

    if let (Some(min), Some(max)) = (min_level, max_level) {
        if min > max {
            panic!("min_level cannot be greater than max_level");
        }
    }

    if let Some(craft_material) = craft_material {
        query_params.push(("craft_material", craft_material.to_string()));
    }

    if let Some(craft_skill) = craft_skill {
        query_params.push(("craft_skill", craft_skill.to_string()));
    }

    if let Some(max_level) = max_level {
        query_params.push(("max_level", max_level.to_string()));
    }

    if let Some(min_level) = min_level {
        query_params.push(("min_level", min_level.to_string()));
    }

    if let Some(name) = name {
        query_params.push(("name", name.to_string()));
    }

    if let Some(_type) = _type {
        query_params.push(("type", _type.as_str().to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/items", Some(query_params)).await
}

/// Retrieve the details of a item.
async fn get_item(settings: Settings, code: &str) -> Result<(), Error> {
    get(settings, &format!("/items/{}", code), None).await
}

/// Fetch leaderboard details.
async fn get_characters_leaderboard(
    settings: Settings,
    name: Option<ValidatedStringWithSpaces>,
    sort: Option<Skill>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(name) = name {
        query_params.push(("name", name.to_string()));
    }

    if let Some(sort) = sort {
        query_params.push(("sort", sort.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/leaderboard/characters", Some(query_params)).await
}

/// Fetch leaderboard details.
async fn get_account_leaderboard(
    settings: Settings,
    name: Option<ValidatedStringWithSpaces>,
    sort: Option<ScoreType>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(name) = name {
        query_params.push(("name", name.to_string()));
    }

    if let Some(sort) = sort {
        query_params.push(("sort", sort.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/leaderboard/accounts", Some(query_params)).await
}

/// Fetch maps details.
async fn get_all_maps(
    settings: Settings,
    content_code: Option<ValidatedString>,
    content_type: Option<MapContentType>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(content_code) = content_code {
        query_params.push(("content_code", content_code.to_string()));
    }

    if let Some(content_type) = content_type {
        query_params.push(("content_type", content_type.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/maps", Some(query_params)).await
}

/// Retrieve the details of a map.
async fn get_map(settings: Settings, x: &str, y: &str) -> Result<(), Error> {
    get(settings, &format!("/maps/{}/{}", x, y), None).await
}

/// Fetch maps details.
async fn get_all_monsters(
    settings: Settings,
    drop: Option<ValidatedString>,
    max_level: Option<isize>,
    min_level: Option<isize>,
    name: Option<ValidatedStringWithSpaces>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let (Some(min), Some(max)) = (min_level, max_level) {
        if min > max {
            panic!("min_level cannot be greater than max_level");
        }
    }

    if let Some(drop) = drop {
        query_params.push(("drop", drop.to_string()));
    }

    if let Some(max_level) = max_level {
        query_params.push(("max_level", max_level.to_string()));
    }

    if let Some(min_level) = min_level {
        query_params.push(("min_level", min_level.to_string()));
    }

    if let Some(name) = name {
        query_params.push(("name", name.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, &format!("/monsters"), Some(query_params)).await
}

/// Retrieve the details of a monster.
async fn get_monster(settings: Settings, code: &str) -> Result<(), Error> {
    get(settings, &format!("/monsters/{}", code), None).await
}

/// Fetch NPCs details.
async fn get_all_npcs(
    settings: Settings,
    name: Option<ValidatedStringWithSpaces>,
    _type: Option<NPCType>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(name) = name {
        query_params.push(("name", name.to_string()));
    }

    if let Some(_type) = _type {
        query_params.push(("type", _type.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, &format!("/npcs/details"), Some(query_params)).await
}

/// Retrieve the details of a NPC.
async fn get_npc(settings: Settings, code: Option<ValidatedString>) -> Result<(), Error> {
    get(
        settings,
        &format!("/npcs/details/{}", code.unwrap_or_default()),
        None,
    )
    .await
}

/// Retrieve the items list of a NPC. If the NPC has items to buy, sell or trade, they will be displayed.
async fn get_npc_items(
    settings: Settings,
    code: &str,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();
    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }
    get(
        settings,
        &format!("/npcs/items/{}", code),
        Some(query_params),
    )
    .await
}

/// Retrieve the list of all NPC items.
async fn get_all_npcs_items(
    settings: Settings,
    code: Option<ValidatedString>,
    currency: Option<ValidatedString>,
    npc: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();
    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }
    if let Some(code) = code {
        query_params.push(("code", code.to_string()));
    }
    if let Some(currency) = currency {
        query_params.push(("currency", currency.to_string()));
    }
    if let Some(npc) = npc {
        query_params.push(("npc", npc.to_string()));
    }

    get(settings, "/npcs/items", Some(query_params)).await
}

/// Fetch resources details.
async fn get_all_resources(
    settings: Settings,
    drop: Option<ValidatedString>,
    max_level: Option<isize>,
    min_level: Option<isize>,
    skill: Option<Skill>,
    name: Option<ValidatedString>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(skill) = &skill {
        if !skill.is_resource_skill() {
            panic!("skill must be a resource skill");
        }
    }

    if let (Some(min), Some(max)) = (min_level, max_level) {
        if min > max {
            panic!("min_level cannot be greater than max_level");
        }
    }

    if let Some(drop) = drop {
        query_params.push(("drop", drop.to_string()));
    }

    if let Some(max_level) = max_level {
        query_params.push(("max_level", max_level.to_string()));
    }

    if let Some(min_level) = min_level {
        query_params.push(("min_level", min_level.to_string()));
    }

    if let Some(skill) = skill {
        query_params.push(("skill", skill.to_string()));
    }

    if let Some(name) = name {
        query_params.push(("name", name.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/resources", Some(query_params)).await
}

/// Retrieve the details of a resource.
async fn get_resource(settings: Settings, code: &str) -> Result<(), Error> {
    get(settings, &format!("/resources/{}", code), None).await
}

/// Retrieve the details of a task.
async fn get_task(settings: Settings, code: &str) -> Result<(), Error> {
    get(settings, &format!("/tasks/list/{}", code), None).await
}

/// Fetch the list of all tasks.
async fn get_all_tasks(
    settings: Settings,
    max_level: Option<isize>,
    min_level: Option<isize>,
    skill: Option<Skill>,
    _type: Option<TaskType>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(skill) = &skill {
        if !skill.is_task_skill() {
            panic!("skill must be a task skill");
        }
    }

    if let (Some(min), Some(max)) = (min_level, max_level) {
        if min > max {
            panic!("min_level cannot be greater than max_level");
        }
    }

    if let Some(max_level) = max_level {
        query_params.push(("max_level", max_level.to_string()));
    }

    if let Some(min_level) = min_level {
        query_params.push(("min_level", min_level.to_string()));
    }

    if let Some(skill) = skill {
        query_params.push(("skill", skill.to_string()));
    }

    if let Some(_type) = _type {
        query_params.push(("type", _type.to_string()));
    }

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/tasks/list", Some(query_params)).await
}

/// Retrieve the details of a tasks reward.
async fn get_tasks_reward(settings: Settings, code: &str) -> Result<(), Error> {
    get(settings, &format!("/tasks/rewards/{}", code), None).await
}

/// Fetch the list of all tasks rewards. To obtain these rewards, you must exchange 6 task coins with a tasks master.
async fn get_all_tasks_rewards(
    settings: Settings,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, "/tasks/rewards", Some(query_params)).await
}

/// Moves a character on the map using the map's X and Y position.
async fn action_move(
    settings: Settings,
    name: ValidatedString,
    x: isize,
    y: isize,
) -> Result<(), Error> {
    let json = format!(r#"{{"x": {}, "y": {}}}"#, x, y);
    post(settings, &format!("/my/{}/action/move", name), &json).await
}

/// Recovers hit points by resting. (1 second per 5 HP, minimum 3 seconds)
async fn action_rest(settings: Settings, name: ValidatedString) -> Result<(), Error> {
    post(settings, &format!("/my/{}/action/rest", name), "").await
}

/// Equip an item on your character.
async fn action_equip_item(
    settings: Settings,
    name: ValidatedString,
    code: ValidatedString,
    slot: EquipmentSlot,
    quantity: Option<isize>,
) -> Result<(), Error> {
    if (slot == EquipmentSlot::Utility1 || slot == EquipmentSlot::Utility2) {
        if let Some(q) = quantity {
            if q < 1 || q > 100 {
                panic!(
                    "Quantity must be between 1 and 100 when equipping an item in a utility slot"
                );
            }
        }
    }

    let json = format!(
        r#"{{"code": "{}", "slot": "{}", "quantity": {}}}"#,
        code,
        slot,
        quantity.unwrap_or(1)
    );
    post(settings, &format!("/my/{}/action/equip", name), &json).await
}

/// Unequip an item on your character.
async fn action_unequip_item(
    settings: Settings,
    name: ValidatedString,
    slot: EquipmentSlot,
    quantity: Option<isize>,
) -> Result<(), Error> {
    if (slot == EquipmentSlot::Utility1 || slot == EquipmentSlot::Utility2) {
        if let Some(q) = quantity {
            if q < 1 || q > 100 {
                panic!(
                    "Quantity must be between 1 and 100 when unequipping an item in a utility slot"
                );
            }
        }
    }

    let json = format!(
        r#"{{"slot": "{}", "quantity": {}}}"#,
        slot,
        quantity.unwrap_or(1)
    );
    post(settings, &format!("/my/{}/action/unequip", name), &json).await
}

/// Use an item as a consumable.
async fn action_use_item(
    settings: Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: isize,
) -> Result<(), Error> {
    if quantity < 1 {
        panic!("Quantity must be at least 1 when using an item");
    }

    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post(settings, &format!("/my/{}/action/use", name), &json).await
}

/// Start a fight against a monster on the character's map.
async fn action_fight(settings: Settings, name: ValidatedString) -> Result<(), Error> {
    post(settings, &format!("/my/{}/action/fight", name), "").await
}

/// Harvest a resource on the character's map.
async fn action_gathering(settings: Settings, name: ValidatedString) -> Result<(), Error> {
    post(settings, &format!("/my/{}/action/gathering", name), "").await
}

/// Crafting an item. The character must be on a map with a workshop.
async fn action_crafting(
    settings: Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: Option<isize>,
) -> Result<(), Error> {
    if let Some(q) = quantity {
        if q < 1 {
            panic!("Quantity must be at least 1 when crafting an item");
        }
    }

    let json = format!(
        r#"{{"code": "{}", "quantity": {}}}"#,
        code,
        quantity.unwrap_or(1)
    );
    post(settings, &format!("/my/{}/action/crafting", name), &json).await
}

/// Deposit gold in a bank on the character's map.
async fn action_deposit_bank_gold(
    settings: Settings,
    name: ValidatedString,
    quantity: isize,
) -> Result<(), Error> {
    if quantity < 1 {
        panic!("Quantity must be at least 1 when depositing gold");
    }

    let json = format!(r#"{{"quantity": {}}}"#, quantity);
    post(
        settings,
        &format!("/my/{}/action/bank/deposit/gold", name),
        &json,
    )
    .await
}

/// Deposit multiple items in a bank on the character's map. The cooldown will be 3 seconds multiplied by the number of different items withdrawn.
async fn action_deposit_bank_item(
    settings: Settings,
    name: ValidatedString,
    items: Vec<(ValidatedString, isize)>,
) -> Result<(), Error> {
    for (code, quantity) in &items {
        if *quantity < 1 {
            panic!("Quantity must be at least 1 when depositing an item");
        }
    }

    let items_json: Vec<String> = items
        .into_iter()
        .map(|(code, quantity)| format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity))
        .collect();

    let json_string = format!("[{}]", items_json.join(","));

    post(
        settings,
        &format!("/my/{}/action/bank/deposit/item", name),
        &json_string,
    )
    .await
}

/// Take items from your bank and put them in the character's inventory. The cooldown will be 3 seconds multiplied by the number of different items withdrawn.
async fn action_withdraw_bank_item(
    settings: Settings,
    name: ValidatedString,
    items: Vec<(ValidatedString, isize)>,
) -> Result<(), Error> {
    for (code, quantity) in &items {
        if *quantity < 1 {
            panic!("Quantity must be at least 1 when withdrawing an item");
        }
    }

    let items_json: Vec<String> = items
        .into_iter()
        .map(|(code, quantity)| format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity))
        .collect();

    let json_string = format!("[{}]", items_json.join(","));

    post(
        settings,
        &format!("/my/{}/action/bank/withdraw/item", name),
        &json_string,
    )
    .await
}

/// Withdraw gold from your bank.
async fn action_withdraw_bank_gold(
    settings: Settings,
    name: ValidatedString,
    quantity: isize,
) -> Result<(), Error> {
    if quantity < 1 {
        panic!("Quantity must be at least 1 when withdrawing gold");
    }

    let json = format!(r#"{{"quantity": {}}}"#, quantity);
    post(
        settings,
        &format!("/my/{}/action/bank/withdraw/gold", name),
        &json,
    )
    .await
}

/// Buy a 25 slots bank expansion.
async fn action_buy_bank_expansion(settings: Settings, name: ValidatedString) -> Result<(), Error> {
    post(
        settings,
        &format!("/my/{}/action/bank/buy_expansion", name),
        "",
    )
    .await
}

/// Buy an item from an NPC on the character's map.
async fn action_npc_buy_item(
    settings: Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: isize,
) -> Result<(), Error> {
    if quantity < 1 || quantity > 100 {
        panic!("Quantity must be between 1 and 100 when buying an item");
    }

    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post(settings, &format!("/my/{}/action/npc/buy", name), &json).await
}

/// Sell an item to an NPC on the character's map.
async fn action_npc_sell_item(
    settings: Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: isize,
) -> Result<(), Error> {
    if quantity < 1 || quantity > 100 {
        panic!("Quantity must be between 1 and 100 when selling an item");
    }

    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post(settings, &format!("/my/{}/action/npc/sell", name), &json).await
}

/// Recycling an item. The character must be on a map with a workshop (only for equipments and weapons).
async fn action_recycling(
    settings: Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: Option<isize>,
) -> Result<(), Error> {
    if let Some(q) = quantity {
        if q < 1 {
            panic!("Quantity must be at least 1 when recycling an item");
        }
    }

    let json = format!(
        r#"{{"code": "{}", "quantity": {}}}"#,
        code,
        quantity.unwrap_or(1)
    );
    post(settings, &format!("/my/{}/action/recycling", name), &json).await
}

/// Buy an item at the Grand Exchange on the character's map.
async fn action_grandexchange_buy_item(
    settings: Settings,
    name: ValidatedString,
    id: String,
    quantity: isize,
) -> Result<(), Error> {
    if quantity < 1 || quantity > 100 {
        panic!("Quantity must be between 1 and 100 when buying an item");
    }

    let json = format!(r#"{{"id": "{}", "quantity": {}}}"#, id, quantity);
    post(
        settings,
        &format!("/my/{}/action/grandexchange/buy", name),
        &json,
    )
    .await
}

/// Create a sell order at the Grand Exchange on the character's map. Please note there is a 3% listing tax, charged at the time of posting, on the total price.
async fn action_grandexchange_create_sell_order(
    settings: Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: isize,
    price: isize,
) -> Result<(), Error> {
    if quantity < 1 || quantity > 100 {
        panic!("Quantity must be between 1 and 100 when creating a sell order");
    }

    if price < 1 || price > 1_000_000_000 {
        panic!("Price must be at least 1 and at most 1,000,000,000 when creating a sell order");
    }

    let json = format!(
        r#"{{"code": "{}", "price": {}, "quantity": {}}}"#,
        code, price, quantity
    );

    post(
        settings,
        &format!("/my/{}/action/grandexchange/sell", name),
        &json,
    )
    .await
}

/// Cancel a sell order at the Grand Exchange on the character's map.
async fn action_grandexchange_cancel_sell_order(
    settings: Settings,
    name: ValidatedString,
    id: String,
) -> Result<(), Error> {
    let json = format!(r#"{{"id": "{}"}}"#, id);

    post(
        settings,
        &format!("/my/{}/action/grandexchange/cancel", name),
        &json,
    )
    .await
}

/// Complete a task.
async fn action_complete_task(settings: Settings, name: ValidatedString) -> Result<(), Error> {
    post(settings, &format!("/my/{}/action/task/complete", name), "").await
}

/// Exchange 6 tasks coins for a random reward. Rewards are exclusive items or resources.
async fn action_task_exchange(settings: Settings, name: ValidatedString) -> Result<(), Error> {
    post(settings, &format!("/my/{}/action/task/exchange", name), "").await
}

/// Accepting a new task.
async fn action_accept_new_task(settings: Settings, name: ValidatedString) -> Result<(), Error> {
    post(settings, &format!("/my/{}/action/task/new", name), "").await
}

/// Trading items with a Tasks Master.
async fn action_task_trade(
    settings: Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: isize,
) -> Result<(), Error> {
    if quantity < 1 {
        panic!("Quantity must be at least 1 when trading an item");
    }

    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post(settings, &format!("/my/{}/action/task/trade", name), &json).await
}

/// Cancel a task for 1 tasks coin.
async fn action_cancel_task(settings: Settings, name: ValidatedString) -> Result<(), Error> {
    post(settings, &format!("/my/{}/action/task/cancel", name), "").await
}

/// Give gold to another character in your account on the same map.
async fn action_give_gold(
    settings: Settings,
    name: ValidatedString,
    quantity: isize,
    character: ValidatedString,
) -> Result<(), Error> {
    if quantity < 1 {
        panic!("Quantity must be at least 1 when giving gold");
    }

    let json = format!(
        r#"{{"quantity": {}, "character": "{}"}}"#,
        quantity, character
    );
    post(settings, &format!("/my/{}/action/give/gold", name), &json).await
}

/// Give items to another character in your account on the same map. The cooldown will be 3 seconds multiplied by the number of different items given.
async fn action_give_item(
    settings: Settings,
    name: ValidatedString,
    items: Vec<(ValidatedString, isize)>,
    character: ValidatedString,
) -> Result<(), Error> {
    if items.len() < 1 || items.len() > 20 {
        panic!("You must give between 1 and 20 different items");
    }

    for (code, quantity) in &items {
        if *quantity < 1 {
            panic!("Quantity must be at least 1 when giving an item");
        }
    }

    let items_json: Vec<String> = items
        .into_iter()
        .map(|(code, quantity)| format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity))
        .collect();

    let json_string = format!(
        r#"{{"items": [{}], "character": "{}"}}"#,
        items_json.join(","),
        character
    );

    post(
        settings,
        &format!("/my/{}/action/give/item", name),
        &json_string,
    )
    .await
}

/// Delete an item from your character's inventory.
async fn action_delete_item(
    settings: Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: isize,
) -> Result<(), Error> {
    if quantity < 1 {
        panic!("Quantity must be at least 1 when deleting an item");
    }

    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post(settings, &format!("/my/{}/action/delete", name), &json).await
}

/// Change the skin of your character.
async fn action_change_skin(
    settings: Settings,
    name: ValidatedString,
    skin: SkinType,
) -> Result<(), Error> {
    let json = format!(r#"{{"skin": "{}"}}"#, skin);
    post(settings, &format!("/my/{}/action/change_skin", name), &json).await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let settings: Settings = app_configuration();
    // get_account_characters(settings, "shafoin".into()).await?;

    // post_request().await?;
    Ok(())
}

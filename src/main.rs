use std::path;

use config::Config;
use dotenv::dotenv;
use reqwest::{header::HeaderValue, Error};
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;

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

enum ResourceSkill {
    Woodcutting,
    Mining,
    Fishing,
    Alchemy,
}

impl ToString for ResourceSkill {
    fn to_string(&self) -> String {
        match self {
            ResourceSkill::Woodcutting => "woodcutting".to_string(),
            ResourceSkill::Mining => "mining".to_string(),
            ResourceSkill::Fishing => "fishing".to_string(),
            ResourceSkill::Alchemy => "alchemy".to_string(),
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

enum TaskSkillType {
    Weaponcrafting,
    Gearcrafting,
    Jewelrycrafting,
    Cooking,
    Woodcutting,
    Mining,
    Alchemy,
    Fishing,
}

impl ToString for TaskSkillType {
    fn to_string(&self) -> String {
        match self {
            TaskSkillType::Weaponcrafting => "weaponcrafting".to_string(),
            TaskSkillType::Gearcrafting => "gearcrafting".to_string(),
            TaskSkillType::Jewelrycrafting => "jewelrycrafting".to_string(),
            TaskSkillType::Cooking => "cooking".to_string(),
            TaskSkillType::Woodcutting => "woodcutting".to_string(),
            TaskSkillType::Mining => "mining".to_string(),
            TaskSkillType::Alchemy => "alchemy".to_string(),
            TaskSkillType::Fishing => "fishing".to_string(),
        }
    }
}

enum CraftSkillType {
    Weaponcrafting,
    Gearcrafting,
    Jewelrycrafting,
    Cooking,
    Woodcutting,
    Mining,
    Alchemy,
}

impl ToString for CraftSkillType {
    fn to_string(&self) -> String {
        match self {
            CraftSkillType::Weaponcrafting => "weaponcrafting".to_string(),
            CraftSkillType::Gearcrafting => "gearcrafting".to_string(),
            CraftSkillType::Jewelrycrafting => "jewelrycrafting".to_string(),
            CraftSkillType::Cooking => "cooking".to_string(),
            CraftSkillType::Woodcutting => "woodcutting".to_string(),
            CraftSkillType::Mining => "mining".to_string(),
            CraftSkillType::Alchemy => "alchemy".to_string(),
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

enum XPType {
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

impl ToString for XPType {
    fn to_string(&self) -> String {
        match self {
            XPType::Combat => "combat".to_string(),
            XPType::Woodcutting => "woodcutting".to_string(),
            XPType::Mining => "mining".to_string(),
            XPType::Fishing => "fishing".to_string(),
            XPType::Weaponcrafting => "weaponcrafting".to_string(),
            XPType::Gearcrafting => "gearcrafting".to_string(),
            XPType::Jewelrycrafting => "jewelrycrafting".to_string(),
            XPType::Cooking => "cooking".to_string(),
            XPType::Alchemy => "alchemy".to_string(),
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
    page: u32,
    size: u32,
}

impl PaginationParams {
    fn new(page: u32, size: u32) -> Result<Self, String> {
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
async fn get_character(settings: Settings, name: &str) -> Result<(), Error> {
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
    item_code: Option<&str>,
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
    code: Option<&str>,
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
    code: Option<&str>,
    id: Option<&str>,
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
    seller: Option<&str>,
    code: Option<&str>,
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
async fn get_grandexchange_order(settings: Settings, id: &str) -> Result<(), Error> {
    get(settings, &format!("/grandexchange/orders/{}", id), None).await
}

// For a specific item only, print the last 7 days of sell history
async fn get_grandexchange_sell_history(
    settings: Settings,
    code: &str,
    buyer: Option<&str>,
    seller: Option<&str>,
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

async fn get_characters_logs(settings: Settings, character: Option<&str>) -> Result<(), Error> {
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
    account: &str,
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
async fn get_account_characters(settings: Settings, account: &str) -> Result<(), Error> {
    get(settings, &format!("/accounts/{}/characters", account), None).await
}

/// Retrieve the details of a character.
async fn get_account(settings: Settings, account: &str) -> Result<(), Error> {
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
async fn get_achievement(settings: Settings, code: &str) -> Result<(), Error> {
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
    craft_material: Option<&str>,
    craft_skill: Option<CraftSkillType>,
    max_level: Option<u32>,
    min_level: Option<u32>,
    name: Option<&str>,
    _type: Option<ItemType>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

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
    name: Option<&str>,
    sort: Option<XPType>,
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
    name: Option<&str>,
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
    content_code: Option<&str>,
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
    drop: Option<&str>,
    max_level: Option<u32>,
    min_level: Option<u32>,
    name: Option<&str>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

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
    name: Option<&str>,
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
async fn get_npc(settings: Settings, code: Option<&str>) -> Result<(), Error> {
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
    code: Option<&str>,
    currency: Option<&str>,
    npc: Option<&str>,
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
    drop: Option<&str>,
    max_level: Option<u32>,
    min_level: Option<u32>,
    skill: Option<ResourceSkill>,
    name: Option<&str>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

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
    max_level: Option<u32>,
    min_level: Option<u32>,
    skill: Option<TaskSkillType>,
    _type: Option<TaskType>,
    pagination: Option<PaginationParams>,
) -> Result<(), Error> {
    let mut query_params = Vec::new();

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let settings: Settings = app_configuration();
    get_account_characters(settings, "shafoin").await?;

    // post_request().await?;
    Ok(())
}

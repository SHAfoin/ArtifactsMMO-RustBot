use anyhow::Result;
use tracing::info_span;

use crate::{
    api::utils::{get, post},
    types::{
        common::{
            pagination_params::PaginationParams, settings::Settings,
            validated_string::ValidatedString,
        },
        game::{equipment_slot::EquipmentSlot, skin_type::SkinType},
    },
};

/// History of the last 250 actions of all your characters.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_characters_logs_my_logs_get
pub async fn get_all_characters_logs(
    settings: &Settings,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let span = info_span!("get_all_characters_logs");
    let _enter = span.enter();

    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, &format!("/my/logs"), Some(query_params)).await
}

/// History of the last actions of your character.
/// https://api.artifactsmmo.com/docs/#/operations/get_character_logs_my_logs__name__get
pub async fn get_character_logs(
    settings: &Settings,
    character: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("get_characters_logs", character = %character);
    let _enter = span.enter();

    get(settings, &format!("/my/logs/{}", character), None).await
}

/// List of your characters.
/// https://api.artifactsmmo.com/docs/#/operations/get_my_characters_my_characters_get
pub async fn get_my_characters(settings: &Settings) -> Result<serde_json::Value> {
    let span = info_span!("get_my_characters");
    let _enter = span.enter();

    get(settings, "/my/characters", None).await
}

/// Moves a character on the map using either the map's ID or X and Y position. Provide either 'map_id' or both 'x' and 'y' coordinates in the request body.
/// https://api.artifactsmmo.com/docs/#/operations/action_move_my__name__action_move_post
pub async fn action_move(
    settings: &Settings,
    name: &ValidatedString,
    x: Option<i64>,
    y: Option<i64>,
    map_id: Option<i64>,
) -> Result<serde_json::Value> {
    if map_id.is_some() && (x.is_some() || y.is_some()) {
        return Err(anyhow::anyhow!(
            "Provide either 'map_id' or both 'x' and 'y', but not both."
        ));
    }

    let mut json_body = serde_json::Map::new();

    if let (Some(x), Some(y)) = (x, y) {
        json_body.insert("x".to_string(), serde_json::json!(x));
        json_body.insert("y".to_string(), serde_json::json!(y));
    }

    if let Some(map_id) = map_id {
        json_body.insert("map_id".to_string(), serde_json::json!(map_id));
    }

    let json = serde_json::Value::Object(json_body).to_string();

    let span = info_span!("action_move", x, y, map_id);
    let _enter = span.enter();

    post(settings, &format!("/my/{}/action/move", name), &json).await
}

/// Execute a transition from the current map to another layer. The character must be on a map that has a transition available.
/// https://api.artifactsmmo.com/docs/#/operations/action_transition_my__name__action_transition_post
pub async fn action_transition(
    settings: &Settings,
    name: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("action_transition");
    let _enter = span.enter();

    post(settings, &format!("/my/{}/action/transition", name), "").await
}

/// Recovers hit points by resting. (1 second per 5 HP, minimum 3 seconds)
/// https://api.artifactsmmo.com/docs/#/operations/action_rest_my__name__action_rest_post
pub async fn action_rest(settings: &Settings, name: ValidatedString) -> Result<serde_json::Value> {
    let span = info_span!("action_rest");
    let _enter = span.enter();

    post(settings, &format!("/my/{}/action/rest", name), "").await
}

/// Equip an item on your character.
/// https://api.artifactsmmo.com/docs/#/operations/action_equip_item_my__name__action_equip_post
pub async fn action_equip_item(
    settings: &Settings,
    name: ValidatedString,
    code: ValidatedString,
    slot: EquipmentSlot,
    quantity: Option<i64>,
) -> Result<serde_json::Value> {
    if slot == EquipmentSlot::Utility1 || slot == EquipmentSlot::Utility2 {
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
/// https://api.artifactsmmo.com/docs/#/operations/action_unequip_item_my__name__action_unequip_post
pub async fn action_unequip_item(
    settings: &Settings,
    name: ValidatedString,
    slot: EquipmentSlot,
    quantity: Option<i64>,
) -> Result<serde_json::Value> {
    if slot == EquipmentSlot::Utility1 || slot == EquipmentSlot::Utility2 {
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
/// https://api.artifactsmmo.com/docs/#/operations/action_use_item_my__name__action_use_post
pub async fn action_use_item(
    settings: &Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let span = info_span!("action_use_item", code = %code, quantity);
    let _enter = span.enter();

    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post(settings, &format!("/my/{}/action/use", name), &json).await
}

/// Start a fight against a monster on the character's map. Add participants for multi-character fights (up to 3 characters, only for boss).
/// https://api.artifactsmmo.com/docs/#/operations/action_fight_my__name__action_fight_post
pub async fn action_fight(
    settings: &Settings,
    name: ValidatedString,
    participants: Option<Vec<ValidatedString>>,
) -> Result<serde_json::Value> {
    let span = info_span!("action_fight", participants = ?participants);
    let _enter = span.enter();

    // Construct the JSON body
    let participants_json = participants
        .map(|p| {
            p.into_iter()
                .map(|s| format!("\"{}\"", s))
                .collect::<Vec<String>>()
                .join(",")
        })
        .unwrap_or_else(|| "".to_string());

    let json = format!("{{\"participants\": [{}]}}", participants_json);

    post(settings, &format!("/my/{}/action/fight", name), &json).await
}

/// Harvest a resource on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_gathering_my__name__action_gathering_post
pub async fn action_gathering(
    settings: &Settings,
    name: &ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("action_gathering");
    let _enter = span.enter();
    post(settings, &format!("/my/{}/action/gathering", name), "").await
}

/// Crafting an item. The character must be on a map with a workshop.
/// https://api.artifactsmmo.com/docs/#/operations/action_crafting_my__name__action_crafting_post
pub async fn action_crafting(
    settings: &Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: Option<i64>,
) -> Result<serde_json::Value> {
    let span = info_span!("action_crafting", code = %code, quantity = quantity.unwrap_or(1));
    let _enter = span.enter();

    let json = format!(
        r#"{{"code": "{}", "quantity": {}}}"#,
        code,
        quantity.unwrap_or(1)
    );
    post(settings, &format!("/my/{}/action/crafting", name), &json).await
}

/// Deposit gold in a bank on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_deposit_bank_gold_my__name__action_bank_deposit_gold_post
pub async fn action_deposit_bank_gold(
    settings: &Settings,
    name: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let span = info_span!("action_deposit_bank_gold", quantity);
    let _enter = span.enter();

    let json = format!(r#"{{"quantity": {}}}"#, quantity);
    post(
        settings,
        &format!("/my/{}/action/bank/deposit/gold", name),
        &json,
    )
    .await
}

/// Deposit multiple items in a bank on the character's map. The cooldown will be 3 seconds multiplied by the number of different items withdrawn.
/// https://api.artifactsmmo.com/docs/#/operations/action_deposit_bank_item_my__name__action_bank_deposit_item_post
pub async fn action_deposit_bank_item(
    settings: &Settings,
    name: ValidatedString,
    items: Vec<(ValidatedString, i64)>,
) -> Result<serde_json::Value> {
    let span = info_span!("action_deposit_bank_item", items = ?items);
    let _enter = span.enter();

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
/// https://api.artifactsmmo.com/docs/#/operations/action_withdraw_bank_item_my__name__action_bank_withdraw_item_post
pub async fn action_withdraw_bank_item(
    settings: &Settings,
    name: ValidatedString,
    items: Vec<(ValidatedString, i64)>,
) -> Result<serde_json::Value> {
    let span = info_span!("action_withdraw_bank_item", items = ?items);
    let _enter = span.enter();

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
/// https://api.artifactsmmo.com/docs/#/operations/action_withdraw_bank_gold_my__name__action_bank_withdraw_gold_post
pub async fn action_withdraw_bank_gold(
    settings: &Settings,
    name: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let span = info_span!("action_withdraw_bank_gold", quantity);
    let _enter = span.enter();

    let json = format!(r#"{{"quantity": {}}}"#, quantity);
    post(
        settings,
        &format!("/my/{}/action/bank/withdraw/gold", name),
        &json,
    )
    .await
}

/// Buy a 25 slots bank expansion.
/// https://api.artifactsmmo.com/docs/#/operations/action_buy_bank_expansion_my__name__action_bank_buy_expansion_post
pub async fn action_buy_bank_expansion(
    settings: &Settings,
    name: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("action_buy_bank_expansion");
    let _enter = span.enter();

    post(
        settings,
        &format!("/my/{}/action/bank/buy_expansion", name),
        "",
    )
    .await
}

/// Buy an item from an NPC on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_npc_buy_item_my__name__action_npc_buy_post
pub async fn action_npc_buy_item(
    settings: &Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let span = info_span!("action_npc_buy_item", code = %code, quantity);
    let _enter = span.enter();

    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post(settings, &format!("/my/{}/action/npc/buy", name), &json).await
}

/// Sell an item to an NPC on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_npc_sell_item_my__name__action_npc_sell_post
pub async fn action_npc_sell_item(
    settings: &Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let span = info_span!("action_npc_sell_item", code = %code, quantity);
    let _enter = span.enter();

    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post(settings, &format!("/my/{}/action/npc/sell", name), &json).await
}

/// Recycling an item. The character must be on a map with a workshop (only for equipments and weapons).
/// https://api.artifactsmmo.com/docs/#/operations/action_recycling_my__name__action_recycling_post
pub async fn action_recycling(
    settings: &Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: Option<i64>,
) -> Result<serde_json::Value> {
    let span = info_span!("action_recycling", code = %code, quantity = quantity.unwrap_or(1));
    let _enter = span.enter();

    let json = format!(
        r#"{{"code": "{}", "quantity": {}}}"#,
        code,
        quantity.unwrap_or(1)
    );
    post(settings, &format!("/my/{}/action/recycling", name), &json).await
}

/// Buy an item at the Grand Exchange on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_ge_buy_item_my__name__action_grandexchange_buy_post
pub async fn action_grandexchange_buy_item(
    settings: &Settings,
    name: ValidatedString,
    id: String,
    quantity: i64,
) -> Result<serde_json::Value> {
    let span = info_span!("action_grandexchange_buy_item", id = %id, quantity);
    let _enter = span.enter();

    let json = format!(r#"{{"id": "{}", "quantity": {}}}"#, id, quantity);
    post(
        settings,
        &format!("/my/{}/action/grandexchange/buy", name),
        &json,
    )
    .await
}

/// Create a sell order at the Grand Exchange on the character's map. Please note there is a 3% listing tax, charged at the time of posting, on the total price.
/// https://api.artifactsmmo.com/docs/#/operations/action_ge_create_sell_order_my__name__action_grandexchange_sell_post
pub async fn action_grandexchange_create_sell_order(
    settings: &Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: i64,
    price: i64,
) -> Result<serde_json::Value> {
    let span = info_span!(
        "action_grandexchange_create_sell_order",
        code = %code,
        quantity,
        price
    );
    let _enter = span.enter();

    let json = format!(
        r#"{{"code": "{}", "price": {}, "quantity": {}}}"#,
        code, price, quantity
    );

    post(
        settings,
        &format!("/my/{}/action/grandexexchange/sell", name),
        &json,
    )
    .await
}

/// Cancel a sell order at the Grand Exchange on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_ge_cancel_sell_order_my__name__action_grandexchange_cancel_post
pub async fn action_grandexchange_cancel_sell_order(
    settings: &Settings,
    name: ValidatedString,
    id: String,
) -> Result<serde_json::Value> {
    let span = info_span!("action_grandexchange_cancel_sell_order", id = %id);
    let _enter = span.enter();

    let json = format!(r#"{{"id": "{}"}}"#, id);

    post(
        settings,
        &format!("/my/{}/action/grandexchange/cancel", name),
        &json,
    )
    .await
}

/// Complete a task.
/// https://api.artifactsmmo.com/docs/#/operations/action_complete_task_my__name__action_task_complete_post
pub async fn action_complete_task(
    settings: &Settings,
    name: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("action_complete_task");
    let _enter = span.enter();
    post(settings, &format!("/my/{}/action/task/complete", name), "").await
}

/// Exchange 6 tasks coins for a random reward. Rewards are exclusive items or resources.
/// https://api.artifactsmmo.com/docs/#/operations/action_task_exchange_my__name__action_task_exchange_post
pub async fn action_task_exchange(
    settings: &Settings,
    name: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("action_task_exchange");
    let _enter = span.enter();
    post(settings, &format!("/my/{}/action/task/exchange", name), "").await
}

/// Accepting a new task.
/// https://api.artifactsmmo.com/docs/#/operations/action_accept_new_task_my__name__action_task_new_post
pub async fn action_accept_new_task(
    settings: &Settings,
    name: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("action_accept_new_task");
    let _enter = span.enter();
    post(settings, &format!("/my/{}/action/task/new", name), "").await
}

/// Trading items with a Tasks Master.
/// https://api.artifactsmmo.com/docs/#/operations/action_task_trade_my__name__action_task_trade_post
pub async fn action_task_trade(
    settings: &Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let span = info_span!("action_task_trade", code = %code, quantity);
    let _enter = span.enter();

    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post(settings, &format!("/my/{}/action/task/trade", name), &json).await
}

/// Cancel a task for 1 tasks coin.
/// https://api.artifactsmmo.com/docs/#/operations/action_task_cancel_my__name__action_task_cancel_post
pub async fn action_cancel_task(
    settings: &Settings,
    name: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("action_cancel_task");
    let _enter = span.enter();
    post(settings, &format!("/my/{}/action/task/cancel", name), "").await
}

/// Give gold to another character in your account on the same map.
/// https://api.artifactsmmo.com/docs/#/operations/action_give_gold_my__name__action_give_gold_post
pub async fn action_give_gold(
    settings: &Settings,
    name: ValidatedString,
    quantity: i64,
    character: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("action_give_gold", quantity, character = %character);
    let _enter = span.enter();

    let json = format!(
        r#"{{"quantity": {}, "character": "{}"}}"#,
        quantity, character
    );
    post(settings, &format!("/my/{}/action/give/gold", name), &json).await
}

/// Give items to another character in your account on the same map. The cooldown will be 3 seconds multiplied by the number of different items given.
/// https://api.artifactsmmo.com/docs/#/operations/action_give_items_my__name__action_give_item_post
pub async fn action_give_item(
    settings: &Settings,
    name: ValidatedString,
    items: Vec<(ValidatedString, i64)>,
    character: ValidatedString,
) -> Result<serde_json::Value> {
    let span = info_span!("action_give_item", items = ?items, character = %character);
    let _enter = span.enter();

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
/// https://api.artifactsmmo.com/docs/#/operations/action_delete_item_my__name__action_delete_post
pub async fn action_delete_item(
    settings: &Settings,
    name: ValidatedString,
    code: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let span = info_span!("action_delete_item", code = %code, quantity);
    let _enter = span.enter();

    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post(settings, &format!("/my/{}/action/delete", name), &json).await
}

/// Change the skin of your character.
/// https://api.artifactsmmo.com/docs/#/operations/action_change_skin_my__name__action_change_skin_post
pub async fn action_change_skin(
    settings: &Settings,
    name: ValidatedString,
    skin: SkinType,
) -> Result<serde_json::Value> {
    let span = info_span!("action_change_skin", skin = %skin);
    let _enter = span.enter();

    let json = format!(r#"{{"skin": "{}"}}"#, skin.to_string());
    post(settings, &format!("/my/{}/action/change_skin", name), &json).await
}

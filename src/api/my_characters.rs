use anyhow::Result;

use crate::{
    api::utils::{get, post_action},
    types::{
        common::{
            pagination_params::PaginationParams, settings::Settings,
            validated_string::ValidatedString,
        },
        game::{character::Character, equipment_slot::EquipmentSlot, skin_type::SkinType},
    },
};

/// History of the last 250 actions of all your characters.
/// https://api.artifactsmmo.com/docs/#/operations/get_all_characters_logs_my_logs_get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_all_characters_logs(
    settings: &Settings,
    pagination: Option<PaginationParams>,
) -> Result<serde_json::Value> {
    let mut query_params = Vec::new();

    if let Some(pagination) = &pagination {
        query_params.extend(pagination.to_query_params());
    }

    get(settings, &format!("/my/logs"), Some(query_params)).await
}

/// History of the last actions of your character.
/// https://api.artifactsmmo.com/docs/#/operations/get_character_logs_my_logs__name__get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_character_logs(
    settings: &Settings,
    character_name: ValidatedString,
) -> Result<serde_json::Value> {
    get(settings, &format!("/my/logs/{}", character_name), None).await
}

/// List of your characters.
/// https://api.artifactsmmo.com/docs/#/operations/get_my_characters_my_characters_get
#[tracing::instrument(skip(settings), target = "http")]
pub async fn get_my_characters(settings: &Settings) -> Result<serde_json::Value> {
    get(settings, "/my/characters", None).await
}

/// Moves a character on the map using either the map's ID or X and Y position. Provide either 'map_id' or both 'x' and 'y' coordinates in the request body.
/// https://api.artifactsmmo.com/docs/#/operations/action_move_my__name__action_move_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_move(
    settings: &Settings,
    character: &mut Character,
    x: Option<i64>,
    y: Option<i64>,
    map_id: Option<i64>,
) -> Result<serde_json::Value> {
    let mut json_body = serde_json::Map::new();

    if let (Some(x), Some(y)) = (x, y) {
        json_body.insert("x".to_string(), serde_json::json!(x));
        json_body.insert("y".to_string(), serde_json::json!(y));
    }

    if let Some(map_id) = map_id {
        json_body.insert("map_id".to_string(), serde_json::json!(map_id));
    }

    let json = serde_json::Value::Object(json_body).to_string();

    post_action(
        settings,
        character,
        &format!("/my/{}/action/move", character.name),
        &json,
    )
    .await
}

/// Execute a transition from the current map to another layer. The character must be on a map that has a transition available.
/// https://api.artifactsmmo.com/docs/#/operations/action_transition_my__name__action_transition_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_transition(
    settings: &Settings,
    character: &mut Character,
) -> Result<serde_json::Value> {
    post_action(
        settings,
        character,
        &format!("/my/{}/action/transition", character.name),
        "",
    )
    .await
}

/// Recovers hit points by resting. (1 second per 5 HP, minimum 3 seconds)
/// https://api.artifactsmmo.com/docs/#/operations/action_rest_my__name__action_rest_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_rest(
    settings: &Settings,
    character: &mut Character,
) -> Result<serde_json::Value> {
    post_action(
        settings,
        character,
        &format!("/my/{}/action/rest", character.name),
        "",
    )
    .await
}

/// Equip an item on your character.
/// https://api.artifactsmmo.com/docs/#/operations/action_equip_item_my__name__action_equip_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_equip_item(
    settings: &Settings,
    character: &mut Character,
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
    post_action(
        settings,
        character,
        &format!("/my/{}/action/equip", character.name),
        &json,
    )
    .await
}

/// Unequip an item on your character.
/// https://api.artifactsmmo.com/docs/#/operations/action_unequip_item_my__name__action_unequip_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_unequip_item(
    settings: &Settings,
    character: &mut Character,
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
    post_action(
        settings,
        character,
        &format!("/my/{}/action/unequip", character.name),
        &json,
    )
    .await
}

/// Use an item as a consumable.
/// https://api.artifactsmmo.com/docs/#/operations/action_use_item_my__name__action_use_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_use_item(
    settings: &Settings,
    character: &mut Character,
    code: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post_action(
        settings,
        character,
        &format!("/my/{}/action/use", character.name),
        &json,
    )
    .await
}

/// Start a fight against a monster on the character's map. Add participants for multi-character fights (up to 3 characters, only for boss).
/// https://api.artifactsmmo.com/docs/#/operations/action_fight_my__name__action_fight_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_fight(
    settings: &Settings,
    character: &mut Character,
    participants: Option<Vec<ValidatedString>>,
) -> Result<serde_json::Value> {
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

    post_action(
        settings,
        character,
        &format!("/my/{}/action/fight", character.name),
        &json,
    )
    .await
}

/// Harvest a resource on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_gathering_my__name__action_gathering_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_gathering(
    settings: &Settings,
    character: &mut Character,
) -> Result<serde_json::Value> {
    post_action(
        settings,
        character,
        &format!("/my/{}/action/gathering", character.name),
        "",
    )
    .await
}

/// Crafting an item. The character must be on a map with a workshop.
/// https://api.artifactsmmo.com/docs/#/operations/action_crafting_my__name__action_crafting_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_crafting(
    settings: &Settings,
    character: &mut Character,
    code: ValidatedString,
    quantity: Option<i64>,
) -> Result<serde_json::Value> {
    let json = format!(
        r#"{{"code": "{}", "quantity": {}}}"#,
        code,
        quantity.unwrap_or(1)
    );
    post_action(
        settings,
        character,
        &format!("/my/{}/action/crafting", character.name),
        &json,
    )
    .await
}

/// Deposit gold in a bank on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_deposit_bank_gold_my__name__action_bank_deposit_gold_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_deposit_bank_gold(
    settings: &Settings,
    character: &mut Character,
    quantity: i64,
) -> Result<serde_json::Value> {
    let json = format!(r#"{{"quantity": {}}}"#, quantity);
    post_action(
        settings,
        character,
        &format!("/my/{}/action/bank/deposit/gold", character.name),
        &json,
    )
    .await
}

/// Deposit multiple items in a bank on the character's map. The cooldown will be 3 seconds multiplied by the number of different items withdrawn.
/// https://api.artifactsmmo.com/docs/#/operations/action_deposit_bank_item_my__name__action_bank_deposit_item_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_deposit_bank_item(
    settings: &Settings,
    character: &mut Character,
    items: Vec<(ValidatedString, i64)>,
) -> Result<serde_json::Value> {
    let items_json: Vec<String> = items
        .into_iter()
        .map(|(code, quantity)| format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity))
        .collect();

    let json_string = format!("[{}]", items_json.join(","));

    post_action(
        settings,
        character,
        &format!("/my/{}/action/bank/deposit/item", character.name),
        &json_string,
    )
    .await
}

/// Take items from your bank and put them in the character's inventory. The cooldown will be 3 seconds multiplied by the number of different items withdrawn.
/// https://api.artifactsmmo.com/docs/#/operations/action_withdraw_bank_item_my__name__action_bank_withdraw_item_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_withdraw_bank_item(
    settings: &Settings,
    character: &mut Character,
    items: Vec<(ValidatedString, i64)>,
) -> Result<serde_json::Value> {
    let items_json: Vec<String> = items
        .into_iter()
        .map(|(code, quantity)| format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity))
        .collect();

    let json_string = format!("[{}]", items_json.join(","));

    post_action(
        settings,
        character,
        &format!("/my/{}/action/bank/withdraw/item", character.name),
        &json_string,
    )
    .await
}

/// Withdraw gold from your bank.
/// https://api.artifactsmmo.com/docs/#/operations/action_withdraw_bank_gold_my__name__action_bank_withdraw_gold_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_withdraw_bank_gold(
    settings: &Settings,
    character: &mut Character,
    quantity: i64,
) -> Result<serde_json::Value> {
    let json = format!(r#"{{"quantity": {}}}"#, quantity);
    post_action(
        settings,
        character,
        &format!("/my/{}/action/bank/withdraw/gold", character.name),
        &json,
    )
    .await
}

/// Buy a 25 slots bank expansion.
/// https://api.artifactsmmo.com/docs/#/operations/action_buy_bank_expansion_my__name__action_bank_buy_expansion_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_buy_bank_expansion(
    settings: &Settings,
    character: &mut Character,
) -> Result<serde_json::Value> {
    post_action(
        settings,
        character,
        &format!("/my/{}/action/bank/buy_expansion", character.name),
        "",
    )
    .await
}

/// Buy an item from an NPC on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_npc_buy_item_my__name__action_npc_buy_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_npc_buy_item(
    settings: &Settings,
    character: &mut Character,
    code: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post_action(
        settings,
        character,
        &format!("/my/{}/action/npc/buy", character.name),
        &json,
    )
    .await
}

/// Sell an item to an NPC on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_npc_sell_item_my__name__action_npc_sell_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_npc_sell_item(
    settings: &Settings,
    character: &mut Character,
    code: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post_action(
        settings,
        character,
        &format!("/my/{}/action/npc/sell", character.name),
        &json,
    )
    .await
}

/// Recycling an item. The character must be on a map with a workshop (only for equipments and weapons).
/// https://api.artifactsmmo.com/docs/#/operations/action_recycling_my__name__action_recycling_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_recycling(
    settings: &Settings,
    character: &mut Character,
    code: ValidatedString,
    quantity: Option<i64>,
) -> Result<serde_json::Value> {
    let json = format!(
        r#"{{"code": "{}", "quantity": {}}}"#,
        code,
        quantity.unwrap_or(1)
    );
    post_action(
        settings,
        character,
        &format!("/my/{}/action/recycling", character.name),
        &json,
    )
    .await
}

/// Buy an item at the Grand Exchange on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_ge_buy_item_my__name__action_grandexchange_buy_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_grandexchange_buy_item(
    settings: &Settings,
    character: &mut Character,
    id: String,
    quantity: i64,
) -> Result<serde_json::Value> {
    let json = format!(r#"{{"id": "{}", "quantity": {}}}"#, id, quantity);
    post_action(
        settings,
        character,
        &format!("/my/{}/action/grandexchange/buy", character.name),
        &json,
    )
    .await
}

/// Create a sell order at the Grand Exchange on the character's map. Please note there is a 3% listing tax, charged at the time of posting, on the total price.
/// https://api.artifactsmmo.com/docs/#/operations/action_ge_create_sell_order_my__name__action_grandexchange_sell_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_grandexchange_create_sell_order(
    settings: &Settings,
    character: &mut Character,
    code: ValidatedString,
    quantity: i64,
    price: i64,
) -> Result<serde_json::Value> {
    let json = format!(
        r#"{{"code": "{}", "price": {}, "quantity": {}}}"#,
        code, price, quantity
    );

    post_action(
        settings,
        character,
        &format!(
            "/my/{}/action/grandexchange/create-sell-order",
            character.name
        ),
        &json,
    )
    .await
}

/// Cancel a sell order at the Grand Exchange on the character's map.
/// https://api.artifactsmmo.com/docs/#/operations/action_ge_cancel_sell_order_my__name__action_grandexchange_cancel_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_grandexchange_cancel_sell_order(
    settings: &Settings,
    character: &mut Character,
    id: String,
) -> Result<serde_json::Value> {
    let json = format!(r#"{{"id": "{}"}}"#, id);

    post_action(
        settings,
        character,
        &format!("/my/{}/action/grandexchange/cancel", character.name),
        &json,
    )
    .await
}

/// Create a buy order at the Grand Exchange on the character's map. The total gold (price * quantity) is locked when creating the order. Other players can then sell items to fulfill your order. Items will be delivered to your pending items when the order is filled.
/// https://api.artifactsmmo.com/docs/#/operations/action_ge_create_buy_order_my__name__action_grandexchange_create_buy_order_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_grandexchange_create_buy_order(
    settings: &Settings,
    character: &mut Character,
    code: ValidatedString,
    quantity: i64,
    price: i64,
) -> Result<serde_json::Value> {
    let json = format!(
        r#"{{"code": "{}", "price": {}, "quantity": {}}}"#,
        code, price, quantity
    );

    post_action(
        settings,
        character,
        &format!(
            "/my/{}/action/grandexexchange/create-buy-order",
            character.name
        ),
        &json,
    )
    .await
}

/// Sell items to an existing buy order at the Grand Exchange on the character's map. You will receive the gold immediately. The buyer will receive the items in their pending items.
/// https://api.artifactsmmo.com/docs/#/operations/action_ge_fill_my__name__action_grandexchange_fill_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_grandexchange_fill(
    settings: &Settings,
    character: &mut Character,
    id: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let json = format!(r#"{{"id": "{}", "quantity": {}}}"#, id, quantity);

    post_action(
        settings,
        character,
        &format!("/my/{}/action/grandexchange/fill", character.name),
        &json,
    )
    .await
}

/// Complete a task.
/// https://api.artifactsmmo.com/docs/#/operations/action_complete_task_my__name__action_task_complete_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_complete_task(
    settings: &Settings,
    character: &mut Character,
) -> Result<serde_json::Value> {
    post_action(
        settings,
        character,
        &format!("/my/{}/action/task/complete", character.name),
        "",
    )
    .await
}

/// Exchange 6 tasks coins for a random reward. Rewards are exclusive items or resources.
/// https://api.artifactsmmo.com/docs/#/operations/action_task_exchange_my__name__action_task_exchange_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_task_exchange(
    settings: &Settings,
    character: &mut Character,
) -> Result<serde_json::Value> {
    post_action(
        settings,
        character,
        &format!("/my/{}/action/task/exchange", character.name),
        "",
    )
    .await
}

/// Accepting a new task.
/// https://api.artifactsmmo.com/docs/#/operations/action_accept_new_task_my__name__action_task_new_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_accept_new_task(
    settings: &Settings,
    character: &mut Character,
) -> Result<serde_json::Value> {
    post_action(
        settings,
        character,
        &format!("/my/{}/action/task/new", character.name),
        "",
    )
    .await
}

/// Trading items with a Tasks Master.
/// https://api.artifactsmmo.com/docs/#/operations/action_task_trade_my__name__action_task_trade_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_task_trade(
    settings: &Settings,
    character: &mut Character,
    code: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post_action(
        settings,
        character,
        &format!("/my/{}/action/task/trade", character.name),
        &json,
    )
    .await
}

/// Cancel a task for 1 tasks coin.
/// https://api.artifactsmmo.com/docs/#/operations/action_task_cancel_my__name__action_task_cancel_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_cancel_task(
    settings: &Settings,
    character: &mut Character,
) -> Result<serde_json::Value> {
    post_action(
        settings,
        character,
        &format!("/my/{}/action/task/cancel", character.name),
        "",
    )
    .await
}

/// Give gold to another character in your account on the same map.
/// https://api.artifactsmmo.com/docs/#/operations/action_give_gold_my__name__action_give_gold_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_give_gold(
    settings: &Settings,
    character: &mut Character,
    quantity: i64,
    target_character: ValidatedString,
) -> Result<serde_json::Value> {
    let json = format!(
        r#"{{"quantity": {}, "character": "{}"}}"#,
        quantity, target_character
    );
    post_action(
        settings,
        character,
        &format!("/my/{}/action/give/gold", character.name),
        &json,
    )
    .await
}

/// Give items to another character in your account on the same map. The cooldown will be 3 seconds multiplied by the number of different items given.
/// https://api.artifactsmmo.com/docs/#/operations/action_give_items_my__name__action_give_item_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_give_item(
    settings: &Settings,
    character: &mut Character,
    items: Vec<(ValidatedString, i64)>,
    target_character: ValidatedString,
) -> Result<serde_json::Value> {
    let items_json: Vec<String> = items
        .into_iter()
        .map(|(code, quantity)| format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity))
        .collect();

    let json_string = format!(
        r#"{{"items": [{}], "character": "{}"}}"#,
        items_json.join(","),
        target_character
    );

    post_action(
        settings,
        character,
        &format!("/my/{}/action/give/item", character.name),
        &json_string,
    )
    .await
}

/// Claim a pending item with a specific character.
/// https://api.artifactsmmo.com/docs/#/operations/action_claim_pending_item_my__name__action_claim_item__id__post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_claim_pending_item(
    settings: &Settings,
    character: &mut Character,
    id: &ValidatedString,
) -> Result<serde_json::Value> {
    post_action(
        settings,
        character,
        &format!("/my/{}/action/claim_item/{}", character.name, id),
        "",
    )
    .await
}

/// Delete an item from your character's inventory.
/// https://api.artifactsmmo.com/docs/#/operations/action_delete_item_my__name__action_delete_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_delete_item(
    settings: &Settings,
    character: &mut Character,
    code: ValidatedString,
    quantity: i64,
) -> Result<serde_json::Value> {
    let json = format!(r#"{{"code": "{}", "quantity": {}}}"#, code, quantity);
    post_action(
        settings,
        character,
        &format!("/my/{}/action/delete", character.name),
        &json,
    )
    .await
}

/// Change the skin of your character.
/// https://api.artifactsmmo.com/docs/#/operations/action_change_skin_my__name__action_change_skin_post
#[tracing::instrument(skip(settings, character), target = "http", fields(character = %character.name))]
pub async fn action_change_skin(
    settings: &Settings,
    character: &mut Character,
    skin: SkinType,
) -> Result<serde_json::Value> {
    let json = format!(r#"{{"skin": "{}"}}"#, skin.to_string());
    post_action(
        settings,
        character,
        &format!("/my/{}/action/change_skin", character.name),
        &json,
    )
    .await
}

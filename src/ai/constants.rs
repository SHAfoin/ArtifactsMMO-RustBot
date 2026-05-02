pub const MINIMUM_HEALTH_TO_REST: i64 = 30; // En dessous de ce pourcentage de santé, le bot se soigne via Rest

//TODO ajouter un paramètre "Bot" dans les execute pour pouvoir l'impacter ? Ou le mettre en const ? c'est possible même ?

// enum DebugCharacterInfos {
//     Health = 100,
//     PositionX = 1,  // pourrait ne pas être un fact mais une variable à part dans le bot
//     PositionY = -2, // pourrait ne pas être un fact mais une variable à part dans le bot
//     TargetId = "chicken", // pourrait ne pas être un fact mais une variable à part dans le bot
//     PositionTargetX = 5, // pourrait ne pas être un fact mais une variable à part dans le bot
//     PositionTargetY = -1, // pourrait ne pas être un fact mais une variable à part dans le bot
// }
pub const DEBUG_TARGET_ID: &str = "target_1"; // ID de la cible à attaquer (brouillon : même cible à chaque fois)

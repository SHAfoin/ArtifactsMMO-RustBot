use crate::types::ai::utility_ai::UtilityAiVariables;

pub struct CharacterAdditionnalInfo {
    //TODO Ici on peut mettre des infos supplémentaires sur le personnage qui ne viennent pas de l'API, mais qui peuvent être utiles pour l'IA
    // Par exemple : les ressources à collecter, les monstres à farmer, les équipements à trouver...
    pub target_id: String,                        // cible actuelle à attaquer
    pub position_target_x: i32,                   // position x de la cible actuelle
    pub position_target_y: i32,                   // position y de la cible actuelle
    pub utility_ai_variables: UtilityAiVariables, // variables utilisées pour le calcul de l'utilité dans l'Utility AI
    pub priority_target: Option<String>, // cible manuelle prioritaire (exemple : si je dis manuellement de buter ça, ou si on a une quête qui demande de buter un monstre précis) IL FAUT QU'A UN MOMENT ça REDEVIENNE NONE, sinon on va toujours attaquer la même cible, genre quand la quête est finie
}

impl CharacterAdditionnalInfo {
    pub fn new() -> Self {
        Self {
            target_id: String::new(),
            position_target_x: 0,
            position_target_y: 0,
            utility_ai_variables: UtilityAiVariables::new(),
            priority_target: None,
        }
    }
}

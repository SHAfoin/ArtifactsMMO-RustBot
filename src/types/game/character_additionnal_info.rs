use crate::types::ai::utility_ai::UtilityAiVariables;

pub struct CharacterAdditionnalInfo {
    //TODO Ici on peut mettre des infos supplémentaires sur le personnage qui ne viennent pas de l'API, mais qui peuvent être utiles pour l'IA
    // Par exemple : les ressources à collecter, les monstres à farmer, les équipements à trouver...
    targetID: String,
    positionTargetX: i32,
    positionTargetY: i32,
    utilityAiVariables: UtilityAiVariables,
}

impl CharacterAdditionnalInfo {
    pub fn new() -> Self {
        Self {
            targetID: String::new(),
            positionTargetX: 0,
            positionTargetY: 0,
            utilityAiVariables: UtilityAiVariables::new(),
        }
    }
}

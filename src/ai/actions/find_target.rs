use crate::{
    ai::goap::{Action, ActionStatus, WorldState},
    types::{
        ai::agent_facts::AgentFact,
        game::{character::Character, character_additionnal_info::CharacterAdditionnalInfo},
    },
};

pub struct FindTarget;

impl Action<AgentFact> for FindTarget {
    fn name(&self) -> &str {
        "FindTarget"
    }

    fn effects(&self) -> WorldState<AgentFact> {
        let mut ws = WorldState::new();
        ws.set(AgentFact::TargetReady, true);
        ws.set(AgentFact::NeedEquipment, true);
        ws
    }

    fn execute(
        &mut self,
        state: &mut WorldState<AgentFact>,
        character: &mut Character,
        additionnal_info: &mut CharacterAdditionnalInfo,
    ) -> ActionStatus {
        let mut cible = Some("");
        if let Some(cible_prioritaire) = &additionnal_info.priority_target {
            cible = Some(cible_prioritaire);
        } else {
            cible = find_best_target(character);
        }

        if let Some(id) = cible {
            println!("  -> Cible trouvee : {}", id);
            additionnal_info.target_id = id.to_string();
            state.set(AgentFact::TargetReady, true);
            return ActionStatus::Success;
        } else {
            println!("  -> Aucune cible trouvee.");
            state.set(AgentFact::TargetReady, false);
            return ActionStatus::Failure;
        }
    }
}

/// TODO regarder cibles autour de mon niveau ET atteignables
pub fn find_best_target(_: &Character) -> Option<&str> {
    Some("chicken")
}

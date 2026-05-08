use crate::{
    ai::goap::{Action, ActionStatus, Condition, FactValue, WorldState},
    types::{
        ai::agent_facts::AgentFact,
        common::settings::{self, Settings},
        game::{character::Character, character_additionnal_info::CharacterAdditionnalInfo},
    },
};

pub struct FindEquipment;

impl Action<AgentFact> for FindEquipment {
    fn name(&self) -> &str {
        "FindEquipment"
    }

    fn preconditions(&self) -> WorldState<AgentFact> {
        let mut ws = WorldState::new();
        ws.require(
            AgentFact::TargetReady,
            Condition::Equals(FactValue::Bool(true)),
        );
        ws.require(
            AgentFact::NeedEquipment,
            Condition::Equals(FactValue::Bool(true)),
        );
        ws
    }

    fn effects(&self) -> WorldState<AgentFact> {
        let mut ws = WorldState::new();
        ws.set(AgentFact::TargetReady, true);
        ws.set(AgentFact::NeedEquipment, false);
        ws
    }

    fn execute(
        &mut self,
        state: &mut WorldState<AgentFact>,
        settings: &Settings,
        character: &mut Character,
        additionnal_info: &mut CharacterAdditionnalInfo,
    ) -> ActionStatus {
        if let Some(need_equipment) = state.get(&AgentFact::NeedEquipment) {
            if need_equipment == &FactValue::Bool(true) {
                println!("  -> Besoin d'equipement, recherche en cours...");
                find_best_equipment(character);
                println!("  -> Equipement trouve.");
                state.set(AgentFact::NeedEquipment, false);
            }
        } else {
            println!("  -> Fact 'NeedEquipment' non trouvé dans l'état du monde.");
            return ActionStatus::Failure;
        }
        ActionStatus::Success
    }
}

///TODO trouver best equipment avec simulation de combat etc
pub fn find_best_equipment(_: &Character) {}

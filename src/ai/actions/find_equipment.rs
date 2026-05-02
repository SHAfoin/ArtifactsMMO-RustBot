use crate::{
    ai::goap::{Action, ActionStatus, Condition, FactValue, WorldState},
    types::{
        ai::agent_facts::AgentFact,
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
        character: &mut Character,
        additionnal_info: &mut CharacterAdditionnalInfo,
    ) -> ActionStatus {
        //TODO Si needEquipment : fonction "Trouver l'equipement"
        // Sinon : return
        // Brouillon : pas d'equipement,
        println!("  -> Cible trouvee.");
        ActionStatus::Success
    }
}

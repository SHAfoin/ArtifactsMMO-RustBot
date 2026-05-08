use crate::{
    ai::goap::{Action, ActionStatus, Condition, FactValue, WorldState},
    types::{
        ai::agent_facts::AgentFact,
        common::settings::{self, Settings},
        game::{character::Character, character_additionnal_info::CharacterAdditionnalInfo},
    },
};

pub struct AttackEnemy;

impl Action<AgentFact> for AttackEnemy {
    fn name(&self) -> &str {
        "AttackEnemy"
    }

    fn preconditions(&self) -> WorldState<AgentFact> {
        let mut ws = WorldState::new();
        ws.require(
            AgentFact::TargetInRange,
            Condition::Equals(FactValue::Bool(true)),
        );
        ws.require(
            AgentFact::TargetReady,
            Condition::Equals(FactValue::Bool(true)),
        );
        ws.require(
            AgentFact::NeedEquipment,
            Condition::Equals(FactValue::Bool(false)),
        );
        ws
    }

    fn effects(&self) -> WorldState<AgentFact> {
        let mut ws = WorldState::new();
        ws.set(AgentFact::TargetAttacked, true);
        ws
    }

    fn cost(&self) -> f64 {
        1.0
    }

    fn execute(
        &mut self,
        state: &mut WorldState<AgentFact>,
        settings: &Settings,
        character: &mut Character,
        additionnal_info: &mut CharacterAdditionnalInfo,
    ) -> ActionStatus {
        //TODO Attaquer la Target
        //TODO Reset tout ici ou le faire dans la gameloop quand on calcule les facts ?
        state.set(AgentFact::TargetReady, false);
        state.set(AgentFact::NeedEquipment, false);
        state.set(AgentFact::TargetInRange, false);
        println!("  -> Attaque fatale: ennemi elimine.");
        ActionStatus::Success
    }
}

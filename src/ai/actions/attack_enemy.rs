use tokio::task;

use crate::{
    ai::goap::{Action, ActionStatus, Condition, FactValue, WorldState},
    api::my_characters::action_fight,
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
        let result = task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { action_fight(&settings, character, None).await })
        });

        if let Err(e) = result {
            match e {
                _ => return ActionStatus::Failure,
            }
        }
        println!("  -> Attaque fatale: ennemi elimine.");
        ActionStatus::Success
    }
}

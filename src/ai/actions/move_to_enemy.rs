use tokio::task;

use crate::{
    ai::goap::{Action, ActionStatus, Condition, FactValue, WorldState},
    api::my_characters::action_move,
    types::{
        ai::agent_facts::AgentFact,
        common::settings::Settings,
        game::{character::Character, character_additionnal_info::CharacterAdditionnalInfo},
    },
};

pub struct MoveToEnemy;

impl Action<AgentFact> for MoveToEnemy {
    fn name(&self) -> &str {
        "MoveToEnemy"
    }

    fn preconditions(&self) -> WorldState<AgentFact> {
        let mut ws = WorldState::new();
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
        ws.set(AgentFact::TargetInRange, true);
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
        println!("  -> Le bot se rapproche de la Target.");
        let x = additionnal_info.position_target_x;
        let y = additionnal_info.position_target_y;

        let result = task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { action_move(&settings, character, Some(x), Some(y), None).await })
        });

        if let Err(e) = result {
            match e {
                490 => {}
                _ => return ActionStatus::Failure,
            }
        }
        ActionStatus::Success
    }
}

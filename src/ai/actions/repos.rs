use tokio::task;

use crate::{
    ai::goap::{Action, ActionStatus, WorldState},
    api::my_characters::action_rest,
    types::{
        ai::agent_facts::AgentFact,
        common::settings::Settings,
        game::{character::Character, character_additionnal_info::CharacterAdditionnalInfo},
    },
};

pub struct Repos;

impl Action<AgentFact> for Repos {
    fn name(&self) -> &str {
        "Repos"
    }

    fn preconditions(&self) -> WorldState<AgentFact> {
        let ws = WorldState::new();
        ws
    }

    fn effects(&self) -> WorldState<AgentFact> {
        let mut ws = WorldState::new();
        ws.set(AgentFact::Health, 100i32);
        ws
    }

    fn cost(&self) -> f64 {
        1.0
    }

    fn execute(
        &mut self,
        _: &mut WorldState<AgentFact>,
        settings: &Settings,
        character: &mut Character,
        _: &mut CharacterAdditionnalInfo,
    ) -> ActionStatus {
        let _ = task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { action_rest(&settings, character).await })
        });
        println!("  -> Soins par repos: sante restauree.");
        ActionStatus::Success
    }
}

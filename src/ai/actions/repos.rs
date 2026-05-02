use crate::{
    ai::goap::{Action, ActionStatus, WorldState},
    types::{
        ai::agent_facts::AgentFact,
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
        state: &mut WorldState<AgentFact>,
        character: &mut Character,
        additionnal_info: &mut CharacterAdditionnalInfo,
    ) -> ActionStatus {
        //TODO Soins par repos
        println!("  -> Soins par repos: sante restauree.");
        ActionStatus::Success
    }
}

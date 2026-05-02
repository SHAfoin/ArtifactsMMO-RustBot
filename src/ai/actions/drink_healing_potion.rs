use crate::{
    ai::{
        constants::*,
        goap::{Action, ActionStatus, Condition, FactValue, WorldState},
    },
    types::ai::agent_facts::AgentFact,
};

pub struct DrinkHealingPotion;

impl Action<AgentFact> for DrinkHealingPotion {
    fn name(&self) -> &str {
        "DrinkHealingPotion"
    }

    fn preconditions(&self) -> WorldState<AgentFact> {
        let mut ws = WorldState::new();
        ws.require(
            AgentFact::Health,
            Condition::GreaterThan(FactValue::Int(MINIMUM_HEALTH_TO_REST)),
        );
        ws.require(AgentFact::HasPotion, Condition::Equals(true.into()));
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

    fn execute(&mut self, state: &mut WorldState<AgentFact>) -> ActionStatus {
        //TODO Boire potion de soin
        println!("  -> Boire potion de soin: sante restauree.");
        ActionStatus::Success
    }
}

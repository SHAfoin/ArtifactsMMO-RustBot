use crate::{
    ai::goap::{Action, ActionStatus, Condition, FactValue, WorldState},
    types::ai::agent_facts::AgentFact,
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

    fn cost(&self) -> f64 {
        1.0
    }

    fn execute(&mut self, state: &mut WorldState<AgentFact>) -> ActionStatus {
        //TODO Si needEquipment : fonction "Trouver l'equipement"
        // Sinon : return
        // Brouillon : pas d'equipement,
        println!("  -> Cible trouvee.");
        ActionStatus::Success
    }
}

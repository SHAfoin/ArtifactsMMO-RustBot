use crate::{
    ai::goap::{Condition, FactValue, Goal, WorldState},
    types::ai::agent_facts::AgentFact,
};

fn heal_score(state: &WorldState<AgentFact>) -> f64 {
    let hp = match state.get(&AgentFact::Health) {
        Some(FactValue::Int(v)) => *v as f64,
        _ => 0.0,
    };
    let max_hp = 100.0;
    let ratio = (1.0 - hp / max_hp).clamp(0.0, 1.0);
    ratio * ratio
}

pub fn heal_goal() -> Goal<AgentFact> {
    let mut heal_goal_state = WorldState::new();
    heal_goal_state.require(AgentFact::Health, Condition::LessThan(FactValue::Int(100)));

    Goal {
        name: "Soin",
        desired_state: heal_goal_state,
        priority: 2.0,
        score_fn: heal_score,
    }
}

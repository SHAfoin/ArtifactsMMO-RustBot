use crate::{
    ai::goap::{Condition, FactValue, Goal, WorldState},
    types::ai::agent_facts::AgentFact,
};

fn attack_score(state: &WorldState<AgentFact>) -> f64 {
    let enemy_hp = match state.get(&AgentFact::EnemyHealth) {
        Some(FactValue::Int(v)) => *v as f64,
        _ => 0.0,
    };
    (enemy_hp / 100.0).clamp(0.0, 1.0)
}

pub fn attack_goal() -> Goal<AgentFact> {
    let mut attack_goal_state = WorldState::new();
    attack_goal_state.require(AgentFact::TargetAttacked, Condition::Equals(true.into()));

    Goal {
        name: "Combattre",
        desired_state: attack_goal_state,
        priority: 1.0,
        score_fn: attack_score,
    }
}

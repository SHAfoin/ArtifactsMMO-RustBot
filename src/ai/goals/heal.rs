use crate::{
    ai::goap::{Condition, FactValue, Goal, WorldState},
    types::{
        ai::agent_facts::AgentFact,
        game::{character::Character, character_additionnal_info::CharacterAdditionnalInfo},
    },
};

fn heal_score(
    _: &WorldState<AgentFact>,
    _: &Character,
    additionnal_info: &CharacterAdditionnalInfo,
) -> f64 {
    let hp_ratio = additionnal_info.utility_ai_variables.health_ratio;
    let score = ((hp_ratio + 0.4).powi(5) * -1.0 * 0.2
        + 1.0
        + additionnal_info.utility_ai_variables.constante_heal)
        .clamp(0.0, 1.0);
    score
}

pub fn heal_goal() -> Goal<AgentFact> {
    let mut heal_goal_state = WorldState::new();
    heal_goal_state.require(AgentFact::Health, Condition::Equals(FactValue::Float(1.0)));

    Goal {
        name: "Soin",
        desired_state: heal_goal_state,
        priority: 1.0,
        score_fn: heal_score,
    }
}

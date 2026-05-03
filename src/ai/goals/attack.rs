use crate::{
    ai::goap::{Condition, FactValue, Goal, WorldState},
    types::{
        ai::agent_facts::AgentFact,
        game::{character::Character, character_additionnal_info::CharacterAdditionnalInfo},
    },
};

fn attack_score(
    state: &WorldState<AgentFact>,
    character: &Character,
    additionnal_info: &CharacterAdditionnalInfo,
) -> f64 {
    additionnal_info.utility_ai_variables.constante_attack
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

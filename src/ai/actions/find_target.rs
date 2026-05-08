use tokio::task;

use crate::{
    ai::goap::{Action, ActionStatus, WorldState},
    api::maps::get_all_maps,
    types::{
        ai::agent_facts::AgentFact,
        common::settings::{self, Settings},
        game::{character::Character, character_additionnal_info::CharacterAdditionnalInfo},
    },
};

pub struct FindTarget;

impl Action<AgentFact> for FindTarget {
    fn name(&self) -> &str {
        "FindTarget"
    }

    fn effects(&self) -> WorldState<AgentFact> {
        let mut ws = WorldState::new();
        ws.set(AgentFact::TargetReady, true);
        ws.set(AgentFact::NeedEquipment, true);
        ws
    }

    fn execute(
        &mut self,
        state: &mut WorldState<AgentFact>,
        settings: &Settings,
        character: &mut Character,
        additionnal_info: &mut CharacterAdditionnalInfo,
    ) -> ActionStatus {
        let cible: Option<String> =
            if let Some(cible_prioritaire) = &additionnal_info.priority_target {
                Some(cible_prioritaire.clone())
            } else {
                Some(find_best_target(settings, character))
            };

        if let Some(id) = cible {
            if let Some((x, y)) = find_target_location(settings, character, &id) {
                println!("  -> Cible trouvee : {}, position: ({}, {})", id, x, y);
                additionnal_info.target_id = id.to_string();
                additionnal_info.position_target_x = x;
                additionnal_info.position_target_y = y;
                state.set(AgentFact::TargetReady, true);
                return ActionStatus::Success;
            } else {
                println!("  -> Cible trouvee : {}, position inconnue", id);
                state.set(AgentFact::TargetReady, false);
                return ActionStatus::Failure;
            }
        } else {
            println!("  -> Aucune cible trouvée.");
            state.set(AgentFact::TargetReady, false);
            return ActionStatus::Failure;
        }
    }
}

/// TODO regarder cibles autour de mon niveau ET atteignables
pub fn find_best_target(_: &Settings, _: &Character) -> String {
    "chicken".to_string()
}

pub fn find_target_location(
    settings: &Settings,
    _: &Character,
    target: &str,
) -> Option<(i64, i64)> {
    let result = task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let validated_target = target.into();
            get_all_maps(settings, Some(&validated_target), None, None, None, None).await
        })
    });

    match result {
        Ok(value) => {
            let x = value["data"]["x"].as_i64()?;
            let y = value["data"]["y"].as_i64()?;
            Some((x, y))
        }
        Err(_) => None,
    }
}

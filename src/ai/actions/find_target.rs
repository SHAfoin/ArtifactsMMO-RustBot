use crate::{
    ai::goap::{Action, ActionStatus, WorldState},
    types::{
        ai::agent_facts::AgentFact,
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
        character: &mut Character,
        additionnal_info: &mut CharacterAdditionnalInfo,
    ) -> ActionStatus {
        //TODO Trouver la Target (brouillon : même target à chaque fois)
        // Si cible identique à avant :
        // state.set(BotFact::NeedEquipment, false);
        // Sinon :
        // state.set(BotFact::NeedEquipment, true);
        println!("  -> Cible trouvee.");
        ActionStatus::Success
    }
}

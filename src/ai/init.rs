use crate::ai::actions::*;
use crate::ai::goals::attack::attack_goal;
use crate::ai::goals::heal::heal_goal;
use crate::ai::goap::*;
use crate::types::ai::agent_facts::AgentFact;
use crate::types::game::character::{self, Character};
use crate::types::game::character_additionnal_info::CharacterAdditionnalInfo;

fn game_loop(agent: &mut Agent<AgentFact>, ticks: usize) {
    for tick in 0..ticks {
        println!("\n=== Tick {} ===", tick);

        // L'Utility AI évalue et choisit le goal
        update_utility_variables(
            &mut agent.state,
            &mut agent.character,
            &mut agent.additionnal_info,
        );

        let Some(goal) = UtilityEvaluator::evaluate(
            &agent.goals,
            &agent.state,
            &agent.character,
            &agent.additionnal_info,
        )
        .cloned() else {
            println!("[Loop] Aucun goal selectionne.");
            break; // Plus rien à faire
        };

        // Si goal différent d'avant, redéterminer les facts
        if agent.current_goal_name != Some(goal.name) {
            update_facts(
                &mut agent.state,
                &mut agent.character,
                &mut agent.additionnal_info,
            );
        }

        println!("[Loop] Goal selectionne: {}", goal.name);

        // l'Agent exécute un tick vers ce goal
        agent.tick(&goal);

        // debug : afficher les facts après le tick
        agent.state.print_state();
    }
}

fn update_facts(
    state: &mut WorldState<AgentFact>,
    character: &mut Character,
    _: &mut CharacterAdditionnalInfo,
) {
    //TODO Ici on met à jour les facts en fonction du caracter et de l'additionnal_info

    // AgentFact::Health : ratio de vie actuelle / vie max
    let hp = character.hp;
    let max_hp = character.max_hp;
    let hp_ratio = hp as f64 / max_hp as f64;
    state.set(AgentFact::Health, FactValue::Float(hp_ratio));

    //     TargetReady,
    // NeedEquipment,
    // TargetInRange,
    // TargetAttacked,
    // HasPotion,

    state.set(AgentFact::TargetReady, FactValue::Bool(false));
    state.set(AgentFact::NeedEquipment, FactValue::Bool(false));
    state.set(AgentFact::TargetInRange, FactValue::Bool(false));
    state.set(AgentFact::TargetAttacked, FactValue::Bool(false));

    // TODO savoir si on a une potion de soin ou pas
    state.set(AgentFact::HasPotion, FactValue::Bool(false));
}

fn update_utility_variables(
    _: &WorldState<AgentFact>,
    character: &Character,
    additionnal_info: &mut CharacterAdditionnalInfo,
) {
    // health_ratio : ratio de vie actuelle / vie max
    let hp = character.hp;
    let max_hp = character.max_hp;
    let hp_ratio = hp as f64 / max_hp as f64;
    additionnal_info.utility_ai_variables.health_ratio = hp_ratio;
}

pub fn new_ai<'a>(
    mut character: Character,
    mut additionnal_info: CharacterAdditionnalInfo,
) -> Agent<AgentFact> {
    // worldstate
    let mut initial_worldstate = WorldState::new();

    // init les facts de base
    update_facts(
        &mut initial_worldstate,
        &mut character,
        &mut additionnal_info,
    );

    // ajouter les actions possibles
    // faut rajouter une ligne dès qu'on en crée une nouvelle !!
    let actions: Vec<Box<dyn Action<AgentFact>>> = vec![
        Box::new(move_to_enemy::MoveToEnemy),
        Box::new(attack_enemy::AttackEnemy),
        Box::new(repos::Repos),
        Box::new(drink_healing_potion::DrinkHealingPotion),
        Box::new(find_target::FindTarget),
        Box::new(find_equipment::FindEquipment),
    ];

    // ajouter les goals possibles
    // idem ajouter ici dès que nouveau goal hein !!
    let goals = vec![attack_goal(), heal_goal()];

    let mut agent = Agent::new(
        initial_worldstate,
        actions,
        goals,
        character,
        additionnal_info,
    );

    // init les vraies valeurs de l'utility AI
    update_utility_variables(
        &mut agent.state,
        &mut agent.character,
        &mut agent.additionnal_info,
    );

    agent

    // game_loop(&mut agent, 20);
}

// il faut update les variables du utility AI à chaque itération au cas où un nouveau goal plus intéressant devient disponible
// problème : il faut donc que les fonctions de score des goals aient accès à additionnal_info pour voir les variables du utility AI
// et il faut changer les facts du worldstate seulement quand on change de goal, car certaines infos dedans servent entre actions d'un même plan

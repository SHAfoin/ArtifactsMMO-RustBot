use crate::ai::actions::*;
use crate::ai::goals::attack::attack_goal;
use crate::ai::goals::heal::heal_goal;
use crate::ai::goap::*;
use crate::types::ai::agent_facts::AgentFact;
use crate::types::game::character::Character;
use crate::types::game::character_additionnal_info::CharacterAdditionnalInfo;

fn game_loop(agent: &mut Agent<AgentFact>, ticks: usize) {
    for tick in 0..ticks {
        println!("\n=== Tick {} ===", tick);

        // L'Utility AI évalue et choisit le goal
        let Some(goal) = UtilityEvaluator::evaluate(&agent.goals, &agent.state).cloned() else {
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

        // 3. Ici dans un vrai jeu : mise à jour du moteur, input, rendu...
    }
}

fn update_facts(
    state: &mut WorldState<AgentFact>,
    character: &mut Character,
    additionnal_info: &mut CharacterAdditionnalInfo,
) {
    //TODO Ici on met à jour les facts en fonction du caracter et de l'additionnal_info
    // Par exemple :
    // state.set(BotFact::Health, 70i32);
    // state.set(BotFact::EnemyHealth, 50i32);
}

fn update_utility_variables(
    state: &WorldState<AgentFact>,
    character: &Character,
    additionnal_info: &CharacterAdditionnalInfo,
) {
    //TODO Ici on met à jour les variables du Utility AI
}

pub fn new_ai<'a>(
    mut character: Character,
    mut additionnal_info: CharacterAdditionnalInfo,
) -> Agent<AgentFact> {
    let mut initial_worldstate = WorldState::new();
    update_facts(
        &mut initial_worldstate,
        &mut character,
        &mut additionnal_info,
    );

    let actions: Vec<Box<dyn Action<AgentFact>>> = vec![
        Box::new(move_to_enemy::MoveToEnemy),
        Box::new(attack_enemy::AttackEnemy),
        Box::new(repos::Repos),
        Box::new(drink_healing_potion::DrinkHealingPotion),
        Box::new(find_target::FindTarget),
        Box::new(find_equipment::FindEquipment),
    ];

    let goals = vec![attack_goal(), heal_goal()];

    Agent::new(
        initial_worldstate,
        actions,
        goals,
        character,
        additionnal_info,
    )

    // game_loop(&mut agent, 20);
}

// il faut update les variables du utility AI à chaque itération au cas où un nouveau goal plus intéressant devient disponible
// problème : il faut donc que les fonctions de score des goals aient accès à additionnal_info pour voir les variables du utility AI
// et il faut changer les facts du worldstate seulement quand on change de goal, car certaines infos dedans servent entre actions d'un même plan

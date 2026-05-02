use crate::ai::actions::*;
use crate::ai::goals::attack::attack_goal;
use crate::ai::goals::heal::heal_goal;
use crate::ai::goap::*;
use crate::types::ai::agent_facts::AgentFact;

fn game_loop(agent: &mut Agent<AgentFact>, ticks: usize) {
    for tick in 0..ticks {
        println!("\n=== Tick {} ===", tick);

        // 1. L'Utility AI évalue et choisit le goal
        let Some(goal) = UtilityEvaluator::evaluate(&agent.goals, &agent.state).cloned() else {
            println!("[Loop] Aucun goal selectionne.");
            break; // Plus rien à faire
        };

        // Si goal différent d'avant, redéterminer les facts
        if agent.current_goal_name != Some(goal.name) {
            update_states(&mut agent.state);
        }

        println!("[Loop] Goal selectionne: {}", goal.name);

        // 2. L'Agent exécute un tick vers ce goal
        agent.tick(&goal);

        agent.state.print_state();

        // 3. Ici dans un vrai jeu : mise à jour du moteur, input, rendu...
    }
}

fn update_states(state: &mut WorldState<AgentFact>) {
    //TODO Ici on met à jour les facts en fonction de la situation du jeu
    // Par exemple :
    // state.set(BotFact::Health, 70i32);
    // state.set(BotFact::EnemyHealth, 50i32);
}

fn main() {
    let mut initial = WorldState::new();
    update_states(&mut initial);

    let actions: Vec<Box<dyn Action<AgentFact>>> = vec![
        Box::new(move_to_enemy::MoveToEnemy),
        Box::new(attack_enemy::AttackEnemy),
        Box::new(repos::Repos),
        Box::new(drink_healing_potion::DrinkHealingPotion),
        Box::new(find_target::FindTarget),
        Box::new(find_equipment::FindEquipment),
    ];

    let goals = vec![attack_goal(), heal_goal()];

    let mut agent = Agent::new(initial, actions, goals);
    game_loop(&mut agent, 20);
}

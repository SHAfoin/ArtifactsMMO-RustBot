// Améliorations :
// - NEVERMIND CE POINT EST JUSTE POUR LE LORE => le ASTAR peut se gourer et tourner en boucle théoriquement, à cause du manque de best_g & closedlist dû à l'impossibilité d'avoir une hashmap... WAIT A MINUTE et si on utilisait pas une hashmap mais un iter avec juste un find ou map ???
// - si on trouve pas de plan le logiciel fait rien
// - j'aime pas trop l'usage des static/'a vers la fin
// - des optimisations mémoires doivent être possible i guess mais complexe

use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

// ======= FACT VALUE =======

/// Représente une valeur d'un fait dans le monde
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum FactValue {
    Bool(bool),
    Int(i64),
    Float(f64),
}

impl From<bool> for FactValue {
    fn from(value: bool) -> Self {
        FactValue::Bool(value)
    }
}

impl From<i64> for FactValue {
    fn from(value: i64) -> Self {
        FactValue::Int(value)
    }
}

impl From<i32> for FactValue {
    fn from(value: i32) -> Self {
        FactValue::Int(value as i64)
    }
}

impl From<f64> for FactValue {
    fn from(value: f64) -> Self {
        FactValue::Float(value)
    }
}

// ======= CONDITIONS =======

/// Représente une condition à vérifier sur un fait
#[derive(Clone, Debug)]
pub enum Condition {
    Equals(FactValue),
    NotEquals(FactValue),
    GreaterThan(FactValue),
    LessThan(FactValue),
    GreaterOrEqual(FactValue),
    LessOrEqual(FactValue),
}

impl Condition {
    /// Comparaison d'une value avec une condition
    fn evaluate(&self, value: &FactValue) -> bool {
        match self {
            Condition::Equals(a) => value == a,
            Condition::NotEquals(a) => value != a,
            Condition::GreaterThan(a) => value > a,
            Condition::LessThan(a) => value < a,
            Condition::GreaterOrEqual(a) => value >= a,
            Condition::LessOrEqual(a) => value <= a,
        }
    }
}

// ======= WORLD STATE =======

/// l'état du monde à un instant t, avec les faits (ex: Health=50) et les conditions à vérifier pour atteindre le goal (ex: Health > 50)
#[derive(Clone, Debug)]
pub struct WorldState<K: Eq + Hash + Clone> {
    facts: HashMap<K, FactValue>,
    conditions: HashMap<K, Condition>,
}

impl<K: Eq + Hash + Clone> WorldState<K> {
    /// création
    pub fn new() -> WorldState<K> {
        WorldState {
            facts: HashMap::new(),
            conditions: HashMap::new(),
        }
    }

    /// mettre de nouveau faits
    pub fn set(&mut self, key: K, value: impl Into<FactValue>) -> &mut Self {
        self.facts.insert(key, value.into());
        self
    }

    /// ajouter de nouvelles conditions à vérifier
    pub fn require(&mut self, key: K, condition: Condition) -> &mut Self {
        self.conditions.insert(key, condition);
        self
    }

    /// récupérer la valeur d'un fait
    pub fn get(&self, key: &K) -> Option<&FactValue> {
        self.facts.get(key)
    }

    /// récupérer tous les faits
    pub fn get_all(&self) -> &HashMap<K, FactValue> {
        &self.facts
    }

    /// Vérifie que self satisfait other
    pub fn satisfies(&self, other: &WorldState<K>) -> bool {
        other.conditions.iter().all(|(key, cond)| {
            self.facts
                .get(key)
                .map_or(false, |value| cond.evaluate(value))
        })
    }

    /// Applique les effets d'une action sur l'état du monde cad modifier les fait en fonction des effets de l'action
    fn apply(&self, effects: &WorldState<K>) -> WorldState<K> {
        let mut new_worldstate = self.clone();
        effects.facts.iter().for_each(|(key, value)| {
            new_worldstate.facts.insert(key.clone(), value.clone());
        });
        new_worldstate
    }
}

/// ====== ACTIONS =======

/// on crée des status pour les cas où on exécute l'action tous les ticks et que elle tourne déjà
pub enum ActionStatus {
    Failure,
    Success,
    Running,
}

/// Les actions qui peuvent être réalisées
/// Ici un trait : comme ça ça peut être implémenté par n'importe quelle struct
pub trait Action<K: Eq + Clone + Hash> {
    /// un nom pour l'action, qui va servir d'identifiant aussi
    fn name(&self) -> &str;
    /// les préconditions nécessaires pour que l'action soit réalisable
    fn preconditions(&self) -> WorldState<K>;
    /// les effets de l'action sur le monde, cad comment elle modifie les faits du monde
    fn effects(&self) -> WorldState<K>;
    /// l'exécution de l'action, qui va modifier le state du bot en fonction des effets de l'action
    fn execute(&mut self, state: &mut WorldState<K>) -> ActionStatus; // &mut self car au cas où ça change l'action actuelle
    /// le coût de l'action, qui va être utilisé par le planner pour calculer le meilleur chemin vers le goal (astar)
    /// défaut 1
    fn cost(&self) -> f64 {
        1.0
    }
    /// condition de validité autre que par les faits, ex: cooldown, ressources, etc... (ex: même si les préconditions sont vérifiées, l'action peut être pas réalisable pour d'autres raisons)
    fn is_valid(&self, _state: &WorldState<K>) -> bool {
        true
    }
}

// ======= GOALS =======

/// objectif à atteindre par le bot, avec une priorité (pour l'utility AI) et un état du monde désiré (pour le planner)
pub struct Goal<K: Eq + Clone + Hash> {
    pub priority: f64,
    pub desired_state: WorldState<K>,
    pub name: &'static str,
    pub score_fn: fn(&WorldState<K>) -> f64,
}

impl<K: Eq + Clone + Hash> Clone for Goal<K> {
    fn clone(&self) -> Self {
        Goal {
            priority: self.priority,
            desired_state: self.desired_state.clone(),
            name: self.name,
            score_fn: self.score_fn,
        }
    }
}

#[derive(Clone)]
pub struct Node<K: Eq + Clone + Hash> {
    state: WorldState<K>,
    g: f64,
    h: f64,
    action_idx: Option<usize>,
    parent: Option<Box<Node<K>>>,
}

impl<K: Eq + Clone + Hash> Node<K> {
    pub fn f(&self) -> f64 {
        self.g + self.h
    }
}

impl<K: Eq + Clone + Hash> PartialEq for Node<K> {
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()
    }
}

impl<K: Eq + Clone + Hash> Eq for Node<K> {}

impl<K: Eq + Clone + Hash> PartialOrd for Node<K> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl<K: Eq + Clone + Hash> Ord for Node<K> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f().total_cmp(&self.f())
    }
}

pub struct Planner;

impl Planner {
    pub fn plan<K: Eq + Clone + Hash>(
        start: &WorldState<K>,
        goal: &Goal<K>,
        actions: &[Box<dyn Action<K>>],
    ) -> Option<Vec<usize>> {
        // créer close list (hashmap)
        let mut closed_list: Vec<Node<K>> = vec![];
        // créer best_g (hashmap)
        let mut best_g: Vec<(Node<K>, f64)> = vec![];
        // créer open list (binaryheap)
        let mut open_list = BinaryHeap::new();
        // ajouter le noeud initial
        open_list.push(Node {
            state: start.clone(),
            g: 0.0,
            h: Planner::heuristic(start, goal),
            action_idx: None,
            parent: None,
        });

        while let Some(node) = open_list.pop() {
            // pour chaque dans openlist
            // si goal atteint
            if node.state.satisfies(&goal.desired_state) {
                // retourner rebuild(node)
                return Some(Planner::rebuild(&node));
            }

            // on itére sur toutes les actions possible
            // long mais pas le choix vu que on ne sais pas ce qui est possible tant qu'on fait pas le test de précondition
            for (index, value) in actions.iter().enumerate() {
                // node dejà dans le chemin, on dégage
                if let Some(_node_in_closed_list) =
                    closed_list.iter().find(|n| n.action_idx == Some(index))
                {
                    continue;
                }
                // le node ne vérifie pas les préconditions de l'action cad c'est pas un node suivant possible, on dégage
                if !node.state.satisfies(&value.preconditions()) {
                    continue;
                }
                // node pas valide (selon ce qu'on a définit) selon le state actuel, on dégage
                if !value.is_valid(&node.state) {
                    continue;
                }

                // calculer cout théorique (g actuel + action.cost), si plus grand que dans best_g ça dégage
                let g = node.g + value.cost();

                // si il existe déjà le node dans best_g
                if let Some(already_tried_g) =
                    best_g.iter().position(|(n, _)| n.action_idx == Some(index))
                {
                    // si le g actuel est plus grand que celui de best_g, on dégage
                    if g >= best_g.get(already_tried_g).unwrap().1 {
                        continue;
                    } else {
                        // remplacer le g de best_g par le g actuel
                        best_g.remove(already_tried_g);
                        best_g.push((node.clone(), g));
                    }
                }

                let next_effects = value.effects();
                let next = node.state.apply(&next_effects);
                let h = Planner::heuristic(&next, goal);

                open_list.push(Node {
                    state: next,
                    g,
                    h,
                    action_idx: Some(index),
                    parent: Some(Box::new(node.clone())),
                })
            }

            // mettre node actuel dans closed list
            closed_list.push(node);
        }
        None
    }

    // Nombre de conditions qu'il reste à vérifier pour accéder au but
    pub fn heuristic<K: Eq + Clone + Hash>(state: &WorldState<K>, goal: &Goal<K>) -> f64 {
        // pour chaque condition du goal, on filtre de sorte à récupérer la condition dans les facts du state, si y a pas on renvoie true, si y a on vérifie si la condition est PAS vérifiée, et on renvoie le nombre
        goal.desired_state
            .conditions
            .iter()
            .filter(|(k, cond)| state.facts.get(k).map_or(true, |v| !cond.evaluate(v)))
            .count() as f64
    }

    pub fn rebuild<K: Eq + Clone + Hash>(node: &Node<K>) -> Vec<usize> {
        let mut actions = vec![];
        let mut actual = node;
        while let Some(index) = actual.action_idx {
            actions.push(index);
            actual = actual.parent.as_deref().unwrap()
        }
        actions.reverse();
        actions
    }
}

pub struct Agent<K: Eq + Hash + Clone> {
    pub state: WorldState<K>,
    pub actions: Vec<Box<dyn Action<K>>>,
    pub goals: Vec<Goal<K>>,
    plan: Vec<usize>,
    step: usize,
    pub current_goal_name: Option<&'static str>,
}

impl<K: Eq + Clone + Hash> Agent<K> {
    pub fn new(
        state: WorldState<K>,
        actions: Vec<Box<dyn Action<K>>>,
        goals: Vec<Goal<K>>,
    ) -> Agent<K> {
        Agent {
            state,
            actions,
            goals,
            plan: vec![],
            step: 0,
            current_goal_name: None,
        }
    }

    pub fn tick(&mut self, selected_goal: &Goal<K>) {
        // on reçoit le goal en paramètre qui via de l'utility AI

        // si goal change, dégager le plan actuel
        if self.current_goal_name != Some(selected_goal.name) {
            self.plan.clear();
            self.step = 0;
            self.current_goal_name = Some(selected_goal.name);
        }

        // si plan vide, on refait un autre
        if self.step >= self.plan.len() {
            match Planner::plan(&self.state, selected_goal, &self.actions) {
                Some(plan) => {
                    println!("[Agent] Nouveau plan pour '{}':", selected_goal.name);
                    for idx in &plan {
                        println!("  -> {}", self.actions[*idx].name());
                    }
                    if plan.is_empty() {
                        println!("[Agent] Goal deja satisfait, aucune action necessaire.");
                        return;
                    }
                    self.plan = plan;
                    self.step = 0;
                }
                None => {
                    println!("[Agent] Aucun plan trouve pour '{}'.", selected_goal.name);
                    return;
                }
            }
        }

        let idx = self.plan[self.step];
        println!("[Agent] Execute action: {}", self.actions[idx].name());
        match self.actions[idx].execute(&mut self.state) {
            ActionStatus::Success => {
                self.step += 1;
            }
            ActionStatus::Failure => {
                self.step = 0;
                self.plan.clear();
            } // TODO: Log ??
            ActionStatus::Running => {
                return;
            }
        }
    }
}

pub struct UtilityEvaluator;

impl UtilityEvaluator {
    pub fn evaluate<'a, K: Eq + Clone + Hash>(
        goals: &'a [Goal<K>],
        state: &WorldState<K>,
    ) -> Option<&'a Goal<K>> {
        goals.iter().max_by(|a, b| {
            let score_a = (a.score_fn)(state);
            let score_b = (b.score_fn)(state);
            score_a
                .partial_cmp(&score_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

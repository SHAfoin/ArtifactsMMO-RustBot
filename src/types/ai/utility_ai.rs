pub struct UtilityAiVariables {
    constanteAttack: f64,
    constanteHeal: f64,
}

impl UtilityAiVariables {
    pub fn new() -> Self {
        Self {
            constanteAttack: 0.5,
            constanteHeal: 0.5,
        }
    }
}

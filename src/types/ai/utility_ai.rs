pub struct UtilityAiVariables {
    pub constante_attack: f64,
    pub constante_heal: f64,
    pub health_ratio: f64,
}

impl UtilityAiVariables {
    pub fn new() -> Self {
        Self {
            constante_attack: 0.5,
            constante_heal: 0.5,
            health_ratio: 1.0,
        }
    }
}

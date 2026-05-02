#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentFact {
    Health,
    TargetReady,
    NeedEquipment,
    TargetInRange,
    TargetAttacked,
    HasPotion,
}

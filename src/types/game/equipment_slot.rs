use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    Head,
    Body,
    Legs,
    Feet,
    Ring1,
    Ring2,
    Amulet,
    Artifact1,
    Artifact2,
    Artifact3,
    Utility1,
    Utility2,
    Bag,
    Rune,
}

impl fmt::Display for EquipmentSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let slot_str = match self {
            EquipmentSlot::Head => "head",
            EquipmentSlot::Body => "body",
            EquipmentSlot::Legs => "legs",
            EquipmentSlot::Feet => "feet",
            EquipmentSlot::Ring1 => "ring1",
            EquipmentSlot::Ring2 => "ring2",
            EquipmentSlot::Amulet => "amulet",
            EquipmentSlot::Artifact1 => "artifact1",
            EquipmentSlot::Artifact2 => "artifact2",
            EquipmentSlot::Artifact3 => "artifact3",
            EquipmentSlot::Utility1 => "utility1",
            EquipmentSlot::Utility2 => "utility2",
            EquipmentSlot::Bag => "bag",
            EquipmentSlot::Rune => "rune",
        };
        write!(f, "{}", slot_str)
    }
}

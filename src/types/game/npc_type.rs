pub enum NPCType {
    Merchant,
    Trader,
}

impl ToString for NPCType {
    fn to_string(&self) -> String {
        match self {
            NPCType::Merchant => "merchant".to_string(),
            NPCType::Trader => "trader".to_string(),
        }
    }
}

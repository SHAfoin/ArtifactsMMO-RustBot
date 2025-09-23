use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SkinType {
    Men1,
    Men2,
    Men3,
    Women1,
    Women2,
    Women3,
    Corrupted1,
    Zombie1,
}

impl fmt::Display for SkinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let skin_str = match self {
            SkinType::Men1 => "men1",
            SkinType::Men2 => "men2",
            SkinType::Men3 => "men3",
            SkinType::Women1 => "women1",
            SkinType::Women2 => "women2",
            SkinType::Women3 => "women3",
            SkinType::Corrupted1 => "corrupted1",
            SkinType::Zombie1 => "zombie1",
        };
        write!(f, "{}", skin_str)
    }
}

impl Default for SkinType {
    fn default() -> Self {
        SkinType::Men1
    }
}

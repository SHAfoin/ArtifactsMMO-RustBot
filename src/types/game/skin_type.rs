use core::fmt;

pub enum SkinType {
    Male1,
    Male2,
    Male3,
    Women1,
    Women2,
    Women3,
    Corrupted1,
    Zombie1,
}

impl fmt::Display for SkinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let skin_str = match self {
            SkinType::Male1 => "men1",
            SkinType::Male2 => "men2",
            SkinType::Male3 => "men3",
            SkinType::Women1 => "women1",
            SkinType::Women2 => "women2",
            SkinType::Women3 => "women3",
            SkinType::Corrupted1 => "corrupted1",
            SkinType::Zombie1 => "zombie1",
        };
        write!(f, "{}", skin_str)
    }
}

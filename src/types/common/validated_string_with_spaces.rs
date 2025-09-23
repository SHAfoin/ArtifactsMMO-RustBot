use core::fmt;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct ValidatedStringWithSpaces(String);

impl ValidatedStringWithSpaces {
    pub fn new(value: &str) -> Result<Self, String> {
        let regex = Regex::new(r"^[a-zA-Z0-9_-]+(\s[a-zA-Z0-9_-]+)*\s?$").unwrap();
        if regex.is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(format!(
                "Invalid string: '{}'. Must match ^[a-zA-Z0-9_-]+(\\s[a-zA-Z0-9_-]+)*\\s?$",
                value
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidatedStringWithSpaces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ValidatedStringWithSpaces {
    fn from(item: &str) -> Self {
        ValidatedStringWithSpaces::new(&item).unwrap_or_default()
    }
}

impl Default for ValidatedStringWithSpaces {
    fn default() -> Self {
        Self("".to_string())
    }
}

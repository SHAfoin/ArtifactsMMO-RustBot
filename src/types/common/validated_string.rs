use core::fmt;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct ValidatedString(String);

impl ValidatedString {
    pub fn new(value: &str) -> Result<Self, String> {
        let regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        if regex.is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(format!(
                "Invalid string: '{}'. Must match ^[a-zA-Z0-9_-]+$",
                value
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidatedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ValidatedString {
    fn from(item: &str) -> Self {
        ValidatedString::new(&item).unwrap_or_default()
    }
}

impl Default for ValidatedString {
    fn default() -> Self {
        Self("".to_string())
    }
}

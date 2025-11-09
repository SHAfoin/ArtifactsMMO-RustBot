use core::fmt;

#[derive(Debug, Clone)]
pub struct PaginationParams {
    page: i64,
    size: i64,
}

impl PaginationParams {
    pub fn new(page: i64, size: i64) -> Result<Self, String> {
        if page < 1 {
            return Err("Page must be >= 1".to_string());
        }
        if size < 1 || size > 100 {
            return Err("Size must be between 1 and 100".to_string());
        }
        Ok(Self { page, size })
    }

    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        vec![
            ("page", self.page.to_string()),
            ("size", self.size.to_string()),
        ]
    }
}

impl fmt::Display for PaginationParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "page: {}, size: {}", self.page, self.size)
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self { page: 1, size: 50 }
    }
}

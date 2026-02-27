use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortingRule {
    pub extension: String,
    pub target_path: String,
    pub category_name: String,
}

impl SortingRule {
    pub fn new(extension: &str, target_path: &str, category_name: &str) -> Self {
        Self {
            extension: extension.to_string(),
            target_path: target_path.to_string(),
            category_name: category_name.to_string(),
        }
    }
}

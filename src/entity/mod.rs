use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    id: Option<String>,
    name: String,
    created_at: DateTime<Utc>,
}

impl Account {
    pub fn new(name: String) -> Self {
        Account {
            id: None,
            name,
            created_at: Utc::now(),
        }
    }

    pub fn create(&self) -> String {
        format!(
            "CREATE account SET name = '{}', created_at = time::now();",
            self.name
        )
    }
}

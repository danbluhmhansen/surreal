use std::collections::HashMap;

use chrono::{DateTime, Utc};
use nameof::name_of;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    id: Option<String>,
    created: DateTime<Utc>,
    name: String,
    #[serde(skip)]
    base: HashMap<String, u32>,
}

impl Character {
    pub fn create(&self) -> String {
        let mut res = format!(
            "CREATE {0} SET {1} = time::now(), {2} = '{3}'",
            name_of!(type Character).to_lowercase(),
            name_of!(created in Character),
            name_of!(name in Character),
            self.name,
        );

        if !self.base.is_empty() {
            res.push_str(", ");
            self.base.iter().for_each(|(k, v)| {
                res.push_str(&format!("{0} = {1} ", k, v));
            });
        }

        res
    }

    pub fn new(name: String) -> Self {
        Character {
            id: None,
            created: Utc::now(),
            name,
            base: HashMap::new(),
        }
    }
}

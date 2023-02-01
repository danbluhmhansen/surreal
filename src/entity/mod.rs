use derive::SurrealCreate;
use r#macro::SurrealCreate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, SurrealCreate)]
pub struct Character {
    name: String,
    strength: i32,
}

impl Character {
    pub fn new(name: String, strength: i32) -> Self {
        Character { name, strength }
    }
}

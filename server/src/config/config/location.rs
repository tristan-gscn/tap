use std::collections::HashMap;

use serde::Deserialize;

use super::direction::Direction;
use super::spawn::Spawn;

pub const DEFAULT_ROOM_KIND: &str = "dungeon";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Location {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub exits: HashMap<Direction, String>,
    #[serde(default)]
    pub spawns: Vec<Spawn>,
    #[serde(default)]
    pub items: Vec<String>,
}

impl Location {
    pub fn kind_or_default(&self) -> &str {
        self.kind.as_deref().unwrap_or(DEFAULT_ROOM_KIND)
    }

    #[allow(dead_code)]
    /// Removes an item by name from this location's item list.
    pub fn remove_item(&mut self, item_name: &str) -> bool {
        if let Some(index) = self.items.iter().position(|item| item == item_name) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }
}
use std::collections::HashMap;

use serde::Deserialize;

use super::item::Item;
use super::location::Location;
use super::npc::Npc;
use super::quest::Quest;

#[derive(Debug, Deserialize, Default)]
pub struct WorldData {
    #[serde(default)]
    pub locations: HashMap<String, Location>,
    #[serde(default)]
    pub items: HashMap<String, Item>,
    #[serde(default)]
    pub npcs: HashMap<String, Npc>,
    #[serde(default)]
    pub quests: HashMap<String, Quest>,
}

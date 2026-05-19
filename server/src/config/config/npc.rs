use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Npc {
    pub name: String,
    pub description: String,
    pub dialogue: Vec<String>,
    pub stats: NpcStats,
}

#[derive(Debug, Deserialize, Default)]
pub struct NpcStats {
    pub hp: i32,
    pub attack: i32,
}

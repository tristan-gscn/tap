use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Npc {
    pub name: String,
    pub stats: NpcStats,
}

#[derive(Debug, Deserialize, Default)]
pub struct NpcStats {
    #[serde(default)]
    pub hp: i32,
    #[serde(default)]
    pub attack: i32,
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Npc {
    pub name: String,
    pub description: String,
    pub dialogue: Vec<String>,
    pub stats: NpcStats,
    #[serde(default)]
    pub quests: Vec<String>,
    #[serde(default)]
    pub model: Option<String>,
    /// Whether the NPC can be attacked. Peaceful NPCs (dialogue/quest-givers)
    /// reject ATTACK with `405 NPC_NOT_HOSTILE`. Defaults to hostile for
    /// backward compatibility with existing dungeon data.
    #[serde(default = "default_hostile")]
    pub hostile: bool,
}

/// NPCs are hostile by default unless the world data marks them peaceful.
fn default_hostile() -> bool {
    true
}

#[derive(Debug, Deserialize, Default)]
pub struct NpcStats {
    pub hp: i32,
    pub attack: i32,
}

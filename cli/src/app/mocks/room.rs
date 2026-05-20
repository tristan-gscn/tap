use std::collections::BTreeMap;

pub struct RoomNpc {
    pub id: u64,
    pub kind: String,
    pub hp: i64,
    pub max_hp: i64,
}

impl RoomNpc {
    /// Returns a human-readable label for the NPC, including HP.
    pub fn label(&self) -> String {
        format!("{} #{}  {}/{} hp", self.kind, self.id, self.hp, self.max_hp)
    }
}

pub struct RoomMock {
    pub name: String,
    pub description: String,
    pub exits: Vec<String>,
    pub players: Vec<String>,
    pub npcs: Vec<RoomNpc>,
    pub items: Vec<String>,
}

impl RoomMock {
    /// Creates a RoomMock from server response data.
    pub fn from_server(
        name: String,
        description: String,
        exits: BTreeMap<String, String>,
        players: Vec<String>,
        npcs: Vec<RoomNpc>,
        items: Vec<String>,
    ) -> Self {
        let exits = exits
            .into_iter()
            .map(|(direction, destination)| format!("{} -> {}", direction, destination))
            .collect();

        Self {
            name,
            description,
            exits,
            players,
            npcs,
            items,
        }
    }
}

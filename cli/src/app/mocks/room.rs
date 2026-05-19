use std::collections::BTreeMap;

pub struct RoomMock {
    pub name: String,
    pub description: String,
    pub exits: Vec<String>,
    pub players: Vec<String>,
    pub npcs: Vec<String>,
    pub items: Vec<String>,
}

impl RoomMock {
    pub fn from_server(
        name: String,
        description: String,
        exits: BTreeMap<String, String>,
        players: Vec<String>,
        npcs: Vec<String>,
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
pub struct RoomMock {
    pub name: String,
    pub description: String,
    pub exits: Vec<String>,
    pub players: Vec<String>,
    pub npcs: Vec<String>,
    pub items: Vec<String>,
}

impl RoomMock {
    pub fn sample() -> Self {
        Self {
            name: "Ironleaf Gate".into(),
            description: "A mossy archway stands before a winding forest road. Torches flicker in the rain.".into(),
            exits: vec!["North".into(), "East".into(), "South".into()],
            players: vec!["Ari".into(), "Bram".into()],
            npcs: vec!["Gatekeeper Rho".into(), "Traveling Merchant".into()],
            items: vec!["Bronze Key".into(), "Lantern".into(), "Old Map".into()],
        }
    }
}
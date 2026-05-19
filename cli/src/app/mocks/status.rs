pub struct PlayerStatusMock {
    pub name: String,
    pub hp_current: u16,
    pub hp_max: u16,
    pub combat_status: String,
    pub inventory: Vec<String>,
    pub quests: Vec<String>,
}

impl PlayerStatusMock {
    pub fn sample(name: String) -> Self {
        Self {
            name,
            hp_current: 72,
            hp_max: 100,
            combat_status: "Calm".into(),
            inventory: vec!["Rusty Dagger".into(), "Traveler Cloak".into(), "Healing Herb".into()],
            quests: vec!["Find the Beacon Shard".into(), "Deliver Supplies to Elmwatch".into()],
        }
    }
}
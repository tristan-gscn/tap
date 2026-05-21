pub struct QuestEntry {
    pub id: String,
    pub name: String,
    pub status: String,
    pub progress: i64,
    pub required: i64,
}

impl QuestEntry {
    /// Returns a human-readable label for the quest, including progress if applicable.
    pub fn label(&self) -> String {
        if self.required > 0 {
            format!(
                "{}: {}  [{}]  {}/{}",
                self.id, self.name, self.status, self.progress, self.required
            )
        } else {
            format!("{}  [{}]", self.name, self.status)
        }
    }
}

pub struct PlayerStatusMock {
    pub name: String,
    pub hp_current: u16,
    pub hp_max: u16,
    pub xp: i64,
    pub combat_status: String,
    pub inventory: Vec<String>,
    pub equipped_right: Option<String>,
    pub equipped_left: Option<String>,
    pub quests: Vec<QuestEntry>,
}

impl PlayerStatusMock {
    /// Creates a sample PlayerStatusMock for a given player name.
    pub fn sample(name: String) -> Self {
        Self {
            name,
            hp_current: 100,
            hp_max: 100,
            xp: 0,
            combat_status: "Calm".into(),
            inventory: Vec::new(),
            equipped_right: None,
            equipped_left: None,
            quests: Vec::new(),
        }
    }
}

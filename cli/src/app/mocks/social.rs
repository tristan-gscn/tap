pub struct SocialMock {
    pub online_players: Vec<String>,
    pub group_leader: String,
    pub group_members: Vec<String>,
}

impl SocialMock {
    pub fn sample() -> Self {
        Self {
            online_players: Vec::new(),
            group_leader: String::new(),
            group_members: Vec::new(),
        }
    }
}

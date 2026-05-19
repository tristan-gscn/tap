pub struct SocialMock {
    pub online_players: Vec<String>,
    pub group_leader: String,
    pub group_members: Vec<String>,
}

impl SocialMock {
    pub fn sample() -> Self {
        Self {
            online_players: vec!["Ari".into(), "Bram".into(), "Lyra".into(), "Nyx".into(), "Sol".into()],
            group_leader: "Ari".into(),
            group_members: vec!["Bram".into(), "Lyra".into()],
        }
    }
}
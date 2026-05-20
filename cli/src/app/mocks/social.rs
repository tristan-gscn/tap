pub struct GroupInvite {
    pub group_id: u64,
    pub leader: String,
    pub from: String,
}

pub struct SocialMock {
    pub online_players: Vec<String>,
    pub group_id: Option<u64>,
    pub group_leader: String,
    pub group_members: Vec<String>,
    pub group_invites: Vec<GroupInvite>,
}

impl SocialMock {
    /// Creates an empty sample SocialMock.
    pub fn sample() -> Self {
        Self {
            online_players: Vec::new(),
            group_id: None,
            group_leader: String::new(),
            group_members: Vec::new(),
            group_invites: Vec::new(),
        }
    }
}

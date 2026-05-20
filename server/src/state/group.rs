use std::collections::HashSet;

pub type GroupId = u64;

pub struct Group {
    pub id: GroupId,
    pub leader: String,
    pub members: Vec<String>,
    pub invited: HashSet<String>,
}

impl Group {
    /// Creates a new group with a leader as the first member.
    pub fn new(id: GroupId, leader: String) -> Self {
        Group {
            id,
            members: vec![leader.clone()],
            leader,
            invited: HashSet::new(),
        }
    }

    /// Removes a member from the group by name.
    pub fn remove_member(&mut self, name: &str) {
        self.members.retain(|m| m != name);
    }
}

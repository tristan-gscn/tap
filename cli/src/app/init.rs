use super::mocks::{PlayerStatusMock, RoomMock, SocialMock};
use super::{App, Tab};

impl App {
    /// Creates a new application instance.
    pub fn new(player_name: String, room: RoomMock, logs: Vec<String>) -> Self {
        Self {
            current_tab: Tab::Adventure,
            input: String::new(),
            logs,
            room,
            status: PlayerStatusMock::sample(player_name),
            social: SocialMock::sample(),
        }
    }
}
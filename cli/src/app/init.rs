use super::mocks::{PlayerStatusMock, RoomMock, SocialMock};
use super::{App, Tab};

impl App {
    pub fn new() -> Self {
        let logs = vec![
            String::from("[System] > Local mock client started"),
            String::from("[Event] > Rain begins to fall outside the gate"),
            String::from("[Chat] > Ari: Ready to move north?"),
        ];
        Self {
            current_tab: Tab::Adventure,
            input: String::new(),
            logs,
            room: RoomMock::sample(),
            status: PlayerStatusMock::sample(),
            social: SocialMock::sample(),
        }
    }
}
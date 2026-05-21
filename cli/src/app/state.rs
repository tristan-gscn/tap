use super::mocks::{PlayerStatusMock, RoomMock, SocialMock};
use super::Tab;

pub struct App {
    pub current_tab: Tab,
    pub input: String,
    pub logs: Vec<String>,
    pub room: RoomMock,
    pub status: PlayerStatusMock,
    pub social: SocialMock,
}

use std::io;

use super::super::protocol::{ApiResponse, LookResponse, InventoryResponse};
use super::super::transport::{read_response, send_line};
use super::Session;

impl Session {
    pub fn send_command(&mut self, line: &str) -> io::Result<ApiResponse> {
        send_line(&mut self.stream, line)?;
        let resp = read_response(&mut self.stream)?;
        match &resp {
            ApiResponse::Ok { kind, data } => {
                self.app.logs.push(format!("[Server] OK {} {}", kind, data));
            }
            ApiResponse::Error { code, message } => {
                self.app.logs.push(format!("[Server] ERR {} {}", code, message));
            }
        }
        Ok(resp)
    }

    pub fn apply_look_to_room(&mut self, look: LookResponse) {
        self.app.room = crate::app::RoomMock::from_server(
            look.room.name,
            look.room.description,
            look.room.exits,
            look.players,
            look.npcs,
            look.items,
        );
    }

    pub fn apply_inventory_to_status(&mut self, inventory: InventoryResponse) {
        self.app.status.inventory = inventory.items;
    }

    // refresh methods are implemented in refresh.rs
}

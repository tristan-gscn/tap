use super::super::protocol::{ApiResponse, LookResponse, InventoryResponse};
use super::Session;

impl Session {
    pub fn log_response(&mut self, resp: &ApiResponse) {
        match resp {
            ApiResponse::Ok { kind, data } => {
                self.app.logs.push(format!("[Server] OK {} {}", kind, data));
            }
            ApiResponse::Error { code, message } => {
                self.app.logs.push(format!("[Server] ERR {} {}", code, message));
            }
        }
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

    // refresh methods removed; updates are event-driven
}

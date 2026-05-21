use std::io;

use super::super::protocol::{
    parse_inventory, parse_look, ApiResponse, InventoryResponse, LookResponse,
};
use super::Session;
use crate::app::{RoomMock, RoomNpc};

/// Converts a LookResponse from the server into a RoomMock for the application.
pub fn look_to_room(look: LookResponse) -> RoomMock {
    let npcs = look
        .npcs
        .into_iter()
        .map(|n| RoomNpc {
            id: n.id,
            kind: n.kind,
            name: n.name,
            hp: n.hp,
            max_hp: n.max_hp,
        })
        .collect();
    RoomMock::from_server(
        look.room.name,
        look.room.description,
        look.room.exits,
        look.players,
        npcs,
        look.items,
    )
}

impl Session {
    /// Logs a server response to the application log.
    pub fn log_response(&mut self, resp: &ApiResponse) {
        match resp {
            ApiResponse::Ok { kind, data } => {
                if kind == "chat" {
                    return;
                }
                let details = fmt_data(data);
                let msg = if details.is_empty() {
                    format!("ok {}", kind)
                } else {
                    format!("ok {} {}", kind, details)
                };
                self.app.logs.push(format!("<server> {}", msg));
            }
            ApiResponse::Error { code, message } => {
                self.app
                    .logs
                    .push(format!("<server> ERR {} {}", code, message));
            }
        }
    }

    /// Applies a LOOK response to the current room state.
    pub fn apply_look_to_room(&mut self, look: LookResponse) {
        self.app.room = look_to_room(look);
    }

    /// Applies an INVENTORY response to the player's status.
    pub fn apply_inventory_to_status(&mut self, inventory: InventoryResponse) {
        self.app.status.inventory = inventory.items;
        self.app.status.equipped_right = inventory.equipped.right;
        self.app.status.equipped_left = inventory.equipped.left;
    }

    /// Sends a LOOK command to refresh the current room state.
    pub fn refresh_look(&mut self) -> io::Result<()> {
        let resp = self.send_command("LOOK")?;
        match parse_look(resp) {
            Ok(look) => self.apply_look_to_room(look),
            Err(err) => self
                .app
                .logs
                .push(format!("[Client] LOOK refresh failed: {}", err)),
        }
        Ok(())
    }

    /// Sends an INVENTORY command to refresh the player's carried items.
    pub fn refresh_inventory(&mut self) -> io::Result<()> {
        let resp = self.send_command("INVENTORY")?;
        match parse_inventory(resp) {
            Ok(inv) => self.apply_inventory_to_status(inv),
            Err(err) => self
                .app
                .logs
                .push(format!("[Client] INVENTORY refresh failed: {}", err)),
        }
        Ok(())
    }
}

/// Formats JSON data into a human-readable string for logging.
fn fmt_data(data: &serde_json::Value) -> String {
    match data {
        serde_json::Value::Object(map) => map
            .iter()
            .map(|(k, v)| format!("{}={}", k, fmt_data(v)))
            .collect::<Vec<_>>()
            .join(", "),
        serde_json::Value::Array(items) => {
            items.iter().map(fmt_data).collect::<Vec<_>>().join(", ")
        }
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
    }
}

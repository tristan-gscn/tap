use super::super::protocol::{ApiResponse, LookResponse, InventoryResponse};
use super::Session;

impl Session {
    pub fn log_response(&mut self, resp: &ApiResponse) {
        match resp {
            ApiResponse::Ok { kind, data } => {
                if kind == "chat" { return; }
                let details = fmt_data(data);
                let msg = if details.is_empty() { format!("ok {}", kind) } else { format!("ok {} {}", kind, details) };
                self.app.logs.push(format!("<server> {}", msg));
            }
            ApiResponse::Error { code, message } => {
                self.app.logs.push(format!("<server> ERR {} {}", code, message));
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

fn fmt_data(data: &serde_json::Value) -> String {
    match data {
        serde_json::Value::Object(map) => map.iter()
            .map(|(k, v)| format!("{}={}", k, fmt_data(v)))
            .collect::<Vec<_>>().join(", "),
        serde_json::Value::Array(items) => items.iter().map(fmt_data).collect::<Vec<_>>().join(", "),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
    }
}

use super::super::Session;

pub fn handle_room_event(session: &mut Session, ev: &str, name: &str, item: &str) -> bool {
    match ev {
        "presence_enter" if !name.is_empty() => {
            if !session.app.room.players.iter().any(|p| p == name) { session.app.room.players.push(name.to_string()); }
            session.app.logs.push(format!("! {} entered", name));
            true
        }
        "presence_leave" if !name.is_empty() => {
            session.app.room.players.retain(|p| p != name);
            session.app.logs.push(format!("! {} left", name));
            true
        }
        "item_taken" if !item.is_empty() => {
            session.app.room.items.retain(|i| i != item);
            session.app.logs.push(format!("! item taken: {}", item));
            true
        }
        "item_dropped" if !item.is_empty() => {
            if !session.app.room.items.iter().any(|i| i == item) { session.app.room.items.push(item.to_string()); }
            session.app.logs.push(format!("! item dropped: {}", item));
            true
        }
        _ => false,
    }
}

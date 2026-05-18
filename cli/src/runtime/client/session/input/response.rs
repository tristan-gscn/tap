use super::super::super::protocol::ApiResponse;
use super::super::Session;

pub fn process_response(session: &mut Session, first_word: &str, response: ApiResponse) {
    let mut look_synced = false;
    let mut inventory_synced = false;

    if first_word == "LOOK" {
        match super::super::super::protocol::parse_look(response) {
            Ok(look) => {
                session.apply_look_to_room(look);
                look_synced = true;
            }
            Err(err) => session
                .app
                .logs
                .push(format!("[Client] Error updating room after LOOK: {}", err)),
        }
    } else if first_word == "INVENTORY" {
        match super::super::super::protocol::parse_inventory(response) {
            Ok(inventory) => {
                session.apply_inventory_to_status(inventory);
                inventory_synced = true;
            }
            Err(err) => session
                .app
                .logs
                .push(format!("[Client] Error updating inventory after INVENTORY: {}", err)),
        }
    }

    if !look_synced {
        if let Err(err) = session.refresh_room_with_look() {
            session.app.logs.push(format!(
                "[Client] Error refreshing room after command: {}",
                err
            ));
        }
    }

    if !inventory_synced {
        if let Err(err) = session.refresh_inventory() {
            session.app.logs.push(format!(
                "[Client] Error refreshing inventory after command: {}",
                err
            ));
        }
    }
}

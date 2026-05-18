use super::super::super::protocol::ApiResponse;
use super::super::Session;

pub fn process_response(session: &mut Session, first_word: &str, response: ApiResponse) {
    if first_word == "LOOK" {
        match super::super::super::protocol::parse_look(response) {
            Ok(look) => {
                session.apply_look_to_room(look);
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
            }
            Err(err) => session
                .app
                .logs
                .push(format!("[Client] Error updating inventory after INVENTORY: {}", err)),
        }
    }
}

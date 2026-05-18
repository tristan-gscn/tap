use std::io;

use super::super::transport::{read_response, send_line};
use super::Session;

impl Session {
    pub fn refresh_room_with_look(&mut self) -> io::Result<()> {
        send_line(&mut self.stream, "LOOK")?;
        let look_response = read_response(&mut self.stream)?;
        let look = super::super::protocol::parse_look(look_response)?;
        self.app.room = crate::app::RoomMock::from_server(
            look.room.name,
            look.room.description,
            look.room.exits,
            look.players,
            look.npcs,
            look.items,
        );
        Ok(())
    }

    pub fn refresh_inventory(&mut self) -> io::Result<()> {
        send_line(&mut self.stream, "INVENTORY")?;
        let inventory_response = read_response(&mut self.stream)?;
        let inventory = super::super::protocol::parse_inventory(inventory_response)?;
        self.app.status.inventory = inventory.items;
        Ok(())
    }
}

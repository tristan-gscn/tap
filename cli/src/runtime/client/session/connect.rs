use std::io;
use std::net::TcpStream;

use crate::app::{App, RoomMock};
use crate::ui;

use super::super::protocol::{parse_inventory, parse_look};
use super::super::transport::{read_response, send_line, SERVER_ADDR};
use super::Session;

pub fn connect() -> io::Result<Session> {
    let player_name = ui::prompt_player_name()?;
    let mut stream = TcpStream::connect(SERVER_ADDR)?;

    send_line(&mut stream, &format!("CONNECT {}", player_name))?;
    let connect_response = read_response(&mut stream)?;
    super::super::protocol::ensure_ok(&connect_response, "connect")?;

    // initial LOOK
    send_line(&mut stream, "LOOK")?;
    let look_response = read_response(&mut stream)?;
    let look = parse_look(look_response)?;

    // initial INVENTORY
    send_line(&mut stream, "INVENTORY")?;
    let inventory_response = read_response(&mut stream)?;
    let inventory = parse_inventory(inventory_response)?;

    let logs = vec![
        format!("[System] > Connected to {} as {}", SERVER_ADDR, player_name),
        format!("[System] > Room loaded: {} ({})", look.room.name, look.room.id),
    ];

    let mut app = App::new(player_name, RoomMock::from_server(
        look.room.name,
        look.room.description,
        look.room.exits,
        look.players,
        look.npcs,
        look.items,
    ), logs);

    app.status.inventory = inventory.items;

    Ok(Session { app, stream })
}

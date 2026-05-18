use std::io; use std::net::TcpStream;
use crate::app::{App, RoomMock}; use crate::ui;
use super::super::protocol::{parse_inventory, parse_look, parse_line};
use super::super::transport::{send_line, SERVER_ADDR};
use super::reader::read_line_blocking; use super::Session;

pub fn connect() -> io::Result<Session> {
    let player_name = ui::prompt_player_name()?;
    let mut stream = TcpStream::connect(SERVER_ADDR)?;
    let mut reader = stream.try_clone()?; reader.set_nonblocking(true)?;
    let mut read_buf = Vec::new();

    send_line(&mut stream, &format!("CONNECT {}", player_name))?;
    let connect_response = parse_line(&read_line_blocking(&mut reader, &mut read_buf)?)?;
    super::super::protocol::ensure_ok(&connect_response, "connect")?;

    send_line(&mut stream, "LOOK")?;
    let look = parse_look(parse_line(&read_line_blocking(&mut reader, &mut read_buf)?)?)?;

    send_line(&mut stream, "INVENTORY")?;
    let inventory = parse_inventory(parse_line(&read_line_blocking(&mut reader, &mut read_buf)?)?)?;

    let logs = vec![
        format!("[System] > Connected to {} as {}", SERVER_ADDR, player_name),
        format!("[System] > Room loaded: {} ({})", look.room.name, look.room.id),
    ];

    let mut app = App::new(player_name, RoomMock::from_server(
        look.room.name, look.room.description, look.room.exits,
        look.players, look.npcs, look.items,
    ), logs);
    app.status.inventory = inventory.items;

    Ok(Session { app, stream, reader, read_buf })
}

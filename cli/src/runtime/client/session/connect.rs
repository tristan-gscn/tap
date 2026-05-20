use std::io;
use std::net::TcpStream;

use crate::app::App;
use crate::ui;
use super::super::protocol::{parse_inventory, parse_line, parse_look, ApiResponse};
use super::super::transport::{send_line, SERVER_ADDR};
use super::reader::read_line_blocking;
use super::send::look_to_room;
use super::Session;

/// Retrieves the next response from the server, ignoring asynchronous events.
fn next_response(reader: &mut TcpStream, buf: &mut Vec<u8>) -> io::Result<ApiResponse> {
    loop {
        let line = read_line_blocking(reader, buf)?;
        let resp = parse_line(&line)?;
        match &resp {
            ApiResponse::Ok { kind, .. } if kind == "event" => continue,
            _ => return Ok(resp),
        }
    }
}

/// Establishes the initial connection with the TAP server and initializes the session.
/// Prompts for the player name, sends the CONNECT command, and retrieves initial state.
pub fn connect() -> io::Result<Session> {
    let info = ui::prompt_player_info()?;
    let mut stream = TcpStream::connect(SERVER_ADDR)?;
    let mut reader = stream.try_clone()?;
    reader.set_nonblocking(true)?;
    let mut read_buf = Vec::new();

    send_line(&mut stream, &format!("CONNECT {} {}", info.name, info.class))?;
    let connect_response = next_response(&mut reader, &mut read_buf)?;
    super::super::protocol::ensure_ok(&connect_response, "connect")?;

    send_line(&mut stream, "LOOK")?;
    let look = parse_look(next_response(&mut reader, &mut read_buf)?)?;

    send_line(&mut stream, "INVENTORY")?;
    let inventory = parse_inventory(next_response(&mut reader, &mut read_buf)?)?;

    send_line(&mut stream, "WHO")?;
    let who = next_response(&mut reader, &mut read_buf)?;

    send_line(&mut stream, "QUESTS")?;
    let quest_summary = next_response(&mut reader, &mut read_buf)?;

    let logs = vec![
        format!("[System] Connected to {} as {} ({})", SERVER_ADDR, info.name, info.class),
        format!("[System] Room loaded: {} ({})", look.room.name, look.room.id),
    ];

    let mut app = App::new(info.name, look_to_room(look), logs);
    app.status.inventory = inventory.items;
    if let ApiResponse::Ok { data, .. } = who {
        if let Some(list) = data.get("players").and_then(|v| v.as_array()) {
            app.social.online_players = list
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
        }
    }
    if let ApiResponse::Ok { data, .. } = quest_summary {
        app.apply_quest_summary(&data);
    }

    Ok(Session {
        app,
        stream,
        reader,
        read_buf,
    })
}

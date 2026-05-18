use std::collections::BTreeMap;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use serde::Deserialize;

use crate::app::{App, RoomMock};
use crate::ui;

const SERVER_ADDR: &str = "127.0.0.1:4000";

pub struct Session {
    pub app: App,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
enum ApiResponse {
    #[serde(rename = "ok")]
    Ok {
        #[serde(rename = "type")]
        kind: String,
        data: serde_json::Value,
    },
    #[serde(rename = "error")]
    Error { code: u16, message: String },
}

#[derive(Debug, Deserialize)]
struct LookResponse {
    room: LookRoom,
    players: Vec<String>,
    items: Vec<String>,
    npcs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LookRoom {
    id: String,
    name: String,
    description: String,
    exits: BTreeMap<String, String>,
}

pub fn connect() -> io::Result<Session> {
    let player_name = ui::prompt_player_name()?;
    let mut stream = TcpStream::connect(SERVER_ADDR)?;

    send_line(&mut stream, &format!("CONNECT {}", player_name))?;
    let connect_response = read_response(&mut stream)?;
    ensure_ok(&connect_response, "connect")?;

    send_line(&mut stream, "LOOK")?;
    let look_response = read_response(&mut stream)?;
    let look = parse_look(look_response)?;

    let logs = vec![
        format!("[System] > Connected to {} as {}", SERVER_ADDR, player_name),
        format!("[System] > Room loaded: {} ({})", look.room.name, look.room.id),
    ];

    let room = RoomMock::from_server(
        look.room.name,
        look.room.description,
        look.room.exits,
        look.players,
        look.npcs,
        look.items,
    );

    Ok(Session {
        app: App::new(player_name, room, logs),
    })
}

fn send_line(stream: &mut TcpStream, line: &str) -> io::Result<()> {
    stream.write_all(line.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()
}

fn read_response(stream: &mut TcpStream) -> io::Result<ApiResponse> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    if line.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "empty server response"));
    }

    serde_json::from_str(line.trim()).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

fn ensure_ok(response: &ApiResponse, expected_kind: &str) -> io::Result<()> {
    match response {
        ApiResponse::Ok { kind, .. } if kind == expected_kind => Ok(()),
        ApiResponse::Ok { kind, .. } => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected response type: {}", kind),
        )),
        ApiResponse::Error { code, message } => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("server error {}: {}", code, message),
        )),
    }
}

fn parse_look(response: ApiResponse) -> io::Result<LookResponse> {
    match response {
        ApiResponse::Ok { kind, data } if kind == "look" => {
            serde_json::from_value(data).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
        }
        ApiResponse::Ok { kind, .. } => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected response type: {}", kind),
        )),
        ApiResponse::Error { code, message } => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("server error {}: {}", code, message),
        )),
    }
}
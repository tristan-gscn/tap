use super::types::{ApiResponse, InventoryResponse, LookResponse};
use std::io;

pub fn ensure_ok(response: &ApiResponse, expected_kind: &str) -> io::Result<()> {
    match response {
        ApiResponse::Ok { kind, .. } if kind == expected_kind => Ok(()),
        ApiResponse::Ok { kind, .. } => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected response type: {}", kind),
        )),
        ApiResponse::Error { code, message } => Err(io::Error::other(format!(
            "server error {}: {}",
            code, message
        ))),
    }
}

pub fn parse_look(response: ApiResponse) -> io::Result<LookResponse> {
    match response {
        ApiResponse::Ok { kind, data } if kind == "look" => serde_json::from_value(data)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err)),
        ApiResponse::Ok { kind, .. } => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected response type: {}", kind),
        )),
        ApiResponse::Error { code, message } => Err(io::Error::other(format!(
            "server error {}: {}",
            code, message
        ))),
    }
}

pub fn parse_inventory(response: ApiResponse) -> io::Result<InventoryResponse> {
    match response {
        ApiResponse::Ok { kind, data } if kind == "inventory" => serde_json::from_value(data)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err)),
        ApiResponse::Ok { kind, .. } => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected response type: {}", kind),
        )),
        ApiResponse::Error { code, message } => Err(io::Error::other(format!(
            "server error {}: {}",
            code, message
        ))),
    }
}

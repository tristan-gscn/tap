use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum ApiResponse {
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
pub struct LookResponse {
    pub room: LookRoom,
    pub players: Vec<String>,
    pub items: Vec<String>,
    pub npcs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LookRoom {
    pub id: String,
    pub name: String,
    pub description: String,
    pub exits: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryResponse {
    pub items: Vec<String>,
}

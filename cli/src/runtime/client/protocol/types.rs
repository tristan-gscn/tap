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
    pub npcs: Vec<LookNpc>,
}

#[derive(Debug, Deserialize)]
pub struct LookRoom {
    pub id: String,
    pub name: String,
    pub description: String,
    pub exits: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct LookNpc {
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub name: Option<String>,
    pub hp: i64,
    pub max_hp: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct EquippedSlots {
    #[serde(default)]
    pub right: Option<String>,
    #[serde(default)]
    pub left: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryResponse {
    pub items: Vec<String>,
    #[serde(default)]
    pub equipped: EquippedSlots,
}

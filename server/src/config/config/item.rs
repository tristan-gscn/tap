use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
    pub obtainable: bool,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Returns the string representation of the direction.
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "west",
        }
    }

    /// Parses a string input into a `Direction`.
    /// Supports full names (e.g., "north") and abbreviations (e.g., "n").
    pub fn from_input(s: &str) -> Option<Direction> {
        match s.trim().to_lowercase().as_str() {
            "north" | "n" => Some(Direction::North),
            "south" | "s" => Some(Direction::South),
            "east" | "e" => Some(Direction::East),
            "west" | "w" => Some(Direction::West),
            _ => None,
        }
    }
}

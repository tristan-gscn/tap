use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub objective: Objective,
    #[serde(default)]
    pub reward: Reward,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Objective {
    Collect { target: String, count: u32 },
    Kill { target: String, count: u32 },
}

impl Objective {
    pub fn target(&self) -> &str {
        match self {
            Objective::Collect { target, .. } | Objective::Kill { target, .. } => target,
        }
    }

    pub fn count(&self) -> u32 {
        match self {
            Objective::Collect { count, .. } | Objective::Kill { count, .. } => *count,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Objective::Collect { .. } => "collect",
            Objective::Kill { .. } => "kill",
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Reward {
    #[serde(default)]
    pub xp: i64,
}

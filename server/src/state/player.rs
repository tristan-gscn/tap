use std::collections::HashMap;

use tokio::sync::mpsc::UnboundedSender;

use crate::protocol::response::Response;
use crate::state::group::GroupId;

pub const DEFAULT_HP: i32 = 100;
pub const DEFAULT_ATTACK: i32 = 10;

#[derive(Clone, Default)]
pub struct QuestProgress {
    pub progress: u32,
    pub completed: bool,
}

pub struct Player {
    pub name: String,
    pub addr: String,
    pub class: Option<String>,
    pub room: String,
    pub tx: UnboundedSender<Response>,
    pub inventory: Vec<String>,
    pub group: Option<GroupId>,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub xp: i64,
    pub quests: HashMap<String, QuestProgress>,
}

impl Player {
    /// Creates a new player with default stats in the starting room.
    pub fn new(
        name: String,
        addr: String,
        class: Option<String>,
        tx: UnboundedSender<Response>,
    ) -> Self {
        Player {
            name,
            addr,
            class,
            room: "start".to_string(),
            tx,
            inventory: Vec::new(),
            group: None,
            hp: DEFAULT_HP,
            max_hp: DEFAULT_HP,
            attack: DEFAULT_ATTACK,
            xp: 0,
            quests: HashMap::new(),
        }
    }

    /// Removes an item from the player's inventory by ID.
    pub fn take_from_inventory(&mut self, item_id: &str) -> bool {
        if let Some(idx) = self.inventory.iter().position(|i| i == item_id) {
            self.inventory.remove(idx);
            true
        } else {
            false
        }
    }

    /// Restores HP and moves the player back to the starting room.
    pub fn respawn(&mut self) {
        self.hp = self.max_hp;
        self.room = "start".to_string();
    }

    /// Updates quest progress for matching objectives and returns touched quest IDs.
    pub fn advance_quests(&mut self, kind: &str, target: &str, amount: u32) -> Vec<String> {
        let cfg = crate::config::get();
        let mut touched = Vec::new();

        for (id, prog) in self.quests.iter_mut() {
            if prog.completed {
                continue;
            }
            let Some(quest) = cfg.world.quests.get(id) else {
                continue;
            };
            if quest.objective.kind() == kind && quest.objective.target() == target {
                prog.progress = (prog.progress + amount).min(quest.objective.count());
                touched.push(id.clone());
            }
        }
        touched
    }
}

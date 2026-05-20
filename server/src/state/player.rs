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
    pub equipped_right: Option<String>,
    pub equipped_left: Option<String>,
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
            equipped_right: None,
            equipped_left: None,
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

<<<<<<< HEAD
    pub fn equip(&mut self, slot: EquipSlot, item_id: String) {
        match slot {
            EquipSlot::Right => self.equipped_right = Some(item_id),
            EquipSlot::Left => self.equipped_left = Some(item_id),
        }
    }

    pub fn unequip(&mut self, slot: EquipSlot) {
        match slot {
            EquipSlot::Right => self.equipped_right = None,
            EquipSlot::Left => self.equipped_left = None,
        }
    }

    pub fn clear_if_equipped(&mut self, item_id: &str) {
        if self.equipped_right.as_deref() == Some(item_id) {
            self.equipped_right = None;
        }
        if self.equipped_left.as_deref() == Some(item_id) {
            self.equipped_left = None;
        }
    }

    pub fn effective_attack(&self) -> i32 {
        self.attack + self.weapon_bonus()
    }

    pub fn shield_reduction(&self) -> i32 {
        if self.has_shield() { 2 } else { 0 }
    }

    fn weapon_bonus(&self) -> i32 {
        let mut bonus = 0;
        if self.has_weapon(self.equipped_right.as_deref()) {
            bonus += 4;
        }
        if self.has_weapon(self.equipped_left.as_deref()) {
            bonus += 4;
        }
        bonus
    }

    fn has_weapon(&self, item_id: Option<&str>) -> bool {
        matches!(item_id, Some(id) if is_weapon(id))
    }

    fn has_shield(&self) -> bool {
        matches!(self.equipped_left.as_deref(), Some(id) if is_shield(id))
            || matches!(self.equipped_right.as_deref(), Some(id) if is_shield(id))
    }

=======
    /// Restores HP and moves the player back to the starting room.
>>>>>>> 7bfee5091c30d156334bc1ea9045cfa6f4629888
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

#[derive(Clone, Copy, Debug)]
pub enum EquipSlot {
    Right,
    Left,
}

fn is_weapon(id: &str) -> bool {
    let id = id.to_lowercase();
    id.contains("sword")
        || id.contains("axe")
        || id.contains("dagger")
        || id.contains("bow")
        || id.contains("crossbow")
        || id.contains("staff")
        || id.contains("wand")
}

fn is_shield(id: &str) -> bool {
    id.to_lowercase().contains("shield")
}

use std::collections::HashMap;

use crate::config;

#[derive(Clone)]
pub struct NpcInstance {
    pub id: u64,
    pub npc_type: String,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
}

pub enum AttackOutcome {
    NoTarget,
    Hit {
        npc_id: u64,
        npc_type: String,
        npc_hp: i32,
        npc_attack: i32,
    },
    Killed { npc_id: u64, npc_type: String },
}

pub struct WorldState {
    room_items: HashMap<String, Vec<String>>,
    room_npcs: HashMap<String, Vec<NpcInstance>>,
}

impl WorldState {
    /// Builds a world state from the loaded configuration.
    pub fn from_config() -> Self {
        let cfg = config::get();
        let mut room_items = HashMap::new();
        let mut room_npcs: HashMap<String, Vec<NpcInstance>> = HashMap::new();
        let mut next_npc_id: u64 = 1;

        for (room_id, loc) in &cfg.world.locations {
            room_items.insert(room_id.clone(), loc.items.clone());

            let mut npcs = Vec::new();
            for spawn in &loc.spawns {
                let stats = cfg
                    .world
                    .npcs
                    .get(&spawn.npc_type)
                    .map(|n| (n.stats.hp, n.stats.attack))
                    .unwrap_or((1, 0));
                for _ in 0..spawn.count {
                    npcs.push(NpcInstance {
                        id: next_npc_id,
                        npc_type: spawn.npc_type.clone(),
                        hp: stats.0,
                        max_hp: stats.0,
                        attack: stats.1,
                    });
                    next_npc_id += 1;
                }
            }
            room_npcs.insert(room_id.clone(), npcs);
        }

        WorldState {
            room_items,
            room_npcs,
        }
    }

    /// Returns a slice of item IDs present in the specified room.
    pub fn items_in(&self, room: &str) -> &[String] {
        self.room_items.get(room).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Removes an item from a room by ID and returns whether it was removed.
    pub fn remove_item(&mut self, room: &str, item_id: &str) -> bool {
        if let Some(items) = self.room_items.get_mut(room) {
            if let Some(idx) = items.iter().position(|i| i == item_id) {
                items.remove(idx);
                return true;
            }
        }
        false
    }

    /// Adds an item ID to the specified room.
    pub fn add_item(&mut self, room: &str, item_id: String) {
        self.room_items.entry(room.to_string()).or_default().push(item_id);
    }

    /// Returns a slice of NPC instances present in the specified room.
    pub fn npcs_in(&self, room: &str) -> &[NpcInstance] {
        self.room_npcs
            .get(room)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Applies damage to the first NPC of a given type in the room.
    pub fn attack_npc(&mut self, room: &str, npc_type: &str, damage: i32) -> AttackOutcome {
        let npcs = match self.room_npcs.get_mut(room) {
            Some(n) => n,
            None => return AttackOutcome::NoTarget,
        };

        let idx = match npcs.iter().position(|n| n.npc_type == npc_type) {
            Some(i) => i,
            None => return AttackOutcome::NoTarget,
        };

        npcs[idx].hp -= damage.max(0);

        if npcs[idx].hp <= 0 {
            let dead = npcs.remove(idx);
            AttackOutcome::Killed {
                npc_id: dead.id,
                npc_type: dead.npc_type,
            }
        } else {
            AttackOutcome::Hit {
                npc_id: npcs[idx].id,
                npc_type: npcs[idx].npc_type.clone(),
                npc_hp: npcs[idx].hp,
                npc_attack: npcs[idx].attack,
            }
        }
    }
}

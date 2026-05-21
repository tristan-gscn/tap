use super::Config;

/// Validates that every cross-reference in the world data resolves.
///
/// Checks performed:
///   * every exit destination points at a real location;
///   * every spawn `npc_type` exists in the `npcs` table;
///   * every item id listed in a location exists in the `items` table;
///   * every quest id listed on an NPC exists in the `quests` table;
///   * every quest objective target resolves (an NPC for `kill`, an item for `collect`).
///
/// Returns the list of human-readable problems found, or an empty list when the
/// world is internally consistent.
pub fn validate(config: &Config) -> Vec<String> {
    let world = &config.world;
    let mut errors = Vec::new();

    for (id, loc) in &world.locations {
        for (dir, dest) in &loc.exits {
            if !world.locations.contains_key(dest) {
                errors.push(format!(
                    "location '{id}' has exit {dir:?} -> '{dest}', which does not exist"
                ));
            }
        }
        for spawn in &loc.spawns {
            if !world.npcs.contains_key(&spawn.npc_type) {
                errors.push(format!(
                    "location '{id}' spawns unknown npc '{}'",
                    spawn.npc_type
                ));
            }
        }
        for item in &loc.items {
            if !world.items.contains_key(item) {
                errors.push(format!("location '{id}' lists unknown item '{item}'"));
            }
        }
    }

    for (id, npc) in &world.npcs {
        for qid in &npc.quests {
            if !world.quests.contains_key(qid) {
                errors.push(format!("npc '{id}' offers unknown quest '{qid}'"));
            }
        }
    }

    for (id, quest) in &world.quests {
        let target = quest.objective.target();
        match quest.objective.kind() {
            "kill" if !world.npcs.contains_key(target) => {
                errors.push(format!("quest '{id}' targets unknown npc '{target}'"));
            }
            "collect" if !world.items.contains_key(target) => {
                errors.push(format!("quest '{id}' targets unknown item '{target}'"));
            }
            _ => {}
        }
    }

    errors
}

use crate::config;

/// Resolves an item query by ID or name, returning the canonical item ID if found.
pub fn resolve_item(query: &str) -> Option<String> {
    let q = query.trim();
    config::get().world.items.iter().find_map(|(id, item)| {
        if id.eq_ignore_ascii_case(q) || item.name.eq_ignore_ascii_case(q) {
            Some(id.clone())
        } else {
            None
        }
    })
}

/// Resolves an NPC query by ID or name, returning the canonical NPC ID if found.
pub fn resolve_npc(query: &str) -> Option<String> {
    let q = query.trim();
    config::get().world.npcs.iter().find_map(|(id, npc)| {
        if id.eq_ignore_ascii_case(q) || npc.name.eq_ignore_ascii_case(q) {
            Some(id.clone())
        } else {
            None
        }
    })
}

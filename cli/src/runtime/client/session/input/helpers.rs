pub fn is_command(first_word: &str) -> bool {
    matches!(
        first_word,
        "CONNECT"
            | "LOOK"
            | "MOVE"
            | "GO"
            | "QUIT"
            | "CHAT"
            | "WHO"
            | "GROUP"
            | "TAKE"
            | "DROP"
            | "INVENTORY"
            | "ATTACK"
            | "TALK"
            | "QUEST"
            | "QUESTS"
            | "STATUS"
            | "EQUIP"
            | "UNEQUIP"
            | "HELP"
    )
}

/// Returns the in-client command reference shown by the `HELP` command.
pub fn help_lines() -> Vec<String> {
    [
        "── TAP commands ───────────────────────────────",
        "LOOK                     inspect the current room",
        "MOVE <n|s|e|w>           walk through an exit (alias: GO)",
        "TAKE <item>              pick up an item (id or name)",
        "DROP <item>              drop an item back into the room",
        "INVENTORY                list what you carry",
        "EQUIP <right|left> <it>  wield an item in a hand",
        "UNEQUIP <right|left>     free a hand",
        "TALK <npc>               speak with an NPC",
        "ATTACK <npc>             strike a hostile NPC",
        "STATUS                   show your HP / XP / state",
        "QUEST <npc>              ask an NPC for a quest",
        "QUESTS                   summarise your quests",
        "WHO                      who is online",
        "GROUP CREATE|INVITE|JOIN|LEAVE   party management",
        "CHAT <global|room|group> <msg>   send a message",
        "HELP                     show this help",
        "QUIT                     disconnect",
        "Tip: plain text (no command word) is sent as ROOM chat.",
        "───────────────────────────────────────────────",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

pub fn chat_message(input: &str) -> Option<String> {
    let mut parts = input.splitn(3, ' ');
    let verb = parts.next().unwrap_or("").to_uppercase();
    let _scope = parts.next().unwrap_or("");
    let msg = parts.next().unwrap_or("").trim();
    if verb == "CHAT" && !msg.is_empty() {
        Some(msg.to_string())
    } else {
        None
    }
}

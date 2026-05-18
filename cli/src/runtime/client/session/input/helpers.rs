pub fn is_command(first_word: &str) -> bool {
    matches!(
        first_word,
        "CONNECT" | "LOOK" | "MOVE" | "QUIT" | "CHAT" | "WHO" | "GROUP" | "TAKE"
            | "DROP" | "INVENTORY" | "TALK" | "ATTACK" | "STATUS" | "QUEST" | "QUESTS"
    )
}

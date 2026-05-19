pub fn is_command(first_word: &str) -> bool {
    matches!(
        first_word,
        "CONNECT" | "LOOK" | "MOVE" | "GO" | "QUIT" | "CHAT" | "WHO" | "GROUP"
            | "TAKE" | "DROP" | "INVENTORY" | "ATTACK" | "TALK" | "QUEST" | "QUESTS"
    )
}

pub fn chat_message(input: &str) -> Option<String> {
    let mut parts = input.splitn(3, ' ');
    let verb = parts.next().unwrap_or("").to_uppercase();
    let _scope = parts.next().unwrap_or("");
    let msg = parts.next().unwrap_or("").trim();
    if verb == "CHAT" && !msg.is_empty() { Some(msg.to_string()) } else { None }
}

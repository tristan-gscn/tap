use crate::config::Direction;
use crate::protocol::response::Response;

#[derive(Debug)]
pub enum ChatScope {
    Global,
    Room,
    Group,
}

#[derive(Debug)]
pub enum GroupAction {
    Create,
    Invite { target: String },
    Join { leader: String },
    Leave,
}

#[derive(Debug)]
pub enum QuestAction {
    List,
    Summary,
    Request { npc: String },
    Status,
    Accept { id: String },
    Complete { id: String },
}

#[derive(Debug)]
pub enum Command {
    Connect { name: String },
    Who,
    Status,
    Look,
    Chat { scope: ChatScope, text: String },
    Take { item: String },
    Drop { item: String },
    Inventory,
    Group(GroupAction),
    Move { direction: Direction },
    Attack { target: String },
    Talk { target: String },
    Quest(QuestAction),
    Quit,
    Unknown(String),
}

impl Command {
    pub fn parse(line: &str) -> Result<Self, Response> {
        let mut parts = line.splitn(2, ' ');
        let verb = parts.next().unwrap_or("").to_uppercase();
        let rest = parts.next().unwrap_or("").trim();

        match verb.as_str() {
            "CONNECT" => require(rest, "CONNECT requires a name")
                .map(|name| Command::Connect { name }),

            "WHO" => Ok(Command::Who),
            "STATUS" => Ok(Command::Status),
            "LOOK" => Ok(Command::Look),
            "INVENTORY" => Ok(Command::Inventory),
            "QUIT" => Ok(Command::Quit),

            "TAKE" => require(rest, "TAKE requires an item")
                .map(|item| Command::Take { item }),

            "DROP" => require(rest, "DROP requires an item")
                .map(|item| Command::Drop { item }),

            "CHAT" => parse_chat(rest),
            "GROUP" => parse_group(rest),

            "MOVE" | "GO" => match Direction::from_input(rest) {
                Some(direction) => Ok(Command::Move { direction }),
                None => Err(Response::error(
                    400,
                    "MOVE requires a direction (north/south/east/west)",
                )),
            },

            "ATTACK" => require(rest, "ATTACK requires a target")
                .map(|target| Command::Attack { target }),

            "TALK" => require(rest, "TALK requires a target")
                .map(|target| Command::Talk { target }),

            "QUEST" => parse_quest(rest),
            "QUESTS" => Ok(Command::Quest(QuestAction::Summary)),

            other => Ok(Command::Unknown(other.to_string())),
        }
    }
}

fn require(rest: &str, msg: &'static str) -> Result<String, Response> {
    if rest.is_empty() {
        Err(Response::error(400, msg))
    } else {
        Ok(rest.to_string())
    }
}

fn parse_chat(rest: &str) -> Result<Command, Response> {
    let mut p = rest.splitn(2, ' ');
    let scope_raw = p.next().unwrap_or("").to_uppercase();
    let msg = p.next().unwrap_or("").trim();

    let scope = match scope_raw.as_str() {
        "GLOBAL" => ChatScope::Global,
        "ROOM" => ChatScope::Room,
        "GROUP" => ChatScope::Group,
        "" => return Err(Response::error(400, "CHAT requires a scope and a message")),
        other => {
            return Err(Response::error(
                400,
                format!("Unknown chat scope: {}", other),
            ))
        }
    };

    if msg.is_empty() {
        Err(Response::error(400, "CHAT requires a message"))
    } else {
        Ok(Command::Chat {
            scope,
            text: msg.to_string(),
        })
    }
}

fn parse_group(rest: &str) -> Result<Command, Response> {
    let mut p = rest.splitn(2, ' ');
    let sub = p.next().unwrap_or("").to_uppercase();
    let arg = p.next().unwrap_or("").trim();

    let action = match sub.as_str() {
        "CREATE" => GroupAction::Create,
        "LEAVE" => GroupAction::Leave,
        "INVITE" => GroupAction::Invite {
            target: require(arg, "GROUP INVITE requires a username")?,
        },
        "JOIN" => GroupAction::Join {
            leader: require(arg, "GROUP JOIN requires a leader name")?,
        },
        "" => return Err(Response::error(400, "GROUP requires a subcommand")),
        other => {
            return Err(Response::error(
                400,
                format!("Unknown GROUP subcommand: {}", other),
            ))
        }
    };
    Ok(Command::Group(action))
}

fn parse_quest(rest: &str) -> Result<Command, Response> {
    let mut p = rest.splitn(2, ' ');
    let sub = p.next().unwrap_or("").to_uppercase();
    let arg = p.next().unwrap_or("").trim();

    let action = match sub.as_str() {
        "LIST" => QuestAction::List,
        "SUMMARY" => QuestAction::Summary,
        "STATUS" => QuestAction::Status,
        "ACCEPT" => QuestAction::Accept {
            id: require(arg, "QUEST ACCEPT requires a quest id")?,
        },
        "COMPLETE" => QuestAction::Complete {
            id: require(arg, "QUEST COMPLETE requires a quest id")?,
        },
        "" => {
            return Err(Response::error(
                400,
                "QUEST requires an npc name",
            ))
        }
        _ => QuestAction::Request {
            npc: rest.to_string(),
        },
    };
    Ok(Command::Quest(action))
}

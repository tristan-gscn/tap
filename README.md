*This project has been created as part of the 42 curriculum by sgil--de, trgascoi.*

# The Answer Protocol (TAP)

A small, shared-world retro text adventure: a TCP game server that several players
connect to at once, exposed through **two interchangeable clients** — a terminal
client (CLI) and a 3D web client (GUI). The server speaks the line-based
**RFC 42TAP** protocol.

## Description

Players explore a hand-built world of **10 interconnected rooms** in a dark dungeon, chat in real time, pick up and drop items, take quests from
NPCs, and fight the restless dead. The server holds all authoritative state and
pushes asynchronous events (someone entered the room, an NPC died, a chat message)
to every connected client.

The project is split into four components:

| Component  | Path        | Language / stack            | Role                                            |
| ---------- | ----------- | --------------------------- | ----------------------------------------------- |
| **Server** | `server/`   | Rust, `tokio`               | Authoritative TCP game server (RFC 42TAP)        |
| **CLI**    | `cli/`      | Rust, `ratatui`             | Terminal client                                  |
| **GUI**    | `gui/`      | Svelte 5 + Threlte (Three.js) | 3D web client                                  |
| **Bridge** | `bridge/`   | Rust, `tokio`               | WebSocket ↔ TCP gateway (browsers can't open TCP) |

## Instructions

You need a recent **Rust toolchain** (`cargo`, `rustfmt`, `clippy`) and **Node.js**
(for the GUI). All tasks are wrapped in the root `Makefile`:

```bash
make install        # fetch all dependencies (cargo + npm)
make build          # compile every component
make run-server     # terminal 1 — the game server (127.0.0.1:4000)
make run-client     # terminal 2 — the CLI client
# For the GUI you also need the bridge:
make run-bridge     # terminal 3 — WebSocket bridge (127.0.0.1:7878)
make run-client-gui # terminal 4 — GUI dev server (Vite prints a localhost URL)
make lint           # rustfmt --check + clippy -D warnings on all crates
make clean          # remove build artifacts
```

Run `make help` for the full target list.

## Building and Running

The build tool is **`make`** wrapping **Cargo** (Rust) and **npm/Vite** (GUI).

1. **Server** — `make run-server`. Listens on `127.0.0.1:4000`, loads and validates
   `server/world.yml` at startup (it refuses to start if the world has dangling
   references). State is in-memory only and resets on restart.
2. **CLI** — `make run-client`. A full-screen terminal UI; it connects to
   `127.0.0.1:4000`, prompts for a name/class, and stays responsive to events while
   you type. Type `HELP` for the command list.
3. **Bridge** — `make run-bridge`. Needed only for the GUI. Listens on
   `127.0.0.1:7878` and forwards to the server. Configurable via `TAP_BRIDGE_ADDR`
   and `TAP_SERVER_ADDR`.
4. **GUI** — `make run-client-gui`. Open the URL Vite prints. Requires the server
   **and** the bridge to be running.

Both clients implement **every** command and event in the protocol, so a CLI from
one group and a GUI from another can talk to the same server interchangeably.

## Architecture

The server uses a **dispatcher/router** design and an **async, task-per-connection**
concurrency model on `tokio`:

- `network::listener` accepts TCP connections and spawns one task per client.
- `network::handle` runs the per-connection loop: it splits the socket into a reader
  and a writer task (the writer drains an `mpsc` channel so broadcasts never block on
  a slow or disconnecting peer), reads newline-delimited lines, and frames them.
- `protocol::command::Command::parse` turns a raw line into a typed `Command`.
- `network::dispatch::dispatch` routes that `Command` to a focused handler module
  (`session`, `world`, `inventory`, `combat`, `chat`, `group`, `quest`).
- Shared state lives in a single `Arc<RwLock<GameState>>`: read-mostly commands take a
  read lock, mutating commands take a write lock.

Static world data is loaded once into a global `OnceLock<Config>` from `world.yml`.

A detailed, diagram-driven walkthrough lives in **[docs.md](docs.md)**.

## Protocol Implementation

We follow RFC 42TAP's command set, arguments, error codes, and event semantics, with
**one deliberate, documented deviation**:

- **JSON response envelope.** Instead of the RFC's plaintext `OK ...` / `ERR <code>
  <msg>` lines, every server reply is a single JSON line:
  - success: `{"status":"ok","type":"<kind>","data":{...}}`
  - error:   `{"status":"error","code":<n>,"message":"<msg>"}`
  - event:   `{"status":"ok","type":"event","data":{"event":"<name>", ...}}`

  **Justification:** both clients need to parse rich, nested payloads (room layouts,
  NPC lists with HP, quest progress). A uniform JSON envelope removes ad-hoc string
  parsing, keeps framing trivial (still exactly one message per `\n`), and lets the
  GUI and CLI share the same decode path. The logical contract of the RFC (the same
  commands, the same numeric error codes such as `404`, `405 NPC_NOT_HOSTILE`,
  `301`/`400` for movement, the same event names) is preserved inside the envelope.

Extra, additive commands beyond the RFC: `EQUIP` / `UNEQUIP` (hand slots) and a
client-side `HELP` in the CLI (never sent over the wire).

## Combat System

- Players start at **100 HP** with a base attack of **10**.
- `ATTACK <npc>` deals the player's *effective attack* to one NPC instance. Effective
  attack = base + **+4 per equipped weapon** (sword/axe/dagger/bow/crossbow/staff/wand).
- The NPC immediately **counter-attacks** for its `attack` stat, reduced by **2** if the
  player has a shield equipped.
- NPCs are unique instances: an NPC at 0 HP is removed from the room and broadcast as
  `npc_killed`; a surviving hit broadcasts `npc_attacked` with the new HP.
- A player reduced to **0 HP respawns** at the safe village square (`start`) at full HP,
  and receives a `player_defeated` event.
- **Peaceful NPCs** (`hostile: false` in the world) reject `ATTACK` with
  `405 NPC_NOT_HOSTILE`; an absent NPC yields `404 NPC_NOT_FOUND`.
- **Inventory cap:** a player carries at most **16 items**; `TAKE` past that returns
  `407 INVENTORY_FULL` and leaves the item in the room.

**Design choice:** combat is **single-exchange and deterministic** (no RNG, no turn
queue) so it is trivial to reason about and to demo in an evaluation. Initiative is
fixed: the player strikes, then the target counters. Damage formulas are flat additive
values driven entirely by world data (`stats.hp`, `stats.attack`) and equipment, so
balancing is a data edit, not a code change.

## Quest System

- Quests are declared in `world.yml` with an **objective** (`kind: kill|collect`,
  `target`, `count`) and a **reward** (`xp`). NPCs advertise quest ids in their
  `quests:` list.
- **Acquisition:** a quest is taken by **asking its quest-giver** — `QUEST <npc>`
  returns the first quest that NPC offers which the player hasn't started, and accepts
  it (a `quest_accepted` event is pushed). In the GUI this is done by clicking the NPC;
  there is no "claim" button. `QUEST COMPLETE <id>` turns a finished quest in.
- **Progression & validation:** progress is tracked **server-side** per player. Killing
  a matching NPC (`kill`) or taking a matching item (`collect`) calls
  `advance_quests`, which increments only matching, non-completed objectives and emits
  `quest_progress`. Progress is clamped to the required count, so it can't be inflated.
- **Completion & reward:** once `progress >= count`, completing the quest marks it done
  and grants its `xp`. The objectives shipped: `clear_hall` (kill 2 rogue skeletons),
  `silence_mage` (kill the vault mage), `gather_herbs` (collect 2 healing herbs).

## World Design

A coherent map of **10 rooms** — a village cluster opening onto a dungeon chain:

```
              garden
                |
  start ────── market ────── tavern
    │            │
  smithy ────── gate ────── entrance ── hall ── vault ── crypt
```

- **Loop:** `start → market → gate → smithy → start` (a true circuit — no line-only map).
- **Branches:** `garden` (off the square), `tavern` (off the market), and the entire
  dungeon chain hanging off the `gate`.
- **NPC roles (3 distinct):** *quest-givers* (Captain Aldric, Merchant Bo — peaceful),
  *dialogue* NPCs (Barkeep Mira, Blacksmith Goran — peaceful), and *enemies*
  (skeleton warrior / rogue / mage — hostile).
- **Items:** 12 distinct items, the majority obtainable in-world (herbs, ale, bread,
  sword, shield, dagger, axe, staff, wand, torch, bone dust, ancient coin).

`start` is the spawn and respawn room (safe; no enemies). All rooms render with the
GUI's dungeon scene; rooms are distinguished narratively by their name and description.

## Server Logging

Logging uses **`tracing`** + `tracing-subscriber` with two layers:

- a **human console** layer (level + wall-clock time), and
- a **structured JSON** layer written to **daily files** under
  `server/logs/<YYYY-MM-DD>/stdout.jsonl` and `stderr.jsonl` (one JSON object per line,
  ISO-8601 timestamps).

Levels (`INFO` / `WARN` / `ERROR`) are controlled by `RUST_LOG` (or `.env`), defaulting
to `info`. Logged events include: connections/disconnections with peer address,
received commands, world-state changes (item taken/dropped, NPC attacked/killed),
combat outcomes and respawns, and quest progress/completion. To monitor behaviour,
`tail -f server/logs/<date>/stdout.jsonl | jq` gives a live structured feed.

## Testing

- **Lint/build:** `make lint` (rustfmt + clippy `-D warnings`) and `make build`.
- **Manual multiplayer:** start the server, connect two CLI clients (and/or the GUI via
  the bridge) with different names. Verify `presence_enter`/`presence_leave` events,
  `CHAT GLOBAL/ROOM/GROUP` scoping, and that `TAKE` in one client removes the item from
  the other client's `LOOK` (no duplication).
- **Combat:** `ATTACK skeleton_rogue` in the hall; watch HP fall on both sides, and a
  defeated player respawn at the square. `ATTACK captain` must return
  `405 NPC_NOT_HOSTILE`.
- **Quests:** `QUEST captain` then clear the hall — watch `quest_progress` then
  completion and XP gain. `QUEST merchant` then `TAKE herbs` twice.
- **Raw protocol:** `nc 127.0.0.1 4000`, then type `CONNECT alice`, `LOOK`, etc., to see
  the JSON envelope directly.

## Group Contributions

- **sgil--de** — Server (protocol, dispatcher, state, combat/quest systems), world design, logging.
- **trgascoi** — GUI client (Threlte scene, panels, room styles), Bridge, CLI client.

## Resources

- RFC 42TAP (`protocol-rfc.html`) and the project subject (`tap.pdf`).
- [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) — requirement keywords.
- [Tokio](https://tokio.rs/) — async runtime & networking.
- [ratatui](https://ratatui.rs/) — terminal UI.
- [Svelte 5](https://svelte.dev/) · [Threlte](https://threlte.xyz/) · [Three.js](https://threejs.org/) — 3D web client.
- [Kenney](https://kenney.nl/) — dungeon 3D art, character and skeleton models.

**Use of AI:** AI assistance was used to (a) cross-check the implementation against the
RFC and subject and surface missing pieces, (b) scaffold repetitive code (the CLI
`HELP`/command-parity wiring, the Makefile), and (c) draft this documentation. Every
generated change was reviewed, built, linted, and adjusted by hand.

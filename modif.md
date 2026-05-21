# Modifications — what changed and why

This file lists every change made during the "make the project match the subject" pass.
Goal: close the gaps against the TAP subject + RFC 42TAP, add the missing tooling/docs,
bring the CLI to feature parity with the GUI, and add a second (village) room style.

---

## 1. Server — world data & rules

### `server/world.yml` (rewritten)
- Grew the world from **3 rooms to 10 interconnected rooms** with a real **loop**
  (`start → market → gate → smithy → start`) plus **branches** (`garden`, `tavern`,
  and the whole dungeon chain) — satisfies "≥8 rooms, one loop, ≥1 branch, full circuit".
- Two room **styles** via `kind`: `village` (exterior) and `dungeon` (interior).
- **3 distinct NPC roles:** quest-givers (Captain Aldric, Merchant Bo), dialogue NPCs
  (Barkeep Mira, Blacksmith Goran), enemies (skeleton warrior/rogue/mage).
- **12 items** (most obtainable), **3 quests** (`clear_hall` kill×2, `silence_mage`
  kill×1, `gather_herbs` collect×2).
- Added per-room **`props`** (decorative scenery) and per-NPC **`hostile`** flags.
- Kept `start` as the spawn/respawn room (the server hard-codes it).

### `server/src/config/config/location.rs`
- Added `props: Vec<String>` (decorative scenery, `#[serde(default)]`).

### `server/src/config/config/npc.rs`
- Added `hostile: bool` (defaults to `true` so existing data stays hostile).

### `server/src/config/validate.rs` (new) + `config/mod.rs` + `main.rs`
- New world **validation** step run at startup: checks every exit destination, spawn
  `npc_type`, room item, NPC quest id, and quest objective target resolve. The server
  **refuses to start** on a broken world and logs each problem — satisfies "Validates
  that all exits and references in the world are correct".
- `main.rs` now logs a `rooms/npcs/items/quests` summary once validated.

### `server/src/network/handlers/combat/attack.rs`
- Enforces hostility: absent NPC → `404 NPC_NOT_FOUND`, peaceful NPC →
  `405 NPC_NOT_HOSTILE` (the RFC error code) before any damage.

### `server/src/network/handlers/world/look.rs`
- `LOOK` now includes `room.props` so clients can render scenery.

---

## 2. CLI — feature parity with the GUI + HELP

### `cli/src/runtime/client/session/input/helpers.rs`
- Registered `STATUS`, `EQUIP`, `UNEQUIP`, `HELP` as commands (previously these were
  wrongly treated as chat text).
- Added `help_lines()` — the command reference printed by `HELP`.

### `cli/src/runtime/client/session/input.rs`
- `HELP` is intercepted **client-side** and prints the reference to the log (it is not
  part of the wire protocol).

### `cli/src/runtime/client/session/input/response.rs`
- New handlers: **TALK** (prints NPC dialogue), **STATUS** (updates + logs HP/XP/state),
  **TAKE/DROP** (refresh room + inventory), **EQUIP/UNEQUIP** (refresh inventory).

### `cli/src/runtime/client/protocol/types.rs`
- `LookRoom` gained `props`; `LookNpc` gained an optional display `name`;
  `InventoryResponse` gained `equipped` (right/left slots).

### `cli/src/app/mocks/room.rs`, `mocks/status.rs`, `session/send.rs`
- `RoomMock` carries `props`; NPC labels now use the **display name** when present.
- `PlayerStatusMock` carries `equipped_right/left`; inventory apply records them.
- Added `refresh_inventory()` alongside `refresh_look()`.

### `cli/src/ui/panels/adventure.rs`, `panels/character.rs`
- Room panel shows a **Scenery** line (props).
- Inventory panel shows the **equipped hands** in its title and tags equipped items.

Result: the CLI now drives every GUI action (LOOK, MOVE, TAKE, DROP, TALK, ATTACK,
STATUS, QUEST, QUESTS, WHO, GROUP, QUIT) plus EQUIP/UNEQUIP and HELP.

---

## 3. GUI — second room style (village) with props

### `gui/src/room/village.svelte` (new) + `gui/src/room/VillageProp.svelte` (new)
- A daylight **exterior** room: grass plane + dirt path, sky/sun lighting, clickable
  fence-gate exits, and the room's `props` loaded from `room/village/<name>.glb` and
  scattered at deterministic spots. Same 38×18 footprint as the dungeon so positioning
  is unchanged.

### `gui/src/registries/rooms.ts`
- Registered the `village` kind; `RoomRenderProps` now carries `props`.

### `gui/src/registries/npcModels.ts`
- Mapped living-character models (`knight`, `ranger`, `barbarian`, `rogue_hooded`) so
  peaceful villagers render as people, not skeletons (skeleton models unchanged).

### `gui/src/state/game.svelte.ts` + `gui/src/Scene.svelte`
- `RoomData` gained optional `props`; `Scene` forwards `game.room.props` to the room.

---

## 4. Tooling & documentation (new files)

- **`Makefile`** (root) — `install`, `build`, `run-server`, `run-client`, `run-bridge`,
  `run-client-gui`, `lint`, `fmt`, `clean`, `help`.
- **`README.md`** (root) — all subject-required sections (Description, Instructions,
  Resources + AI usage, Architecture, Protocol Implementation, Combat System, Quest
  System, World Design, Server Logging, Group Contributions, Building and Running,
  Testing).
- **`docs.md`** — technical deep-dive with mermaid diagrams (topology, modules,
  lifecycle, state model, world map, protocol, combat, quests, GUI).
- **`modif.md`** — this file.

---

## 5. Lint

- Ran `cargo fmt` across `server/`, `cli/`, `bridge/` (whole tree now formatted).
- Fixed clippy findings so `make lint` (`clippy -D warnings`) is clean:
  - `server`: `map_or(true, …)` → `is_none_or(…)`; `#[allow(clippy::module_inception)]`
    on the `config::config` module.
  - `cli`: `io::Error::new(ErrorKind::Other, …)` → `io::Error::other(…)` (×3).

---

## 6. Revert of the village room style + asset cleanup + object lighting

A later pass removed the experimental exterior "village" room style and its props, on
request, and trimmed the unused art:

- **GUI village removed:** deleted `room/village.svelte`, `room/VillageProp.svelte`,
  `room/FbxModel.svelte`, `utils/groundTexture.ts`; unregistered `village` in
  `registries/rooms.ts`; reverted `Scene.svelte` (no `props`), `RoomData` (no `props`),
  `npcModels.ts` (skeletons only again), and `App.svelte` (`<Canvas>` without shadows).
- **Server:** removed the `props` field from `Location` and from the `LOOK` payload;
  removed every `kind:`/`props:` entry from `world.yml` (all rooms now use the default
  dungeon style). The 10-room map, `hostile` flags, and validation are kept.
- **CLI:** removed the props plumbing (`LookRoom.props`, `RoomMock.props`, the room
  "Scenery" line). The `HELP`/`STATUS`/`EQUIP`/`UNEQUIP` commands, NPC display names,
  equipped-slot display, and TALK dialogue are kept.
- **Unused models deleted:** the whole `public/room/village/` pack; all unused
  `public/room/dungeon/gltf/*` (kept only the 6 referenced models + their `.bin` + the
  shared texture); all unused `public/models/props/adventurers/*` (kept only
  `sword_1handed` + `shield_round` + `knight_texture.png`); the entire
  `public/models/props/skeletons/` folder; and the dead `actors/Props.ts` enum.
- **Lighting:** `Scene.svelte` now adds a point light around every object — a cool glow
  above each NPC (brighter on hover) and a warm glow over each ground item.

---

## 7. Inventory cap, NPC interaction & quest acquisition

- **Server — inventory cap:** `TAKE` now refuses to pick up an item when the player
  already carries `MAX_INVENTORY = 16` items, returning `407 INVENTORY_FULL`; the item
  stays in the room (`server/src/network/handlers/inventory/take.rs`,
  `state/player.rs`).
- **Server — hostility in LOOK:** the `LOOK` NPC payload now carries `hostile` so
  clients can tell quest-givers/dialogue NPCs from enemies.
- **GUI — loot sack fallback:** items without a dedicated 3D prop now drop a procedural
  loot sack (`actors/SackItem.svelte`) instead of being invisible; still clickable to
  pick up and lit by the per-item glow.
- **GUI — talking to peaceful NPCs:** clicking a non-hostile NPC now talks to it and
  asks it for a quest (instead of a no-op attack); hostile NPCs are attacked on
  left-click and talked to on right-click.
- **GUI — quests come from NPCs:** removed the "accept" (claim) button from the quest
  panel. A quest can only be taken by interacting with its quest-giver in the world; the
  panel shows "Ask the quest-giver in person to take this quest." (Completion/turn-in is
  unchanged.)

---

## Follow-ups for the team

- Fill the **42 logins** on README line 1 and the **Group Contributions** section.
- The JSON response envelope is the one intentional RFC deviation — keep it documented.

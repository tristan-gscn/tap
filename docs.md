# The Answer Protocol — Technical Documentation

This document is the technical deep-dive that complements [README.md](README.md). It
explains how the system is wired together, the data model, the protocol on the wire,
and each gameplay subsystem, with diagrams.

---

## 1. System topology

Four processes cooperate. The CLI talks raw TCP; the browser GUI cannot open TCP
sockets, so it goes through the WebSocket bridge.

```mermaid
flowchart LR
    CLI["CLI client<br/>(ratatui, Rust)"] -- "TCP : 4000<br/>JSON lines" --> SRV["TAP Server<br/>(tokio, Rust)"]
    GUI["GUI client<br/>(Svelte + Threlte)"] -- "WebSocket : 7878" --> BR["Bridge<br/>(tokio, Rust)"]
    BR -- "TCP : 4000" --> SRV
    SRV -- "loads / validates" --> WORLD[("world.yml")]
    SRV -- "structured logs" --> LOGS[("logs/&lt;date&gt;/*.jsonl")]
```

Both clients are **interchangeable**: they implement the same command set and consume
the same events, so any compliant client can drive the server.

---

## 2. Server module map

```mermaid
flowchart TD
    main["main.rs<br/>init logger · load+validate world · start listener"]
    subgraph config
        cfg["config::*<br/>WorldData, Location, Npc, Item, Quest"]
        val["validate<br/>(exits / refs / quest targets)"]
    end
    subgraph network
        lis["listener<br/>accept loop"]
        hdl["handle<br/>per-connection reader/writer"]
        dsp["dispatch<br/>Command → handler"]
        subgraph handlers
            sess[session]
            wld[world]
            inv[inventory]
            cbt[combat]
            cht[chat]
            grp[group]
            qst[quest]
        end
    end
    subgraph protocol
        cmd["command::parse"]
        rsp["response (JSON envelope)"]
    end
    subgraph state
        gs["GameState<br/>players, groups"]
        ws["WorldState<br/>room_items, room_npcs"]
        pl["Player"]
    end

    main --> cfg --> val
    main --> lis --> hdl --> cmd --> dsp --> handlers
    handlers --> rsp
    handlers --> gs
    gs --> ws
    gs --> pl
```

Handlers are small and single-purpose; `dispatch` is the only place that knows the full
command → handler mapping.

---

## 3. Connection lifecycle

The per-connection task splits the socket so that a separate writer task drains an
`mpsc` channel. This is what lets the server **broadcast without blocking** if one
client is slow or disconnects mid-send.

```mermaid
sequenceDiagram
    participant C as Client
    participant H as handle (reader)
    participant W as writer task
    participant D as dispatch
    participant S as GameState

    C->>H: TCP connect
    Note over H,W: spawn writer task draining mpsc<Response>
    C->>H: CONNECT alice knight\n
    H->>D: Command::Connect
    D->>S: insert Player (room = "start")
    D-->>W: {"status":"ok","type":"connect",...}
    W-->>C: JSON line
    loop each command line
        C->>H: e.g. MOVE north\n
        H->>D: parsed Command
        D->>S: read/write under RwLock
        D-->>W: response (to this client)
        D-->>W: events (to other clients in room/group)
    end
    C--xH: disconnect (EOF)
    H->>S: remove player, broadcast presence_leave
```

Per the global rules, player state is removed **before** the leave event is broadcast.

---

## 4. State model

```mermaid
classDiagram
    class GameState {
        players: HashMap<String, Player>
        groups: GroupRegistry
        world: WorldState
        +name_of(addr)
        +broadcast_room(room, except, resp)
        +send_to(name, resp)
    }
    class WorldState {
        room_items: HashMap<String, Vec<String>>
        room_npcs: HashMap<String, Vec<NpcInstance>>
        +items_in(room) / npcs_in(room)
        +attack_npc(room, type, dmg) AttackOutcome
    }
    class Player {
        name, addr, class, room
        hp, max_hp, attack, xp
        inventory, equipped_right/left
        quests: HashMap<String, QuestProgress>
        +effective_attack() / shield_reduction()
        +advance_quests(kind, target, n)
        +respawn()
    }
    class NpcInstance {
        id, npc_type, hp, max_hp, attack
    }
    GameState --> WorldState
    GameState "1" --> "*" Player
    WorldState "1" --> "*" NpcInstance
```

Static, immutable data (room descriptions, NPC stats, quest definitions) lives in the
global `Config`. Mutable, per-session data (who is where, what's been picked up, which
NPCs are still alive) lives in `GameState`. `start` is special: it is hard-coded as the
spawn and respawn room, so it is always present and safe.

---

## 5. World map

```mermaid
flowchart TD
    garden["garden 🌿<br/>(village)"]
    start["start ⛲ Village Square<br/>(village, spawn)"]
    market["market 🏪<br/>(village)"]
    tavern["tavern 🍺<br/>(village)"]
    smithy["smithy 🔨<br/>(village)"]
    gate["gate 🏛️ Old Gate<br/>(village)"]
    entrance["entrance 💀<br/>(dungeon)"]
    hall["hall 🦴<br/>(dungeon)"]
    vault["vault 🔮<br/>(dungeon)"]
    crypt["crypt ⚰️<br/>(dungeon)"]

    start <--> garden
    start <--> market
    start <--> smithy
    market <--> tavern
    market <--> gate
    smithy <--> gate
    gate <--> entrance
    entrance <--> hall
    hall <--> vault
    vault <--> crypt
```

The `start / market / gate / smithy` square is the loop; `garden`, `tavern`, and the
dungeon chain are branches. Rooms are themed by name/description (village vs. dungeon)
but all render with the GUI's single dungeon scene.

---

## 6. Protocol on the wire

Transport: TCP, UTF-8, one message per `\n`. Every server message is one JSON object.

```mermaid
flowchart LR
    subgraph "Client → Server (plain text verbs)"
      A["CONNECT alice knight"]
      B["LOOK"]
      C["MOVE north"]
      E["ATTACK skeleton_rogue"]
    end
    subgraph "Server → Client (JSON envelope)"
      OK["{ status: ok, type, data }"]
      ERR["{ status: error, code, message }"]
      EVT["{ status: ok, type: event, data: { event, ... } }"]
    end
    A --> OK
    B --> OK
    C --> OK
    E --> OK
    A -.NAME_IN_USE.-> ERR
    E -.404 / 405.-> ERR
    C -.301 NO_EXIT.-> ERR
```

This JSON envelope is the project's single, intentional deviation from RFC 42TAP's
plaintext replies; the rationale and the preserved error codes/events are documented in
the README's *Protocol Implementation* section. Key events: `presence_enter/leave`,
`chat_global/room/group`, `npc_attacked`, `npc_killed`, `player_defeated`,
`item_taken/dropped`, `quest_accepted`, `quest_progress`, `quest_completed`.

---

## 7. Combat

```mermaid
sequenceDiagram
    participant P as Player
    participant Sv as Server
    participant R as Room

    P->>Sv: ATTACK skeleton_rogue
    alt NPC absent
        Sv-->>P: error 404 NPC_NOT_FOUND
    else NPC peaceful (hostile=false)
        Sv-->>P: error 405 NPC_NOT_HOSTILE
    else hit
        Sv->>Sv: npc.hp -= effective_attack()
        alt npc.hp <= 0
            Sv->>R: broadcast npc_killed
            Sv->>Sv: advance_quests("kill", type)
        else still alive
            Sv->>R: broadcast npc_attacked (new hp)
            Sv->>Sv: player.hp -= max(0, npc.attack - shield)
            opt player.hp <= 0
                Sv->>Sv: respawn at "start"
                Sv-->>P: player_defeated
            end
        end
        Sv-->>P: ok attack { ... }
    end
```

Deterministic, single-exchange resolution; all numbers come from world data and
equipment (see README *Combat System*).

---

## 8. Quests

```mermaid
stateDiagram-v2
    [*] --> Available: declared on an NPC
    Available --> Active: QUEST <npc> (auto-accept) / QUEST ACCEPT
    Active --> Active: advance_quests on kill/collect (clamped) → quest_progress
    Active --> Completable: progress >= count
    Completable --> Completed: QUEST COMPLETE → grant xp → quest_completed
    Completed --> [*]
```

Progress is server-authoritative and clamped to the required count, so a client cannot
over-report. The GUI auto-completes completable quests; the CLI exposes the lifecycle
explicitly via `QUEST` / `QUESTS`.

---

## 9. GUI architecture

```mermaid
flowchart TD
    TAP["TAPManager<br/>(WebSocket to bridge)"] --> GS["game.svelte.ts<br/>($state store)"]
    GS --> Scene["Scene.svelte"]
    Scene --> RoomReg["registries/rooms.ts<br/>resolveRoom(kind)"]
    RoomReg --> Dungeon["room/dungeon.svelte"]
    Scene --> NpcReg["registries/npcModels.ts"]
    Scene --> Items["GroundItem · itemProps"]
    GS --> HUD["ui/HUD + panels<br/>(Room, Inventory, Quest, Status, Who, Chat, Dialogue)"]
```

`game.svelte.ts` is the single reactive store: it sends commands through `TAPManager`,
applies responses, and reacts to pushed events by refreshing the relevant slice (look,
inventory, status, quests, who). `Scene.svelte` resolves a room component by the room's
`kind` (only `dungeon` is registered) and renders NPCs, ground items, and other players
at deterministic positions (`utils/positions.ts`) on the room's 38×18 footprint.

### Lighting

The dungeon is lit by a dim ambient/hemisphere base plus warm torch point-lights around
the walls. On top of that, `Scene.svelte` adds a **per-object glow**: a cool point-light
above each NPC (brighter on hover) and a warm point-light over each ground item, so
interactable objects stand out from the gloom.

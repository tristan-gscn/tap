import { ActorAnimation } from '../actors/ActorAnimation';
import { TAPManager, type ChatScope, type Direction, type TapOk, type TapResponse } from '../utils/TAPManager';

export type RoomData = {
    id: string;
    kind: string;
    name: string;
    description: string;
    exits: Record<string, string>;
};

export type NpcInfo = {
    id: number;
    type: string;
    name: string;
    model: string | null;
    hp: number;
    max_hp: number;
};

export type PlayerDetail = {
    name: string;
    class: string | null;
    hp: number;
    max_hp: number;
};

export type ChatEntry = {
    scope: string;
    from: string;
    text: string;
    ts: number;
};

export type EventEntry = {
    text: string;
    ts: number;
};

export type QuestEntry = {
    id: string;
    name: string;
    description: string;
    objective: { kind: string; target: string; count: number };
    reward: { xp: number };
    status: 'available' | 'active' | 'completed';
};

export type ActiveQuest = {
    id: string;
    name: string;
    progress: number;
    required: number;
    completed: boolean;
};

export type Dialogue = {
    npc: string;
    name: string;
    description: string;
    dialogue: string[];
};

export type ItemDetail = { id: string; name: string };

export const PLAYER_SPAWN: [number, number, number] = [19, 0, 14];
const ROOM_LAST_X = 38;
const ROOM_LAST_Z = 18;

function spawnForDirection(direction: Direction): [number, number, number] {
    // When entering a room from <direction>, we appear at the OPPOSITE door.
    switch (direction) {
        case 'north':
            return [ROOM_LAST_X / 2, 0, ROOM_LAST_Z - 2];
        case 'south':
            return [ROOM_LAST_X / 2, 0, 2];
        case 'east':
            return [2, 0, ROOM_LAST_Z / 2];
        case 'west':
            return [ROOM_LAST_X - 2, 0, ROOM_LAST_Z / 2];
    }
}

function ok<T>(resp: TapResponse, fallback: T): T {
    if (resp.status === 'ok' && resp.data !== undefined && resp.data !== null) {
        return resp.data as T;
    }
    return fallback;
}

class GameStore {
    connected = $state(false);
    connecting = $state(false);
    error = $state<string | null>(null);

    playerName = $state('');
    playerClass = $state('knight');

    room = $state<RoomData | null>(null);
    npcs = $state<NpcInfo[]>([]);
    items = $state<string[]>([]);
    itemsDetail = $state<ItemDetail[]>([]);
    players = $state<string[]>([]);
    playersDetail = $state<PlayerDetail[]>([]);

    hp = $state(100);
    maxHp = $state(100);
    xp = $state(0);
    status = $state('healthy');
    inventory = $state<string[]>([]);
    inventoryDetail = $state<ItemDetail[]>([]);

    playerPos = $state<[number, number, number]>([...PLAYER_SPAWN]);
    playerTarget = $state<[number, number, number]>([...PLAYER_SPAWN]);
    playerAnimation = $state<ActorAnimation>(ActorAnimation.Idle_A);
    attacking = $state(false);

    questsAll = $state<QuestEntry[]>([]);
    questsActive = $state<ActiveQuest[]>([]);

    chatLog = $state<ChatEntry[]>([]);
    eventLog = $state<EventEntry[]>([]);
    onlinePlayers = $state<string[]>([]);

    dialogue = $state<Dialogue | null>(null);

    tap = new TAPManager();
    private unsubEvent: (() => void) | null = null;

    async connect(name: string, playerClass: string): Promise<boolean> {
        if (this.connecting || this.connected) return this.connected;
        this.connecting = true;
        this.error = null;
        this.playerName = name;
        this.playerClass = playerClass;

        try {
            await this.tap.connect();
        } catch {
            this.connecting = false;
            this.error = 'Bridge unreachable. Is the bridge running on ws://localhost:7878?';
            return false;
        }

        const resp = await this.tap.connectPlayer(name, playerClass);
        if (resp.status !== 'ok') {
            this.connecting = false;
            this.error = resp.message ?? 'Connection refused';
            return false;
        }

        this.bindEvents();
        this.connected = true;
        this.connecting = false;

        await Promise.all([
            this.refreshLook(),
            this.refreshInventory(),
            this.refreshStatus(),
            this.refreshQuestList(),
            this.refreshQuestStatus(),
            this.refreshWho(),
        ]);
        return true;
    }

    disconnect() {
        this.unsubEvent?.();
        this.unsubEvent = null;
        this.connected = false;
        this.error = null;
    }

    private bindEvents() {
        this.unsubEvent?.();
        this.unsubEvent = this.tap.onEvent((event: TapOk) => this.handleEvent(event));
    }

    private pushEvent(text: string) {
        this.eventLog = [...this.eventLog, { text, ts: Date.now() }].slice(-50);
    }

    private handleEvent(event: TapOk) {
        const data = (event.data ?? {}) as Record<string, unknown>;
        const kind = data.event as string | undefined;
        if (!kind) return;

        switch (kind) {
            case 'presence_enter': {
                const who = data.name as string;
                if (who && who !== this.playerName) {
                    this.pushEvent(`${who} entered the room`);
                }
                void this.refreshLook();
                void this.refreshWho();
                break;
            }
            case 'presence_leave': {
                const who = data.name as string;
                if (who && who !== this.playerName) {
                    this.pushEvent(`${who} left the room`);
                }
                void this.refreshLook();
                void this.refreshWho();
                break;
            }
            case 'npc_attacked': {
                const npcId = data.npc_id as number;
                const hp = data.npc_hp as number;
                this.npcs = this.npcs.map((n) => (n.id === npcId ? { ...n, hp } : n));
                break;
            }
            case 'npc_killed': {
                const npcId = data.npc_id as number;
                this.npcs = this.npcs.filter((n) => n.id !== npcId);
                this.pushEvent(`${data.by ?? '?'} slew ${data.npc ?? '?'}`);
                break;
            }
            case 'player_defeated': {
                this.pushEvent(`You were defeated by ${data.by ?? '?'}!`);
                void this.refreshLook();
                void this.refreshStatus();
                break;
            }
            case 'chat_global':
            case 'chat_room':
            case 'chat_group': {
                this.chatLog = [
                    ...this.chatLog,
                    {
                        scope: data.scope as string,
                        from: data.from as string,
                        text: data.text as string,
                        ts: Date.now(),
                    },
                ].slice(-100);
                break;
            }
            case 'quest_progress':
            case 'quest_completed': {
                void this.refreshQuestStatus();
                this.pushEvent(`Quest update: ${data.id ?? ''}`);
                break;
            }
        }
    }

    async refreshLook() {
        const resp = await this.tap.look();
        if (resp.status !== 'ok') return;
        type LookData = {
            room: RoomData;
            npcs: NpcInfo[];
            items: string[];
            items_detail?: ItemDetail[];
            players: string[];
            players_detail?: PlayerDetail[];
        };
        const data = ok<LookData>(resp, {
            room: { id: '', kind: 'dungeon', name: '', description: '', exits: {} },
            npcs: [],
            items: [],
            items_detail: [],
            players: [],
            players_detail: [],
        });
        this.room = data.room;
        this.npcs = data.npcs ?? [];
        this.items = data.items ?? [];
        this.itemsDetail = data.items_detail ?? (data.items ?? []).map((id) => ({ id, name: id }));
        this.players = data.players ?? [];
        this.playersDetail = data.players_detail ?? [];
    }

    async refreshInventory() {
        const resp = await this.tap.inventory();
        if (resp.status !== 'ok') return;
        const data = ok<{ items: string[]; items_detail?: ItemDetail[] }>(resp, { items: [] });
        this.inventory = data.items ?? [];
        this.inventoryDetail = data.items_detail ?? (data.items ?? []).map((id) => ({ id, name: id }));
    }

    setMoveTarget(pos: [number, number, number]) {
        this.playerTarget = pos;
    }

    teleportPlayer(pos: [number, number, number]) {
        this.playerPos = [...pos];
        this.playerTarget = [...pos];
        this.playerAnimation = ActorAnimation.Idle_A;
    }

    async refreshStatus() {
        const resp = await this.tap.status();
        if (resp.status !== 'ok') return;
        type StatusData = {
            hp: number;
            max_hp: number;
            xp?: number;
            status: string;
            class?: string;
        };
        const data = ok<StatusData>(resp, { hp: 0, max_hp: 100, status: 'healthy' });
        this.hp = data.hp;
        this.maxHp = data.max_hp;
        this.xp = data.xp ?? 0;
        this.status = data.status;
        if (data.class) this.playerClass = data.class;
    }

    async refreshQuestList() {
        const resp = await this.tap.questList();
        if (resp.status !== 'ok') return;
        const data = ok<{ quests: QuestEntry[] }>(resp, { quests: [] });
        this.questsAll = data.quests ?? [];
    }

    async refreshQuestStatus() {
        const resp = await this.tap.questStatus();
        if (resp.status !== 'ok') return;
        const data = ok<{ quests: ActiveQuest[]; xp: number }>(resp, { quests: [], xp: 0 });
        this.questsActive = data.quests ?? [];
        this.xp = data.xp ?? this.xp;
    }

    async refreshWho() {
        const resp = await this.tap.who();
        if (resp.status !== 'ok') return;
        const data = ok<{ players: string[] }>(resp, { players: [] });
        this.onlinePlayers = data.players ?? [];
    }

    async move(direction: Direction) {
        const resp = await this.tap.move(direction);
        if (resp.status !== 'ok') {
            this.pushEvent(`Move failed: ${resp.message}`);
            return;
        }
        this.dialogue = null;
        const spawn = spawnForDirection(direction);
        this.teleportPlayer(spawn);
        await Promise.all([this.refreshLook(), this.refreshInventory()]);
    }

    async take(item: string) {
        const resp = await this.tap.take(item);
        if (resp.status !== 'ok') {
            this.pushEvent(`Take failed: ${resp.message}`);
            return;
        }
        await Promise.all([this.refreshLook(), this.refreshInventory()]);
    }

    async drop(item: string) {
        const resp = await this.tap.drop(item);
        if (resp.status !== 'ok') {
            this.pushEvent(`Drop failed: ${resp.message}`);
            return;
        }
        await Promise.all([this.refreshLook(), this.refreshInventory()]);
    }

    async attack(target: string) {
        if (this.attacking) return;
        this.attacking = true;
        this.playerAnimation = ActorAnimation.Melee_1H_Attack_Slice_Diagonal;
        const resp = await this.tap.attack(target);
        setTimeout(() => {
            this.attacking = false;
            this.playerAnimation = ActorAnimation.Idle_A;
        }, 1000);
        if (resp.status !== 'ok') {
            this.pushEvent(`Attack failed: ${resp.message}`);
            return;
        }
        await this.refreshStatus();
        await this.refreshLook();
    }

    async talk(target: string) {
        const resp = await this.tap.talk(target);
        if (resp.status !== 'ok') {
            this.pushEvent(`Talk failed: ${resp.message}`);
            return;
        }
        const data = (resp as TapOk).data as Dialogue;
        this.dialogue = data;
    }

    closeDialogue() {
        this.dialogue = null;
    }

    async acceptQuest(id: string) {
        const resp = await this.tap.questAccept(id);
        if (resp.status !== 'ok') {
            this.pushEvent(`Accept failed: ${resp.message}`);
            return;
        }
        await Promise.all([this.refreshQuestList(), this.refreshQuestStatus()]);
    }

    async completeQuest(id: string) {
        const resp = await this.tap.questComplete(id);
        if (resp.status !== 'ok') {
            this.pushEvent(`Complete failed: ${resp.message}`);
            return;
        }
        await Promise.all([this.refreshQuestList(), this.refreshQuestStatus(), this.refreshStatus()]);
    }

    async sendChat(scope: ChatScope, text: string) {
        const trimmed = text.trim();
        if (!trimmed) return;
        await this.tap.chat(scope, trimmed);
    }
}

export const game = new GameStore();

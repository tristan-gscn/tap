export type TapOk = {
    status: 'ok';
    type: string;
    data?: unknown;
};

export type TapError = {
    status: 'error';
    code: number;
    message: string;
};

export type TapResponse = TapOk | TapError;

export type TapEventHandler = (event: TapOk) => void;

export type ChatScope = 'GLOBAL' | 'ROOM' | 'GROUP';
export type Direction = 'north' | 'south' | 'east' | 'west';

export class TAPManager {
    private url: string;
    private socket: WebSocket | null = null;
    private pending: Array<(resp: TapResponse) => void> = [];
    private eventHandlers: TapEventHandler[] = [];
    private buffer = '';
    private connectedName: string | null = null;

    constructor(url?: string) {
        const envUrl = (import.meta as { env?: Record<string, string> }).env?.VITE_TAP_BRIDGE_URL;
        this.url = url ?? envUrl ?? 'ws://localhost:7878';
    }

    connect(): Promise<void> {
        if (this.socket && this.socket.readyState === WebSocket.OPEN) {
            return Promise.resolve();
        }

        return new Promise((resolve, reject) => {
            const socket = new WebSocket(this.url);
            this.socket = socket;

            socket.addEventListener('open', () => resolve());
            socket.addEventListener('error', () => reject(new Error('bridge connection failed')));
            socket.addEventListener('message', (event) => this.onMessage(event.data));
            socket.addEventListener('close', () => {
                this.socket = null;
                this.pending.splice(0).forEach((p) => p({ status: 'error', code: 499, message: 'bridge closed' }));
            });
        });
    }

    onEvent(handler: TapEventHandler): () => void {
        this.eventHandlers.push(handler);
        return () => {
            this.eventHandlers = this.eventHandlers.filter((h) => h !== handler);
        };
    }

    private onMessage(data: unknown) {
        const chunk = typeof data === 'string' ? data : '';
        if (!chunk) {
            return;
        }
        this.buffer += chunk;
        let idx = this.buffer.indexOf('\n');
        while (idx >= 0) {
            const line = this.buffer.slice(0, idx).trim();
            this.buffer = this.buffer.slice(idx + 1);
            if (line.length > 0) {
                this.dispatchLine(line);
            }
            idx = this.buffer.indexOf('\n');
        }
    }

    private dispatchLine(line: string) {
        let parsed: TapResponse | null = null;
        try {
            parsed = JSON.parse(line) as TapResponse;
        } catch {
            return;
        }

        if (parsed.status === 'ok' && parsed.type === 'event') {
            this.eventHandlers.forEach((h) => h(parsed));
            return;
        }

        const resolver = this.pending.shift();
        if (resolver) {
            resolver(parsed);
        }
    }

    private sendRaw(line: string): Promise<TapResponse> {
        if (!this.socket || this.socket.readyState !== WebSocket.OPEN) {
            return Promise.resolve({ status: 'error', code: 499, message: 'not connected' });
        }

        return new Promise((resolve) => {
            this.pending.push(resolve);
            this.socket?.send(`${line}\n`);
        });
    }

    connectPlayer(name: string): Promise<TapResponse> {
        if (this.connectedName === name) {
            return Promise.resolve({ status: 'ok', type: 'connect', data: { name } });
        }
        return this.sendRaw(`CONNECT ${name}`).then((resp) => {
            if (resp.status === 'ok' && resp.type === 'connect') {
                this.connectedName = name;
            }
            return resp;
        });
    }

    who(): Promise<TapResponse> {
        return this.sendRaw('WHO');
    }

    look(): Promise<TapResponse> {
        return this.sendRaw('LOOK');
    }

    inventory(): Promise<TapResponse> {
        return this.sendRaw('INVENTORY');
    }

    move(direction: Direction): Promise<TapResponse> {
        return this.sendRaw(`MOVE ${direction}`);
    }

    go(direction: Direction): Promise<TapResponse> {
        return this.sendRaw(`GO ${direction}`);
    }

    take(item: string): Promise<TapResponse> {
        return this.sendRaw(`TAKE ${item}`);
    }

    drop(item: string): Promise<TapResponse> {
        return this.sendRaw(`DROP ${item}`);
    }

    attack(target: string): Promise<TapResponse> {
        return this.sendRaw(`ATTACK ${target}`);
    }

    talk(target: string): Promise<TapResponse> {
        return this.sendRaw(`TALK ${target}`);
    }

    chat(scope: ChatScope, text: string): Promise<TapResponse> {
        return this.sendRaw(`CHAT ${scope} ${text}`);
    }

    groupCreate(): Promise<TapResponse> {
        return this.sendRaw('GROUP CREATE');
    }

    groupInvite(target: string): Promise<TapResponse> {
        return this.sendRaw(`GROUP INVITE ${target}`);
    }

    groupJoin(leader: string): Promise<TapResponse> {
        return this.sendRaw(`GROUP JOIN ${leader}`);
    }

    groupLeave(): Promise<TapResponse> {
        return this.sendRaw('GROUP LEAVE');
    }

    quest(npc: string): Promise<TapResponse> {
        return this.sendRaw(`QUEST ${npc}`);
    }

    questList(): Promise<TapResponse> {
        return this.sendRaw('QUEST LIST');
    }

    questStatus(): Promise<TapResponse> {
        return this.sendRaw('QUEST STATUS');
    }

    questAccept(id: string): Promise<TapResponse> {
        return this.sendRaw(`QUEST ACCEPT ${id}`);
    }

    questComplete(id: string): Promise<TapResponse> {
        return this.sendRaw(`QUEST COMPLETE ${id}`);
    }

    quests(): Promise<TapResponse> {
        return this.sendRaw('QUESTS');
    }

    quit(): Promise<TapResponse> {
        return this.sendRaw('QUIT');
    }
}

export const tapClient = new TAPManager();

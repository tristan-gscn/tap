// Grid bounds for the dungeon room. The room is 38 x 18 (units), centered around (19, 9).
const X_MIN = 4;
const X_MAX = 34;
const Z_NPC_MIN = 2;
const Z_NPC_MAX = 8;
const Z_PLAYER_MIN = 10;
const Z_PLAYER_MAX = 16;

function hashString(s: string): number {
    let h = 2166136261;
    for (let i = 0; i < s.length; i++) {
        h ^= s.charCodeAt(i);
        h = Math.imul(h, 16777619);
    }
    return h >>> 0;
}

function clamp(v: number, lo: number, hi: number) {
    return Math.max(lo, Math.min(hi, v));
}

export function npcPosition(npcId: number, index: number, total: number): [number, number, number] {
    const span = X_MAX - X_MIN;
    const slot = total <= 1 ? 0.5 : index / (total - 1);
    const x = X_MIN + slot * span;
    const z = Z_NPC_MIN + ((npcId * 37) % 100) / 100 * (Z_NPC_MAX - Z_NPC_MIN);
    return [clamp(x, X_MIN, X_MAX), 0, z];
}

export function otherPlayerPosition(name: string, roomId: string): [number, number, number] {
    const h = hashString(`${roomId}:${name}`);
    const x = X_MIN + (h % 1000) / 1000 * (X_MAX - X_MIN);
    const z = Z_PLAYER_MIN + ((h >>> 10) % 1000) / 1000 * (Z_PLAYER_MAX - Z_PLAYER_MIN);
    return [clamp(x, X_MIN, X_MAX), 0, z];
}

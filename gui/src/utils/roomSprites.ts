import type { Point } from './roomLayout';

export type Sprite = { id: string; src: string };
export type SpritePositions = {
    enemy: Point[];
    npc: Point[];
    otherPlayers: Point[];
};

export const buildSprites = (
    ids: string[],
    assets: string[],
    basePath: string = '/characters/'
): Sprite[] =>
    ids.map((id, index) => ({
        id,
        src: `${basePath}${assets[index % assets.length]}`
    }));

export const splitPositions = (
    counts: { enemy: number; npc: number; otherPlayers: number },
    positions: Point[]
): SpritePositions => {
    const enemy = positions.slice(0, counts.enemy);
    const npc = positions.slice(counts.enemy, counts.enemy + counts.npc);
    const otherPlayers = positions.slice(counts.enemy + counts.npc);

    return { enemy, npc, otherPlayers };
};

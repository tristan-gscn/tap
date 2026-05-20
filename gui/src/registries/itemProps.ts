const PROP_MAP: Record<string, string> = {
    sword: '/models/props/adventurers/sword_1handed.gltf',
    shield: '/models/props/adventurers/shield_round.gltf',
};

export function resolveItemProp(itemId: string | null | undefined): string | null {
    if (!itemId) return null;
    return PROP_MAP[itemId] ?? null;
}

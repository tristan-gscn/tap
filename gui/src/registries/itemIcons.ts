import type { Component } from 'svelte';
import Bone from 'lucide-svelte/icons/bone';
import CircleHelp from 'lucide-svelte/icons/circle-question-mark';
import Coins from 'lucide-svelte/icons/coins';
import Flame from 'lucide-svelte/icons/flame';
import FlaskRound from 'lucide-svelte/icons/flask-round';
import Gem from 'lucide-svelte/icons/gem';
import Key from 'lucide-svelte/icons/key';
import Leaf from 'lucide-svelte/icons/leaf';
import Scroll from 'lucide-svelte/icons/scroll';
import Shield from 'lucide-svelte/icons/shield';
import Sword from 'lucide-svelte/icons/sword';

type IconComponent = Component<{ size?: number; strokeWidth?: number; class?: string }>;

const ICONS: Record<string, IconComponent> = {
    torch: Flame as unknown as IconComponent,
    flame: Flame as unknown as IconComponent,
    bone_dust: Bone as unknown as IconComponent,
    bone: Bone as unknown as IconComponent,
    ancient_coin: Coins as unknown as IconComponent,
    coin: Coins as unknown as IconComponent,
    coins: Coins as unknown as IconComponent,
    ale: FlaskRound as unknown as IconComponent,
    potion: FlaskRound as unknown as IconComponent,
    herbs: Leaf as unknown as IconComponent,
    lantern: Flame as unknown as IconComponent,
    gem: Gem as unknown as IconComponent,
    key: Key as unknown as IconComponent,
    scroll: Scroll as unknown as IconComponent,
    sword: Sword as unknown as IconComponent,
    shield: Shield as unknown as IconComponent,
};

export function resolveItemIcon(id: string | undefined | null): IconComponent {
    if (!id) return CircleHelp as unknown as IconComponent;
    return ICONS[id] ?? (CircleHelp as unknown as IconComponent);
}

export const FALLBACK_ITEM_ICON = CircleHelp as unknown as IconComponent;

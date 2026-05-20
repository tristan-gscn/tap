import type { Component } from 'svelte';
import type { ActorProps } from '../actors/ActorProps';
import BarbarianCharacter from '../actors/character/BarbarianCharacter_Actor.svelte';
import KnightCharacter from '../actors/character/KnightCharacter_Actor.svelte';
import MageCharacter from '../actors/character/MageCharacter_Actor.svelte';
import RangerCharacter from '../actors/character/RangerCharacter_Actor.svelte';
import RogueCharacter from '../actors/character/RogueCharacter_Actor.svelte';
import RogueHoodedCharacter from '../actors/character/RogueHoodedCharacter_Actor.svelte';

type ActorComponent = Component<ActorProps>;

export type CharacterClass = {
    id: string;
    label: string;
    description: string;
    component: ActorComponent;
};

export const CHARACTER_CLASSES: CharacterClass[] = [
    {
        id: 'knight',
        label: 'Knight',
        description: 'Steel and shield. Sturdy frontline.',
        component: KnightCharacter as unknown as ActorComponent,
    },
    {
        id: 'barbarian',
        label: 'Barbarian',
        description: 'Two-handed fury.',
        component: BarbarianCharacter as unknown as ActorComponent,
    },
    {
        id: 'mage',
        label: 'Mage',
        description: 'Spellcaster wielding arcane fire.',
        component: MageCharacter as unknown as ActorComponent,
    },
    {
        id: 'ranger',
        label: 'Ranger',
        description: 'Keen-eyed archer of the wilds.',
        component: RangerCharacter as unknown as ActorComponent,
    },
    {
        id: 'rogue',
        label: 'Rogue',
        description: 'Quick blades, quicker steps.',
        component: RogueCharacter as unknown as ActorComponent,
    },
    {
        id: 'rogue_hooded',
        label: 'Hooded Rogue',
        description: 'A face under shadow.',
        component: RogueHoodedCharacter as unknown as ActorComponent,
    },
];

const BY_ID: Record<string, ActorComponent> = Object.fromEntries(
    CHARACTER_CLASSES.map((c) => [c.id, c.component])
);

export function resolveCharacterModel(id: string | null | undefined): ActorComponent {
    if (id && BY_ID[id]) return BY_ID[id];
    return BY_ID.knight;
}

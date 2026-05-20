import type { Component } from 'svelte';
import type { ActorProps } from '../actors/ActorProps';
import MageSkeleton from '../actors/skeleton/MageSkeleton_Actor.svelte';
import MinionSkeleton from '../actors/skeleton/MinionSkeleton_Actor.svelte';
import RogueSkeleton from '../actors/skeleton/RogueSkeleton_Actor.svelte';
import WarriorSkeleton from '../actors/skeleton/WarriorSkeleton_Actor.svelte';

type ActorComponent = Component<ActorProps>;

const NPC_MODELS: Record<string, ActorComponent> = {
    warrior: WarriorSkeleton as unknown as ActorComponent,
    rogue: RogueSkeleton as unknown as ActorComponent,
    mage: MageSkeleton as unknown as ActorComponent,
    minion: MinionSkeleton as unknown as ActorComponent,
};

export function resolveNpcModel(model: string | null | undefined): ActorComponent {
    if (model && NPC_MODELS[model]) return NPC_MODELS[model];
    return NPC_MODELS.warrior;
}

export function registerNpcModel(model: string, component: ActorComponent): void {
    NPC_MODELS[model] = component;
}

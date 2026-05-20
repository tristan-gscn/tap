import type { Component } from 'svelte';
import Dungeon from '../room/dungeon.svelte';

export type RoomRenderProps = {
    roomId: string;
};

const ROOMS: Record<string, Component<RoomRenderProps>> = {
    dungeon: Dungeon as unknown as Component<RoomRenderProps>,
};

export function resolveRoom(kind: string | undefined): Component<RoomRenderProps> {
    if (kind && ROOMS[kind]) return ROOMS[kind];
    return ROOMS.dungeon;
}

export function registerRoom(kind: string, component: Component<RoomRenderProps>): void {
    ROOMS[kind] = component;
}

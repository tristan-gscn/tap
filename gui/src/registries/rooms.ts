import type { Component } from 'svelte';
import type { Direction } from '../utils/TAPManager';
import Dungeon from '../room/dungeon.svelte';

export type RoomRenderProps = {
    roomId?: string;
    onDoorClick?: (direction: Direction) => void;
    onFloorClick?: (position: [number, number, number]) => void;
    availableExits?: Direction[];
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

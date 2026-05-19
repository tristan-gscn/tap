<script lang="ts">
    import type { Room } from '../../models/Room';
    import { createEventDispatcher, onMount } from 'svelte';
    import ChatBox from '../../components/ChatBox.svelte';
    import { tapClient, type TapOk, type Direction } from '../../utils/TAPManager';
    import AdventureView from './AdventureView.svelte';
    import CharacterView from './CharacterView.svelte';
    import SocialView from './SocialView.svelte';

    let activeTab: 'adventure' | 'character' | 'social' = $state('adventure');

    let room: Room = $state({
        room: {
            id: '',
            name: '',
            description: '',
            exits: {
                north: '',
                south: '',
                east: '',
                west: ''
            }
        },
        players: [],
        items: [],
        npcs: []
    });

    const tap = tapClient;
    const dispatch = createEventDispatcher<{ quit: void }>();

    const resolveName = () => {
        const stored = localStorage.getItem('tap-player-name');
        if (stored && stored.trim().length > 0) {
            return stored;
        }
        return 'Unknown';
    };

    const applyLook = (data: unknown) => {
        const payload = data as {
            room?: { id?: string; name?: string; description?: string; exits?: Record<string, string> };
            players?: string[];
            items?: string[];
            npcs?: Array<{ type?: string } | string>;
        };
        room = {
            room: {
                id: payload.room?.id ?? '',
                name: payload.room?.name ?? '',
                description: payload.room?.description ?? '',
                exits: {
                    north: payload.room?.exits?.north ?? '',
                    south: payload.room?.exits?.south ?? '',
                    east: payload.room?.exits?.east ?? '',
                    west: payload.room?.exits?.west ?? ''
                }
            },
            players: payload.players ?? [],
            items: payload.items ?? [],
            npcs: (payload.npcs ?? []).map((n) => {
                if (typeof n === 'string') {
                    return n;
                }
                return n.type ?? '?';
            })
        };
    };

    const handleRoomEvent = (data: Record<string, unknown>) => {
        const event = data.event;
        if (event === 'presence_enter') {
            const name = data.name as string | undefined;
            if (name && !room.players.includes(name)) {
                room = { ...room, players: [...room.players, name] };
            }
        }
        if (event === 'presence_leave') {
            const name = data.name as string | undefined;
            if (name) {
                room = { ...room, players: room.players.filter((p) => p !== name) };
            }
        }
        if (event === 'item_taken') {
            const item = data.item as string | undefined;
            if (item) {
                room = { ...room, items: room.items.filter((i) => i !== item) };
            }
        }
        if (event === 'item_dropped') {
            const item = data.item as string | undefined;
            if (item && !room.items.includes(item)) {
                room = { ...room, items: [...room.items, item] };
            }
        }
        if (event === 'npc_killed') {
            const npc = data.npc as string | undefined;
            if (npc) {
                room = { ...room, npcs: room.npcs.filter((n) => n !== npc) };
            }
        }
    };

    const moveTo = async (direction: Direction) => {
        const resp = await tap.move(direction);
        if (resp.status === 'ok') {
            const look = await tap.look();
            if (look.status === 'ok' && look.type === 'look') {
                applyLook(look.data);
            }
        }
    };

    const handleQuit = async () => {
        localStorage.removeItem('tap-player-name');
        await tap.quit();
        dispatch('quit');
    };

    onMount(() => {
        let unsubscribe: (() => void) | null = null;

        const init = async () => {
            await tap.connect();
            await tap.connectPlayer(resolveName());
            const resp = await tap.look();
            if (resp.status === 'ok' && resp.type === 'look') {
                applyLook(resp.data);
            }
            unsubscribe = tap.onEvent((evt: TapOk) => {
                const data = (evt.data ?? {}) as Record<string, unknown>;
                if (!data.event) {
                    return;
                }
                handleRoomEvent(data);
            });
        };

        void init();

        return () => {
            unsubscribe?.();
        };
    });
</script>

<div class="w-screen h-screen bg-black flex items-center justify-center">
    <div class="relative w-full h-full">
        <ChatBox />
        
        {#if activeTab === 'adventure'}
            <AdventureView {room} {moveTo} />
        {:else if activeTab === 'character'}
            <CharacterView />
        {:else if activeTab === 'social'}
            <SocialView />
        {/if}

        <div class="absolute bottom-6 left-6 z-10 flex items-center gap-3">
            <button
                class="text-white text-2xl px-4 py-2 cursor-pointer transition-all {activeTab === 'adventure' ? 'bg-purple-500' : 'bg-gray-500 hover:bg-purple-500'}"
                onclick={() => activeTab = 'adventure'}
            >
                ADVENTURE
            </button>
            <button
                class="text-white text-2xl px-4 py-2 cursor-pointer transition-all {activeTab === 'character' ? 'bg-purple-500' : 'bg-gray-500 hover:bg-purple-500'}"
                onclick={() => activeTab = 'character'}
            >
                CHARACTER
            </button>
            <button
                class="text-white text-2xl px-4 py-2 cursor-pointer transition-all {activeTab === 'social' ? 'bg-purple-500' : 'bg-gray-500 hover:bg-purple-500'}"
                onclick={() => activeTab = 'social'}
            >
                SOCIAL
            </button>
        </div>
        <div class="absolute bottom-6 right-6 z-10">
            <button
                class="bg-red-500 hover:bg-red-600 text-white text-2xl px-4 py-2 cursor-pointer"
                onclick={handleQuit}
            >
                QUIT
            </button>
        </div>
    </div>
</div>
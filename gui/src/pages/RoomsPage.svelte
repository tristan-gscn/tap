<script lang="ts">
    import type { Room } from '../models/Room';
    import { createEventDispatcher, onMount } from 'svelte';
    import { placeInCircle } from '../utils/roomLayout';
    import { buildSprites, splitPositions } from '../utils/roomSprites';
    import ChatBox from '../components/ChatBox.svelte';
    import { tapClient, type TapOk, type Direction } from '../utils/TAPManager';

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
        const name = `gui-${Math.floor(Math.random() * 9000 + 1000)}`;
        localStorage.setItem('tap-player-name', name);
        return name;
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

    const enemyAssets = ['npc_en1.png'];
    const npcAssets = ['npc1.png', 'npc2.png'];

    let enemies = $state<string[]>([]);

    let enemySprites = $derived(buildSprites(enemies, enemyAssets));
    let npcSprites = $derived(buildSprites(room.npcs, npcAssets));

    let otherPlayers = $derived(room.players.slice(1));
    let otherPlayerSprites = $derived(
        buildSprites(otherPlayers, ['player_2.png'])
    );

    // Calculate all positions together so enemies and NPCs don't overlap
    let combinedPositions = $derived(
        placeInCircle(enemySprites.length + npcSprites.length + otherPlayerSprites.length, 32)
    );
    let spritePositions = $derived(
        splitPositions(
            {
                enemy: enemySprites.length,
                npc: npcSprites.length,
                otherPlayers: otherPlayerSprites.length
            },
            combinedPositions
        )
    );
</script>

<div class="w-screen h-screen bg-black flex items-center justify-center">
    <div class="relative w-full h-full">
        <ChatBox />
        <!-- Main Room Block -->
        <div
            class="absolute left-1/2 top-1/2 h-[80%] w-[80%] -translate-x-1/2 -translate-y-1/2 border-5 border-white relative"
        >
            <!-- svelte-ignore a11y_label_has_associated_control -->
            <h1 class="text-white text-xl absolute right-4 top-2 select-none">
                {room.room.name} <label class="text-purple-500">({room.room.id})</label>
            </h1>
            <img
                    src="/characters/player_1.png"
                    alt="Player Character"
                    class="w-28 h-28 absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 cursor-pointer hover:scale-110 transition-all"
            />

            <div class="absolute inset-0 pointer-events-none">
                {#each enemySprites as enemy, index (`${enemy.id}-${index}`)}
                    <img
                        src={enemy.src}
                        alt="Enemy"
                        class="absolute w-28 h-28 -translate-x-1/2 -translate-y-1/2 origin-center cursor-pointer pointer-events-auto transition-transform hover:scale-110"
                        style={`left: ${spritePositions.enemy[index]?.x ?? 50}%; top: ${spritePositions.enemy[index]?.y ?? 50}%;`}
                    />
                {/each}
                {#each npcSprites as npc, index (`${npc.id}-${index}`)}
                    <img
                        src={npc.src}
                        alt="NPC"
                        class="absolute w-28 h-28 -translate-x-1/2 -translate-y-1/2 origin-center cursor-pointer pointer-events-auto transition-transform hover:scale-110"
                        style={`left: ${spritePositions.npc[index]?.x ?? 50}%; top: ${spritePositions.npc[index]?.y ?? 50}%;`}
                    />
                {/each}
                {#each otherPlayerSprites as player, index (`${player.id}-${index}`)}
                    <img
                        src={player.src}
                        alt="Other Player"
                        class="absolute w-28 h-28 -translate-x-1/2 -translate-y-1/2 origin-center cursor-pointer pointer-events-auto transition-transform hover:scale-110"
                        style={`left: ${spritePositions.otherPlayers[index]?.x ?? 50}%; top: ${spritePositions.otherPlayers[index]?.y ?? 50}%;`}
                    />
                {/each}
            </div>
        </div>

        {#if room.room.exits.north}
            <div
                class="absolute left-1/2 top-0 h-[11%] w-60 -translate-x-1/2 bg-black border-l-[5px] border-r-[5px] border-white flex items-center justify-center"
            >
                <button
                    class="text-white transition-all hover:text-yellow-500 text-3xl cursor-pointer"
                    onclick={() => moveTo('north')}
                >
                    {room.room.exits.north}
                </button>
            </div>
        {/if}

        {#if room.room.exits.south}
            <div
                class="absolute left-1/2 bottom-0 h-[11%] w-60 -translate-x-1/2 bg-black border-l-[5px] border-r-[5px] border-white flex items-center justify-center"
            >
                <button
                    class="text-white transition-all hover:text-yellow-500 text-3xl cursor-pointer"
                    onclick={() => moveTo('south')}
                >
                    {room.room.exits.south}
                </button>
            </div>
        {/if}

        {#if room.room.exits.east}
            <div
                class="absolute right-0 top-1/2 h-60 w-[11%] -translate-y-1/2 bg-black border-t-[5px] border-b-[5px] border-white flex items-center justify-center"
            >
                <button
                    class="text-white transition-all hover:text-yellow-500 text-3xl cursor-pointer"
                    onclick={() => moveTo('east')}
                >
                    {room.room.exits.east}
                </button>
            </div>
        {/if}

        {#if room.room.exits.west}
            <div
                class="absolute left-0 top-1/2 h-60 w-[11%] -translate-y-1/2 bg-black border-t-[5px] border-b-[5px] border-white flex items-center justify-center"
            >
                <button
                    class="text-white transition-all hover:text-yellow-500 text-3xl cursor-pointer"
                    onclick={() => moveTo('west')}
                >
                    {room.room.exits.west}
                </button>
            </div>
        {/if}

        <div class="absolute bottom-6 left-6 z-10 flex items-center gap-3">
            <button
                class="bg-purple-500  text-white text-2xl px-4 py-2"
            >
                ADVENTURE
            </button>
            <button
                class="bg-gray-500 hover:bg-purple-500 text-white text-2xl px-4 py-2 cursor-pointer"
            >
                CHARACTER
            </button>
            <button
                class="bg-gray-500 hover:bg-purple-500 text-white text-2xl px-4 py-2 cursor-pointer"
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
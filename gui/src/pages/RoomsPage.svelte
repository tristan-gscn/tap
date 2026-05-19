<script lang="ts">
    import type { Room } from '../models/Room';
    import { placeInCircle } from '../utils/roomLayout';
    import { buildSprites, splitPositions } from '../utils/roomSprites';
    import ChatBox from '../components/ChatBox.svelte';

    // TODO: connect backend to replace fake values
    let room: Room = $state({
        room: {
            id: 'room.identifier',
            name: 'Room Display Name',
            description: 'Room description text',
            exits: {
                north: 'room.north_id',
                south: '',
                east: 'x',
                west: ''
            }
        },
        players: ['username1', 'username2'],
        items: ['item.id1', 'item.id2'],
        npcs: ['npc.id1', 'npc.id2']
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
                {#each enemySprites as enemy, index (enemy.id)}
                    <img
                        src={enemy.src}
                        alt="Enemy"
                        class="absolute w-28 h-28 -translate-x-1/2 -translate-y-1/2 origin-center cursor-pointer pointer-events-auto transition-transform hover:scale-110"
                        style={`left: ${spritePositions.enemy[index]?.x ?? 50}%; top: ${spritePositions.enemy[index]?.y ?? 50}%;`}
                    />
                {/each}
                {#each npcSprites as npc, index (npc.id)}
                    <img
                        src={npc.src}
                        alt="NPC"
                        class="absolute w-28 h-28 -translate-x-1/2 -translate-y-1/2 origin-center cursor-pointer pointer-events-auto transition-transform hover:scale-110"
                        style={`left: ${spritePositions.npc[index]?.x ?? 50}%; top: ${spritePositions.npc[index]?.y ?? 50}%;`}
                    />
                {/each}
                {#each otherPlayerSprites as player, index (player.id)}
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
                class="absolute left-1/2 top-0 h-[11%] w-60 -translate-x-1/2 bg-black border-l-[5px] border-r-[5px] border-white"
            ></div>
        {/if}

        {#if room.room.exits.south}
            <div
                class="absolute left-1/2 bottom-0 h-[11%] w-60 -translate-x-1/2 bg-black border-l-[5px] border-r-[5px] border-white"
            ></div>
        {/if}

        {#if room.room.exits.east}
            <div
                class="absolute right-0 top-1/2 h-60 w-[11%] -translate-y-1/2 bg-black border-t-[5px] border-b-[5px] border-white"
            ></div>
        {/if}

        {#if room.room.exits.west}
            <div
                class="absolute left-0 top-1/2 h-60 w-[11%] -translate-y-1/2 bg-black border-t-[5px] border-b-[5px] border-white"
            ></div>
        {/if}

        <div class="absolute bottom-6 left-6 z-10 flex gap-3">
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
    </div>
</div>
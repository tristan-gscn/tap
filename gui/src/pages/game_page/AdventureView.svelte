<script lang="ts">
    import type { Room } from '../../models/Room';
    import { placeInCircle } from '../../utils/roomLayout';
    import { buildSprites, splitPositions } from '../../utils/roomSprites';
    import type { Direction } from '../../utils/TAPManager';

    interface Props {
        room: Room;
        moveTo: (direction: Direction) => Promise<void>;
    }

    let { room, moveTo }: Props = $props();

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

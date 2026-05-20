<script lang="ts">
    import { T } from '@threlte/core';
    import { interactivity } from '@threlte/extras';
    import PlayerActor from './actors/PlayerActor.svelte';
    import { ActorAnimation } from './actors/ActorAnimation';
    import { resolveCharacterModel } from './registries/characterModels';
    import { resolveNpcModel } from './registries/npcModels';
    import { resolveRoom } from './registries/rooms';
    import { game } from './state/game.svelte';
    import type { Direction } from './utils/TAPManager';
    import { npcPosition, otherPlayerPosition } from './utils/positions';

    interactivity();

    const RoomComp = $derived(resolveRoom(game.room?.kind));

    const availableExits = $derived(
        Object.keys(game.room?.exits ?? {}) as Direction[]
    );

    const otherPlayers = $derived(
        game.players.filter((p) => p !== game.playerName)
    );

    function handleFloorClick(pos: [number, number, number]) {
        const x = Math.max(2, Math.min(36, pos[0]));
        const z = Math.max(2, Math.min(16, pos[2]));
        game.setMoveTarget([x, 0, z]);
    }

    function handleDoorClick(direction: Direction) {
        game.move(direction);
    }
</script>

<RoomComp
    onDoorClick={handleDoorClick}
    onFloorClick={handleFloorClick}
    availableExits={availableExits}
/>

{#if game.connected}
    <PlayerActor />
{/if}

{#each game.npcs as npc, i (npc.id)}
    {@const NpcComp = resolveNpcModel(npc.model)}
    {@const pos = npcPosition(npc.id, i, game.npcs.length)}
    <T.Group position={pos} rotation={[0, Math.PI, 0]}>
        <NpcComp animation={ActorAnimation.Skeletons_Idle} />
    </T.Group>
{/each}

{#each otherPlayers as name (name)}
    {@const detail = game.playersDetail.find((p) => p.name === name)}
    {@const OtherComp = resolveCharacterModel(detail?.class ?? null)}
    {@const pos = otherPlayerPosition(name, game.room?.id ?? '')}
    <T.Group position={pos} rotation={[0, Math.PI, 0]}>
        <OtherComp animation={ActorAnimation.Idle_A} />
    </T.Group>
{/each}

<script lang="ts">
    import { T } from '@threlte/core';
    import { interactivity, HTML } from '@threlte/extras';
    import PlayerActor from './actors/PlayerActor.svelte';
    import { ActorAnimation } from './actors/ActorAnimation';
    import { resolveCharacterModel } from './registries/characterModels';
    import { resolveNpcModel } from './registries/npcModels';
    import { resolveRoom } from './registries/rooms';
    import { game } from './state/game.svelte';
    import type { Direction } from './utils/TAPManager';
    import { npcPosition, otherPlayerPosition } from './utils/positions';
    import type { IntersectionEvent } from '@threlte/extras';

    interactivity();

    const RoomComp = $derived(resolveRoom(game.room?.kind));

    const availableExits = $derived(
        Object.keys(game.room?.exits ?? {}) as Direction[]
    );

    const otherPlayers = $derived(
        game.players.filter((p) => p !== game.playerName)
    );

    let hoveredNpcId = $state<number | null>(null);
    let npcAnimations = $state<Record<number, ActorAnimation>>({});
    const npcTimers = new Map<number, ReturnType<typeof setTimeout>>();

    function randomSpawnAnimation() {
        return Math.random() < 0.5 ? ActorAnimation.Spawn_Air : ActorAnimation.Spawn_Ground;
    }

    function setNpcAnimation(id: number, animation: ActorAnimation) {
        npcAnimations = { ...npcAnimations, [id]: animation };
    }

    $effect(() => {
        const ids = new Set(game.npcs.map((n) => n.id));
        for (const npc of game.npcs) {
            if (npcAnimations[npc.id]) continue;
            const anim = randomSpawnAnimation();
            setNpcAnimation(npc.id, anim);
            const timer = setTimeout(() => {
                setNpcAnimation(npc.id, ActorAnimation.Skeletons_Idle);
            }, 1100);
            npcTimers.set(npc.id, timer);
        }
        for (const key of Object.keys(npcAnimations)) {
            const id = Number(key);
            if (!ids.has(id)) {
                const timer = npcTimers.get(id);
                if (timer) {
                    clearTimeout(timer);
                    npcTimers.delete(id);
                }
                const { [id]: _, ...rest } = npcAnimations;
                npcAnimations = rest;
            }
        }
    });

    function handleFloorClick(pos: [number, number, number]) {
        const x = Math.max(2, Math.min(36, pos[0]));
        const z = Math.max(2, Math.min(16, pos[2]));
        game.setMoveTarget([x, 0, z]);
    }

    function handleDoorClick(direction: Direction) {
        game.move(direction);
    }

    function handleNpcPointer(
        e: IntersectionEvent<PointerEvent>,
        npcType: string,
        npcId: number,
        pos: [number, number, number]
    ) {
        e.stopPropagation();
        const button = (e as unknown as { event: PointerEvent }).event?.button ?? 0;
        if (button === 2) {
            game.talkAndQuest(npcType);
            return;
        }
        game.requestAttack(npcId, npcType, pos);
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
    {@const hpPct = npc.max_hp > 0 ? Math.max(0, (npc.hp / npc.max_hp) * 100) : 0}
    <T.Group
        position={pos}
        rotation={[0, Math.PI, 0]}
        onpointerdown={(e: IntersectionEvent<PointerEvent>) => handleNpcPointer(e, npc.type, npc.id, pos)}
        onpointerenter={() => (hoveredNpcId = npc.id)}
        onpointerleave={() => hoveredNpcId === npc.id && (hoveredNpcId = null)}
        oncontextmenu={(e: Event) => e.preventDefault()}
    >
        <NpcComp animation={npcAnimations[npc.id] ?? ActorAnimation.Skeletons_Idle} />
        {#if hoveredNpcId === npc.id}
            <T.Group position={[0, 4.0, 0]}>
                <HTML center>
                    <div class="pointer-events-none rounded-md border border-white/30 bg-black/70 px-2 py-1 text-[10.5px] text-white shadow-lg">
                        <div class="flex items-center justify-between gap-2">
                            <span class="font-medium">{npc.name}</span>
                            <span class="text-white/70">{npc.hp}/{npc.max_hp}</span>
                        </div>
                        <div class="mt-1 h-1.5 w-28 rounded-full bg-white/10">
                            <div
                                class="h-1.5 rounded-full {hpPct < 30 ? 'bg-rose-400' : hpPct < 60 ? 'bg-amber-300' : 'bg-emerald-300'}"
                                style="width: {hpPct}%"
                            ></div>
                        </div>
                    </div>
                </HTML>
            </T.Group>
        {/if}
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

<script lang="ts">
    import { Canvas } from '@threlte/core';
    import Scene from './Scene.svelte';
    import CharacterSelect from './ui/CharacterSelect.svelte';
    import HUD from './ui/HUD.svelte';
    import { game } from './state/game.svelte';
</script>

<div class="relative h-screen w-screen overflow-hidden bg-[#0a0a0e]">
    <Canvas>
        <Scene />
    </Canvas>

    {#if game.connected}
        <div class="pointer-events-none absolute left-4 top-4">
            <h1 class="font-upheaval text-2xl tracking-[0.18em] text-white drop-shadow-[0_2px_3px_rgba(0,0,0,0.8)]">
                The Answer Protocol
            </h1>
            {#if game.room}
                <div class="text-[10px] uppercase tracking-[0.15em] text-white/70">
                    salle · {game.room.id}
                </div>
            {/if}
        </div>
        <HUD />
    {:else}
        <CharacterSelect />
    {/if}
</div>

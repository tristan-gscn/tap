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
        <HUD />
    {:else}
        <CharacterSelect />
    {/if}
</div>

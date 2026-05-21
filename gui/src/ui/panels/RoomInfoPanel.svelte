<script lang="ts">
    import { game } from '../../state/game.svelte';
</script>

<div class="mt-4 select-none">
    {#if game.room}
        <div class="tap-panel-title text-center">
            <span class="w-full text-2xl">{game.room.name}</span>
        </div>
        <p class="text-[12px] leading-relaxed text-white/75 relative bottom-2">{game.room.description}</p>
    {:else}
        <div class="text-[12px] text-white/55">No room loaded.</div>
    {/if}
</div>

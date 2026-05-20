<script lang="ts">
    import { game } from '../../state/game.svelte';
</script>

<div class="tap-panel">
    {#if game.room}
        <div class="tap-panel-title">
            <span>{game.room.name}</span>
            <span class="text-[10.5px] font-normal text-white/45">{game.room.kind}</span>
        </div>
        <p class="text-[12px] leading-relaxed text-white/75">{game.room.description}</p>
        {#if game.players.length > 1}
            <div class="mt-2.5 text-[11px] text-white/50">
                Also here: <span class="text-white/85">{game.players.filter((p) => p !== game.playerName).join(', ')}</span>
            </div>
        {/if}
    {:else}
        <div class="text-[12px] text-white/55">No room loaded.</div>
    {/if}
</div>

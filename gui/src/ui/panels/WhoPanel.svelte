<script lang="ts">
    import LiquidGlassPanel from '../LiquidGlassPanel.svelte';
    import { game } from '../../state/game.svelte';
</script>

<LiquidGlassPanel>
    <div class="mb-2 flex items-center justify-between text-[0.82rem] font-medium text-white/90">
        <span>Players</span>
    </div>
    {#if game.onlinePlayers.length === 0}
        <div class="text-[12px] text-white/55">No one online.</div>
    {:else}
        <ul class="space-y-1">
            {#each game.onlinePlayers as p}
                <li class="rounded-lg border border-white/10 bg-white/5 px-2.5 py-1.5 text-[12px]">
                    <span class={p === game.playerName ? 'font-medium text-white' : 'text-white/80'}>{p}</span>
                </li>
            {/each}
        </ul>
    {/if}
</LiquidGlassPanel>

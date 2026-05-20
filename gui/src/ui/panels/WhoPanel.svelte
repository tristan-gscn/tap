<script lang="ts">
    import RefreshCw from 'lucide-svelte/icons/refresh-cw';
    import { game } from '../../state/game.svelte';
</script>

<div class="tap-panel">
    <div class="tap-panel-title">
        <span>Joueurs</span>
        <button
            type="button"
            class="tap-btn px-1.5 py-0.5"
            onclick={() => game.refreshWho()}
            aria-label="rafraîchir"
        >
            <RefreshCw size={11} />
        </button>
    </div>
    {#if game.onlinePlayers.length === 0}
        <div class="text-[12px] text-white/55">Personne en ligne.</div>
    {:else}
        <ul class="space-y-1">
            {#each game.onlinePlayers as p}
                <li class="rounded-lg border border-white/10 bg-white/5 px-2.5 py-1.5 text-[12px]">
                    <span class={p === game.playerName ? 'font-medium text-white' : 'text-white/80'}>{p}</span>
                </li>
            {/each}
        </ul>
    {/if}
</div>

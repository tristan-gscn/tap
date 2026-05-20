<script lang="ts">
    import { game } from '../../state/game.svelte';
</script>

<div class="tap-panel">
    <div class="tap-panel-title">
        <span>Ennemis</span>
        <span class="text-[11px] font-normal text-white/55">{game.npcs.length}</span>
    </div>
    {#if game.npcs.length === 0}
        <div class="text-[12px] text-white/55">La salle est calme.</div>
    {:else}
        <ul class="space-y-2">
            {#each game.npcs as npc (npc.id)}
                {@const pct = npc.max_hp > 0 ? Math.max(0, (npc.hp / npc.max_hp) * 100) : 0}
                <li class="rounded-lg border border-white/10 bg-white/5 p-2.5">
                    <div class="mb-1.5 flex items-baseline justify-between">
                        <span class="text-[12.5px] font-medium text-white">{npc.name}</span>
                        <span class="text-[10.5px] text-white/55">{npc.hp}/{npc.max_hp}</span>
                    </div>
                    <div class="tap-bar-bg h-1.5 w-full">
                        <div class="tap-bar-fill {pct < 30 ? 'hp-low' : pct < 60 ? 'hp-mid' : ''}" style="width: {pct}%"></div>
                    </div>
                    <div class="mt-2.5 flex gap-1.5">
                        <button
                            type="button"
                            class="tap-btn px-2 py-0.5 text-[10.5px]"
                            onclick={() => game.talk(npc.type)}
                        >parler</button>
                        <button
                            type="button"
                            class="tap-btn tap-btn-danger px-2 py-0.5 text-[10.5px]"
                            onclick={() => game.attack(npc.type)}
                            disabled={game.attacking}
                        >attaquer</button>
                    </div>
                </li>
            {/each}
        </ul>
    {/if}
</div>

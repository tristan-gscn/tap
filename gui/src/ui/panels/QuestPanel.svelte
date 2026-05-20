<script lang="ts">
    import RefreshCw from 'lucide-svelte/icons/refresh-cw';
    import { game } from '../../state/game.svelte';
</script>

<div class="tap-panel">
    <div class="tap-panel-title">
        <span>Quests</span>
        <button
            type="button"
            class="tap-btn px-1.5 py-0.5"
            onclick={() => {
                game.refreshQuestList();
                game.refreshQuestStatus();
            }}
            aria-label="refresh"
        >
            <RefreshCw size={11} />
        </button>
    </div>

    {#if game.questsAll.length === 0}
        <div class="text-[12px] text-white/55">No known quests.</div>
    {:else}
        <ul class="max-h-72 space-y-2 overflow-y-auto pr-1">
            {#each game.questsAll as q (q.id)}
                {@const active = game.questsActive.find((a) => a.id === q.id)}
                <li class="rounded-lg border border-white/10 bg-white/5 p-2.5">
                    <div class="flex items-baseline justify-between gap-2">
                        <span class="text-[12.5px] font-medium text-white">{q.name}</span>
                        <span
                            class="shrink-0 text-[10px] {q.status === 'completed'
                                ? 'text-emerald-300'
                                : q.status === 'active'
                                  ? 'text-white/80'
                                  : 'text-white/45'}"
                        >
                            {q.status === 'completed' ? 'completed' : q.status === 'active' ? 'active' : 'available'}
                        </span>
                    </div>
                    <div class="mt-1 text-[11.5px] leading-snug text-white/70">{q.description}</div>
                    <div class="mt-1.5 text-[10.5px] text-white/45">
                        {q.objective.kind} · {q.objective.target} ×{q.objective.count} · {q.reward.xp} XP
                    </div>
                    {#if active}
                        <div class="mt-1.5 text-[11px] text-white/85">
                            {active.progress}/{active.required}
                        </div>
                    {/if}
                    {#if q.status === 'available' || (active && active.progress >= active.required && !active.completed)}
                        <div class="mt-2 flex gap-1.5">
                            {#if q.status === 'available'}
                                <button
                                    type="button"
                                    class="tap-btn tap-btn-primary px-2 py-0.5 text-[10.5px]"
                                    onclick={() => game.acceptQuest(q.id)}>accept</button
                                >
                            {/if}
                            {#if active && active.progress >= active.required && !active.completed}
                                <button
                                    type="button"
                                    class="tap-btn tap-btn-accept px-2 py-0.5 text-[10.5px]"
                                    onclick={() => game.completeQuest(q.id)}>complete</button
                                >
                            {/if}
                        </div>
                    {/if}
                </li>
            {/each}
        </ul>
    {/if}
</div>

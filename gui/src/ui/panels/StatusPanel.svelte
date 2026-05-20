<script lang="ts">
    import { game } from '../../state/game.svelte';

    const statusColor = $derived(
        game.status === 'dead'
            ? 'text-red-300'
            : game.status === 'critical'
              ? 'text-orange-300'
              : game.status === 'wounded'
                ? 'text-white/70'
                : 'text-emerald-300'
    );
    const hpPct = $derived(game.maxHp > 0 ? Math.max(0, Math.min(100, (game.hp / game.maxHp) * 100)) : 0);
    const hpClass = $derived(hpPct < 25 ? 'hp-low' : hpPct < 60 ? 'hp-mid' : '');
</script>

<div class="tap-panel">
    <div class="mb-2 flex items-baseline justify-between">
        <div class="text-base font-medium text-white">{game.playerName || 'Anon'}</div>
        <div class="text-[11px] text-white/50">{game.playerClass}</div>
    </div>
    <div class="mb-1.5 flex items-center justify-between text-[11.5px]">
        <span class="text-white/85">{game.hp} / {game.maxHp}</span>
        <span class={statusColor}>{game.status}</span>
    </div>
    <div class="tap-bar-bg h-2.5 w-full">
        <div class="tap-bar-fill {hpClass}" style="width: {hpPct}%"></div>
    </div>
    <div class="mt-2 text-[11px] text-white/55">{game.xp} XP</div>
</div>

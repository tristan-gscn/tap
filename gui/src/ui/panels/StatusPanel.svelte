<script lang="ts">
    import { Canvas } from '@threlte/core';
    import StatusPreview from '../StatusPreview.svelte';
    import { game } from '../../state/game.svelte';

    const hpPct = $derived(game.maxHp > 0 ? Math.max(0, Math.min(100, (game.hp / game.maxHp) * 100)) : 0);
    const hpFillClass = $derived(
        hpPct < 25
            ? 'bg-[linear-gradient(180deg,rgba(255,110,110,0.95)_0%,rgba(180,40,40,0.75)_100%)] shadow-[0_1px_0_rgba(255,255,255,0.3)_inset,0_0_12px_rgba(255,80,80,0.4)]'
            : hpPct < 60
              ? 'bg-[linear-gradient(180deg,rgba(255,200,110,0.95)_0%,rgba(200,130,40,0.75)_100%)] shadow-[0_1px_0_rgba(255,255,255,0.3)_inset,0_0_12px_rgba(255,180,80,0.4)]'
              : 'bg-[linear-gradient(180deg,rgba(150,240,130,0.95)_0%,rgba(60,170,70,0.75)_100%)] shadow-[0_1px_0_rgba(255,255,255,0.3)_inset,0_0_12px_rgba(110,220,90,0.4)]'
    );
    const hpWidth = $derived(hpPct >= 99.5 ? 100 : hpPct);
</script>

<div class="flex items-center gap-3 m-2">
    <div class="relative z-[2] h-[72px] w-[72px] rotate-45 overflow-hidden rounded-[14px] border border-white/20 bg-[#050508] shadow-[0_1px_0_rgba(255,255,255,0.08)_inset,0_6px_18px_rgba(0,0,0,0.35)]">
        <div class="h-full w-full origin-center -rotate-45 scale-[1.18] bg-[#050508]">
            <Canvas class="h-full w-full">
                <StatusPreview classId={game.playerClass || 'knight'} />
            </Canvas>
        </div>
    </div>

    <div class="flex min-w-0 flex-1 flex-col gap-1.5">
        <div class="text-[12px] font-medium text-white select-none">
            {game.playerName || 'Anon'}
            <span class="text-white/60">· {game.xp} XP</span>
        </div>
        <div class="relative z-[1] -ml-[36px] h-3.5 w-full overflow-hidden rounded-full border border-white/10 bg-black/40 pl-5 shadow-[0_1px_2px_rgba(0,0,0,0.4)_inset]">
            <div class={`h-full ${hpFillClass}`} style="width: {hpWidth}%"></div>
        </div>
    </div>
</div>

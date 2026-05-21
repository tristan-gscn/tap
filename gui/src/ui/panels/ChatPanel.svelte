<script lang="ts">
    import { game } from '../../state/game.svelte';
    import type { ChatScope } from '../../utils/TAPManager';

    type LogItem =
        | { kind: 'event'; ts: number; text: string }
        | { kind: 'chat'; ts: number; scope: string; from: string; text: string };

    let scope = $state<ChatScope>('ROOM');
    let text = $state('');

    const MAX_VISIBLE = 8;

    const logItems = $derived<LogItem[]>(
        [...game.eventLog.map((e) => ({ kind: 'event' as const, ts: e.ts, text: e.text })),
        ...game.chatLog.map((c) => ({ kind: 'chat' as const, ts: c.ts, scope: c.scope, from: c.from, text: c.text }))]
            .sort((a, b) => a.ts - b.ts)
    );

    const visibleItems = $derived<LogItem[]>(
        logItems.length > MAX_VISIBLE ? logItems.slice(-MAX_VISIBLE) : logItems
    );

    function send() {
        if (!text.trim()) return;
        game.sendChat(scope, text);
        text = '';
    }
</script>

<div class="flex h-64 flex-col gap-2">
    <div class="flex-1 text-[11.5px] leading-relaxed">
        {#if visibleItems.length > 0}
            <div class="min-h-full flex flex-col justify-end gap-0.5">
                {#each visibleItems as item, i}
                    <div style="opacity: {Math.max(0.1, 1 - (visibleItems.length - i - 1) * 0.15)}" class={item.kind === 'event' ? 'text-white/45' : ''}>
                        {#if item.kind === 'event'}
                            {item.text}
                        {:else}
                            <span class="font-medium text-white">{item.from}</span>
                            <span class="text-white/85">{item.text}</span>
                        {/if}
                    </div>
                {/each}
            </div>
        {/if}
    </div>

    <div class="flex gap-1.5">
        <input
            class="flex-1 rounded-md border border-white/20 bg-white/10 px-2 py-1 text-[11px] text-white placeholder-white/40 shadow-[0_1px_0_rgba(255,255,255,0.05)_inset] transition focus:border-white/50 focus:bg-white/15 focus:outline-none focus:ring-2 focus:ring-white/15"
            placeholder="Type a message…"
            bind:value={text}
            onkeydown={(e) => e.key === 'Enter' && send()}
        />
        <select
            class="cursor-pointer rounded-md border border-white/20 bg-white/10 px-2 py-1 text-[11px] text-white shadow-[0_1px_0_rgba(255,255,255,0.05)_inset] transition focus:border-white/50 focus:bg-white/15 focus:outline-none focus:ring-2 focus:ring-white/15"
            bind:value={scope}
        >
            <option value="ROOM">ROOM</option>
            <option value="GLOBAL">GLOBAL</option>
            <option value="GROUP">GROUP</option>
        </select>
    </div>
</div>

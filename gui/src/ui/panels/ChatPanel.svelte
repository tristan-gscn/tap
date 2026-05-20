<script lang="ts">
    import { game } from '../../state/game.svelte';
    import type { ChatScope } from '../../utils/TAPManager';

    type LogItem =
        | { kind: 'event'; ts: number; text: string }
        | { kind: 'chat'; ts: number; scope: string; from: string; text: string };

    let scope = $state<ChatScope>('ROOM');
    let text = $state('');

    const logItems = $derived<LogItem[]>(
        [...game.eventLog.map((e) => ({ kind: 'event' as const, ts: e.ts, text: e.text })),
        ...game.chatLog.map((c) => ({ kind: 'chat' as const, ts: c.ts, scope: c.scope, from: c.from, text: c.text }))]
            .sort((a, b) => a.ts - b.ts)
    );

    function send() {
        if (!text.trim()) return;
        game.sendChat(scope, text);
        text = '';
    }
</script>

<div class="flex h-64 flex-col gap-2">
    <div class="flex-1 overflow-y-auto text-[11.5px] leading-relaxed">
        {#if logItems.length > 0}
            <div class="min-h-full flex flex-col justify-end gap-0.5">
                {#each logItems as item, i}
                    <div style="opacity: {Math.max(0.1, 1 - (logItems.length - i - 1) * 0.15)}" class={item.kind === 'event' ? 'text-white/45' : ''}>
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
            class="tap-input flex-1"
            placeholder="Type a message…"
            bind:value={text}
            onkeydown={(e) => e.key === 'Enter' && send()}
        />
        <select class="tap-input py-0.5 text-4 cursor-pointer" bind:value={scope}>
            <option value="ROOM">room</option>
            <option value="GLOBAL">global</option>
            <option value="GROUP">group</option>
        </select>
    </div>
</div>

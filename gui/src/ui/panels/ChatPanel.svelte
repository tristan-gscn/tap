<script lang="ts">
    import { game } from '../../state/game.svelte';
    import type { ChatScope } from '../../utils/TAPManager';

    let scope = $state<ChatScope>('ROOM');
    let text = $state('');

    function send() {
        if (!text.trim()) return;
        game.sendChat(scope, text);
        text = '';
    }
</script>

<div class="tap-panel flex h-64 flex-col">
    <div class="tap-panel-title">
        <span>Chat</span>
        <select class="tap-input py-0.5 text-[10.5px]" bind:value={scope}>
            <option value="ROOM">room</option>
            <option value="GLOBAL">global</option>
            <option value="GROUP">group</option>
        </select>
    </div>
    <div class="mb-2 flex-1 overflow-y-auto rounded-lg border border-white/10 bg-white/5 p-2.5 text-[11.5px] leading-relaxed">
        {#if game.chatLog.length === 0 && game.eventLog.length === 0}
            <div class="text-white/45">No messages yet.</div>
        {:else}
            {#each game.eventLog as e}
                <div class="text-white/45">{e.text}</div>
            {/each}
            {#each game.chatLog as c}
                <div>
                    <span class="text-white/50">{c.scope}</span>
                    <span class="font-medium text-white">{c.from}</span>
                    <span class="text-white/85">{c.text}</span>
                </div>
            {/each}
        {/if}
    </div>
    <div class="flex gap-1.5">
        <input
            class="tap-input flex-1"
            placeholder="Type a message…"
            bind:value={text}
            onkeydown={(e) => e.key === 'Enter' && send()}
        />
        <button type="button" class="tap-btn tap-btn-primary px-3" onclick={send}>
            send
        </button>
    </div>
</div>

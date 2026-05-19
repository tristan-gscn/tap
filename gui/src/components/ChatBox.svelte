<script lang="ts">
    import { onMount, tick } from 'svelte';
    import { tapClient } from '../utils/TAPManager';

    type ChatMessage = {
        from: string;
        text: string;
        scope: string;
    };

    let message = $state('');
    let messages = $state<ChatMessage[]>([]);
    let scrollContainer: HTMLDivElement | null = null;
    let isAtBottom = true;
    const bottomThreshold = 8;

    const tap = tapClient;

    const updateIsAtBottom = () => {
        if (!scrollContainer) {
            return;
        }
        const { scrollTop, clientHeight, scrollHeight } = scrollContainer;
        isAtBottom = scrollTop + clientHeight >= scrollHeight - bottomThreshold;
    };

    const pushMessage = async (from: string, text: string, scope: string) => {
        const shouldScroll = isAtBottom;
        messages = [...messages, { from, text, scope }].slice(-200);
        if (shouldScroll) {
            await tick();
            if (scrollContainer) {
                scrollContainer.scrollTop = scrollContainer.scrollHeight;
                updateIsAtBottom();
            }
        }
    };

    function resolveName(): string {
        const stored = localStorage.getItem('tap-player-name');
        if (stored && stored.trim().length > 0) {
            return stored;
        }
        return 'Unknown';
    }

    onMount(() => {
        let unsubscribe: (() => void) | null = null;

        const init = async () => {
            await tap.connect();
            await tap.connectPlayer(resolveName());

            unsubscribe = tap.onEvent((evt) => {
                const data = evt.data as { event?: string; from?: string; text?: string; scope?: string };
                if (!data || !data.event) {
                    return;
                }
                if (data.event === 'chat_room' || data.event === 'chat_global' || data.event === 'chat_group') {
                    pushMessage(data.from ?? '?', data.text ?? '', data.scope ?? '');
                }
            });
        };

        void init();

        return () => {
            unsubscribe?.();
        };
    });

    async function sendMessage() {
        const text = message.trim();
        if (!text) {
            return;
        }
        message = '';
        await tap.chat('ROOM', text);
    }

    function onKeydown(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            event.preventDefault();
            void sendMessage();
        }
    }
</script>

<div class="absolute left-4 top-4 z-20 w-72 border-2 border-white bg-black/90 text-white">
    <div class="border-b-2 border-white px-3 py-2 text-lg select-none">CHAT</div>
    <div
        class="no-scrollbar h-60 overflow-y-auto px-3 py-2 text-sm"
        bind:this={scrollContainer}
        onscroll={updateIsAtBottom}
    >
        <div class="min-h-full flex flex-col justify-end">
            {#each messages as msg, index (index)}
                <div class="mb-1">
                    <span class="text-purple-500">&lt{msg.from}&gt</span>
                    <span> {msg.text}</span>
                </div>
            {/each}
        </div>
    </div>
    <div class="border-t-2 border-white p-2">
        <input
            type="text"
            placeholder="Type a message..."
            class="w-full bg-black text-white placeholder:text-white/50 focus:outline-none"
            bind:value={message}
            onkeydown={onKeydown}
        />
    </div>
</div>

<style>
    :global(.no-scrollbar) {
        scrollbar-width: none;
        -ms-overflow-style: none;
    }

    :global(.no-scrollbar::-webkit-scrollbar) {
        width: 0;
        height: 0;
    }
</style>

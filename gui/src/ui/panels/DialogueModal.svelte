<script lang="ts">
    import X from 'lucide-svelte/icons/x';
    import { game } from '../../state/game.svelte';
</script>

{#if game.dialogue}
    <div class="pointer-events-auto absolute inset-x-0 bottom-32 mx-auto w-[640px] max-w-[92vw]">
        <div class="tap-panel relative">
            <button
                type="button"
                class="tap-btn absolute right-2 top-2 px-1.5 py-0.5"
                onclick={() => game.closeDialogue()}
                aria-label="close"
            >
                <X size={11} />
            </button>
            <div class="pr-8 text-base font-medium text-white">{game.dialogue.name}</div>
            <div class="mt-1 text-[12px] text-white/65">{game.dialogue.description}</div>
            <ul class="mt-3 space-y-1.5">
                {#each game.dialogue.dialogue as line}
                    <li class="rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-[12.5px] text-white/90">
                        {line}
                    </li>
                {/each}
            </ul>
        </div>
    </div>
{/if}

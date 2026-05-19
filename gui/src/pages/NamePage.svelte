<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher<{ ready: void }>();

    let name = $state('');

    const saveName = () => {
        const trimmed = name.trim();
        if (!trimmed) {
            return;
        }
        localStorage.setItem('tap-player-name', trimmed);
        dispatch('ready');
    };

    const handleKeydown = (event: KeyboardEvent) => {
        if (event.key === 'Enter') {
            event.preventDefault();
            saveName();
        }
    };
</script>

<div class="w-screen h-screen bg-black flex items-center justify-center">
    <div class="text-center text-white w-full max-w-lg px-6">
        <h1 class="text-4xl mb-6">Choose your name</h1>
        <div class="border-2 border-white bg-black/80 px-4 py-3">
            <input
                class="w-full bg-black text-white text-2xl placeholder:text-white/50 focus:outline-none"
                placeholder="Your name"
                bind:value={name}
                onkeydown={handleKeydown}
            />
        </div>
        <button
            class="bg-purple-500 hover:bg-purple-600 text-2xl px-6 py-2 mt-6 cursor-pointer"
            onclick={saveName}
        >
            START ADVENTURE
        </button>
    </div>
</div>

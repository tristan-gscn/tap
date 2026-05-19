<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher<{ start: void }>();

    const handleStart = () => {
        dispatch('start');
    };
</script>

<div class="w-screen h-screen bg-black flex items-center justify-center">
    <div class="text-center text-white">
        <img
                src="/characters/player_1.png"
                alt="Player Character"
                class="w-48 h-48 mx-auto mb-4"
        />
        <h1 class="text-6xl">THE ANSWER PROTOCOL</h1>
        <p>A game by <a class="text-purple-500">SGIL--DE</a> and
            <a class="text-purple-500">TRGASCOI</a></p>
        <button
            class="
                bg-purple-500 hover:bg-purple-600
                text-3xl
                px-4 py-2 mt-10
                cursor-pointer
            "
            onclick={handleStart}
        >
            START
        </button>
    </div>
</div>
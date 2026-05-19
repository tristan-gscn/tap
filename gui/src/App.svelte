<script lang="ts">
    import StartPage from "./pages/StartPage.svelte";
    import GamePage from "./pages/game_page/GamePage.svelte";
    import NamePage from "./pages/NamePage.svelte";

    const storedName = localStorage.getItem('tap-player-name');
    let screen: 'start' | 'name' | 'rooms' = storedName ? 'rooms' : 'start';

    const handleStart = () => {
        screen = 'name';
    };

    const handleNameReady = (event: CustomEvent<{name: string}>) => {
        localStorage.setItem('tap-player-name', event.detail.name);
        screen = 'rooms';
    };

    const handleQuit = () => {
        screen = 'start';
    };
</script>

<div class="relative w-screen h-screen">
    {#if screen === 'start'}
        <StartPage on:start={handleStart} />
    {:else if screen === 'name'}
        <NamePage on:ready={handleNameReady} />
    {:else}
        <GamePage on:quit={handleQuit} />
    {/if}
</div>
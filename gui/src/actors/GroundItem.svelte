<script lang="ts">
    import { T } from '@threlte/core';
    import { useGltf } from '@threlte/extras';

    let { url }: { url: string } = $props();
    const gltf = $derived(useGltf(url));
</script>

{#if $gltf}
    <T is={$gltf.scene.clone()} />
{/if}

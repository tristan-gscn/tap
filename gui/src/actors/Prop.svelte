<script lang="ts">
    import { T } from '@threlte/core'
    import { useGltf } from '@threlte/extras'
    import type { Object3D } from 'three'

    let { url, parentBone, side }: { url: string; parentBone: Object3D; side: 'left' | 'right' } = $props()

    const gltf = $derived(useGltf(url))

    $effect(() => {
        // Wait for the GLTF to load and ensure the parent bone exists
        const currentGltf = $gltf
        if (currentGltf && parentBone) {
            if (side === 'right') {
                currentGltf.scene.rotation.z = Math.PI / 2
            } else {
                currentGltf.scene.rotation.z = 3 * Math.PI / 2
            }

            parentBone.add(currentGltf.scene)

            // Cleanup function
            return () => {
                parentBone.remove(currentGltf.scene)
            }
        }
    })
</script>

{#if $gltf && parentBone}
    <T is={$gltf.scene} />
{/if}

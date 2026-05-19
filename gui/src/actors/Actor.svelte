<script lang="ts">
    import { T } from '@threlte/core'
    import { useGltf, useGltfAnimations } from '@threlte/extras'
    import { derived } from 'svelte/store'

    import type { ActorProps } from './ActorProps'

    let { 
        modelUrl, 
        movementUrl, 
        generalUrl, 
        animation,
        position = [0, 0, 0],
        scale = 1
    }: ActorProps & {
        modelUrl: string
        movementUrl?: string
        generalUrl?: string
        position?: [number, number, number]
        scale?: number
    } = $props()

    const gltf = useGltf(modelUrl)
    const movementAnimations = movementUrl ? useGltf(movementUrl) : undefined
    const generalAnimations = generalUrl ? useGltf(generalUrl) : undefined

    // Combine the main model and all animations into a single store
    const combinedGltf = derived(
        [gltf, movementAnimations || derived([], () => undefined), generalAnimations || derived([], () => undefined)],
        ([mainGltf, $movement, $general]) => {
            if (!mainGltf) return undefined

            return {
                ...mainGltf,
                animations: [
                    ...(mainGltf.animations || []),
                    ...($movement?.animations || []),
                    ...($general?.animations || [])
                ]
            }
        }
    )

    const { actions } = useGltfAnimations(combinedGltf)

    $inspect($actions)
    $effect(() => {
        if (animation && $actions[animation]) {
            $actions[animation].play()
            return () => {
                $actions[animation]?.stop()
            }
        }
    })
</script>

{#if $gltf}
    <T is={$gltf.scene} {position} {scale} />
{/if}

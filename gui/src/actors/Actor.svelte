<script lang="ts">
    import { T } from '@threlte/core'
    import { useGltf, useGltfAnimations } from '@threlte/extras'
    import { derived } from 'svelte/store'
    import type { Object3D, SkinnedMesh } from 'three'

    import type { ActorProps } from './ActorProps'
    import Prop from './Prop.svelte'

    let { 
        modelUrl, 
        movementUrl, 
        generalUrl, 
        animation,
        leftHandProp,
        rightHandProp,
        position = [0, 0, 0],
        scale = 1
    }: ActorProps & {
        modelUrl: string
        movementUrl?: string
        generalUrl?: string
        position?: [number, number, number]
        scale?: number
    } = $props()

    const gltf = $derived(useGltf(modelUrl))
    const movementAnimations = $derived(movementUrl ? useGltf(movementUrl) : undefined)
    const generalAnimations = $derived(generalUrl ? useGltf(generalUrl) : undefined)

    const combatMeleeAnimations = useGltf('/models/animations/combat_melee.glb')
    const combatRangedAnimations = useGltf('/models/animations/combat_ranged.glb')
    const simulationAnimations = useGltf('/models/animations/simulation.glb')
    const specialAnimations = useGltf('/models/animations/special.glb')
    const toolsAnimations = useGltf('/models/animations/tools.glb')
    const movementAdvancedAnimations = useGltf('/models/animations/movement_advanced.glb')

    // Combine the main model and all animations into a single store
    const combinedGltf = derived(
        [
            derived([gltf], ([$g]) => $g), 
            movementAnimations ? derived([movementAnimations], ([$m]) => $m) : derived([], () => undefined), 
            generalAnimations ? derived([generalAnimations], ([$g]) => $g) : derived([], () => undefined),
            combatMeleeAnimations,
            combatRangedAnimations,
            simulationAnimations,
            specialAnimations,
            toolsAnimations,
            movementAdvancedAnimations
        ],
        ([mainGltf, $movement, $general, $melee, $ranged, $sim, $special, $tools, $movAdv]) => {
            if (!mainGltf) return undefined

            return {
                ...mainGltf,
                animations: [
                    ...(mainGltf.animations || []),
                    ...($movement?.animations || []),
                    ...($general?.animations || []),
                    ...($melee?.animations || []),
                    ...($ranged?.animations || []),
                    ...($sim?.animations || []),
                    ...($special?.animations || []),
                    ...($tools?.animations || []),
                    ...($movAdv?.animations || [])
                ]
            }
        }
    )

    const { actions } = useGltfAnimations(combinedGltf)

    let rightHand = $derived.by(() => {
        const currentGltf = $gltf
        if (!currentGltf) return undefined
        let found: Object3D | undefined
        currentGltf.scene.traverse((child) => {
            if (child.name.endsWith('_ArmRight')) {
                found = child
            }
        })
        const skinned = found as SkinnedMesh | undefined
        if (!skinned || !skinned.skeleton) return undefined
        return skinned.skeleton.bones.find((bone) => bone.name == "handr")
    })

    let leftHand = $derived.by(() => {
        const currentGltf = $gltf
        if (!currentGltf) return undefined
        let found: Object3D | undefined
        currentGltf.scene.traverse((child) => {
            if (child.name.endsWith('_ArmLeft')) {
                found = child
            }
        })
        const skinned = found as SkinnedMesh | undefined
        return skinned?.skeleton.bones.find((bone) => bone.name == "handl")
    })

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

    {#if rightHandProp && rightHand}
        <Prop url={rightHandProp} parentBone={rightHand} side="right" />
    {/if}

    {#if leftHandProp && leftHand}
        <Prop url={leftHandProp} parentBone={leftHand} side="left" />
    {/if}
{/if}

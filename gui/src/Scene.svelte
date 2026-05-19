<script lang="ts">
    import { T } from '@threlte/core'
    import { OrbitControls } from '@threlte/extras'
    import {
        ActorAnimation
    } from "./actors/ActorAnimation";
    import KnightCharacter_Actor
        from "./actors/character/KnightCharacter_Actor.svelte";
</script>

<T.PerspectiveCamera makeDefault position={[5, 5, 5]}>
    <OrbitControls enableDamping target={[0, 1, 0]} />
</T.PerspectiveCamera>

<T.DirectionalLight position={[3, 10, 10]} intensity={1} />
<T.AmbientLight intensity={0.5} />

<KnightCharacter_Actor animation={ActorAnimation.Walking_A} />

<script lang="ts">
    import { T } from '@threlte/core'
    import { OrbitControls } from '@threlte/extras'
    import MageSkeleton_Actor
        from "./actors/skeleton/MageSkeleton_Actor.svelte";
    import {
        Skeleton_ActorAnimation
    } from "./actors/skeleton/Skeleton_ActorAnimation";
</script>

<T.PerspectiveCamera makeDefault position={[5, 5, 5]}>
    <OrbitControls enableDamping target={[0, 1, 0]} />
</T.PerspectiveCamera>

<T.DirectionalLight position={[3, 10, 10]} intensity={1} />
<T.AmbientLight intensity={0.5} />

<MageSkeleton_Actor animation={Skeleton_ActorAnimation.Death_B} />

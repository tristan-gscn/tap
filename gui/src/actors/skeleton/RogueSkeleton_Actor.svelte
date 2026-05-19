<script lang="ts">
    import Actor from "../Actor.svelte";
    import {Skeleton_ActorAnimation} from "./Skeleton_ActorAnimation.ts";

    let { animation }: { animation: Skeleton_ActorAnimation } = $props()
</script>

<Actor
        modelUrl="/models/skeleton/rogue.glb"
        movementUrl="/models/skeleton/rig_movement.glb"
        generalUrl="/models/skeleton/rig_general.glb"
        animation={animation}
/>

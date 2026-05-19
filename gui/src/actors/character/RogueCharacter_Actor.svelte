<script lang="ts">
    import Actor from "../Actor.svelte";
    import {ActorAnimation} from "../ActorAnimation.ts";

    let { animation }: { animation: ActorAnimation } = $props()
</script>

<Actor
        modelUrl="/models/character/rogue.glb"
        movementUrl="/models/animations/movement.glb"
        generalUrl="/models/animations/general.glb"
        animation={animation}
/>

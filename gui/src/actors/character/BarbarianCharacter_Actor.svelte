<script lang="ts">
    import Actor from "../Actor.svelte";
    import type { ActorProps } from "../ActorProps";

    let { animation }: ActorProps = $props()
</script>

<Actor
        modelUrl="/models/character/barbarian.glb"
        movementUrl="/models/animations/movement.glb"
        generalUrl="/models/animations/general.glb"
        animation={animation}
/>

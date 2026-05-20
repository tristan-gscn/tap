<script lang="ts">
    import Actor from "../Actor.svelte";
    import type { ActorProps } from "../ActorProps";

    let { animation, leftHandProp, rightHandProp }: ActorProps = $props()
</script>

<Actor
        modelUrl="/models/character/knight.glb"
        movementUrl="/models/animations/movement.glb"
        generalUrl="/models/animations/general.glb"
        animation={animation}
        leftHandProp={leftHandProp}
        rightHandProp={rightHandProp}
/>

<script lang="ts">
    import { T, useTask } from '@threlte/core';
    import { ActorAnimation } from '../actors/ActorAnimation';
    import { resolveCharacterModel } from '../registries/characterModels';

    let { classId }: { classId: string } = $props();
    const CharComp = $derived(resolveCharacterModel(classId));

    let rotationY = $state(0);

    useTask((delta) => {
        rotationY += delta * 0.6;
    });
</script>

<T.AmbientLight intensity={0.6} color="#fff5e1" />
<T.DirectionalLight position={[3, 4, 3]} intensity={1.4} color="#fff5e1" />
<T.DirectionalLight position={[-3, 2, -2]} intensity={0.6} color="#7aa3ff" />

<T.PerspectiveCamera makeDefault position={[0, 1.5, 4]} fov={32} />

<T.Group position={[0, 0, 0]} rotation={[0, rotationY, 0]}>
    <CharComp animation={ActorAnimation.Idle_A} />
</T.Group>

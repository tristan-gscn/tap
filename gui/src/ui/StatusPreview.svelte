<script lang="ts">
    import { T } from '@threlte/core';
    import { ActorAnimation } from '../actors/ActorAnimation';
    import { resolveCharacterModel } from '../registries/characterModels';

    let { classId }: { classId: string } = $props();
    const CharComp = $derived(resolveCharacterModel(classId));

    const rotationY = 0;
</script>

<T.AmbientLight intensity={0.65} color="#f5f2ff" />
<T.DirectionalLight position={[2.5, 3.5, 3]} intensity={1.1} color="#ffffff" />
<T.DirectionalLight position={[-2.5, 2, -2]} intensity={0.5} color="#7aa3ff" />

<T.PerspectiveCamera makeDefault position={[0, 1.5, 7]} fov={28} />

<T.Group position={[-0.35, 0.15, 0]} rotation={[0, rotationY, 0]}>
    <CharComp animation={ActorAnimation.Idle_A} />
</T.Group>

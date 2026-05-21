<script lang="ts">
    // A simple procedural loot sack, used as the ground model for any item that
    // has no dedicated 3D prop. No external asset required.
    import { T } from '@threlte/core';
</script>

<T.Group>
    <!-- bulging body -->
    <T.Mesh position={[0, 0.45, 0]} scale={[1, 1.15, 1]} castShadow receiveShadow>
        <T.SphereGeometry args={[0.5, 14, 12]} />
        <T.MeshStandardMaterial color="#8a6a40" roughness={0.95} />
    </T.Mesh>
    <!-- tied neck -->
    <T.Mesh position={[0, 0.92, 0]} castShadow>
        <T.CylinderGeometry args={[0.16, 0.26, 0.18, 12]} />
        <T.MeshStandardMaterial color="#6f5532" roughness={1} />
    </T.Mesh>
    <!-- cinched top -->
    <T.Mesh position={[0, 1.05, 0]} castShadow>
        <T.SphereGeometry args={[0.13, 8, 8]} />
        <T.MeshStandardMaterial color="#5e4628" roughness={1} />
    </T.Mesh>
</T.Group>

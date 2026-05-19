<script lang="ts">
  const sizeX = 20;
  const sizeZ = 10;
  const lastX = (sizeX - 1) * 2;
  const lastZ = (sizeZ - 1) * 2;

  import Dungeon from './room/dungeon.svelte'
  import Actor from "./Actor.svelte"
  const heroPosition: [number, number, number] = [lastX / 2 + 2, 0, lastZ / 2];
</script>

<Dungeon />

<Actor
  modelUrl="models/Skeleton_Warrior.glb"
  position={heroPosition}
  scale={1}
/>
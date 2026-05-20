<script lang="ts">
  import { T } from "@threlte/core";
  import { OrbitControls, useGltf } from "@threlte/extras";
  import type { IntersectionEvent } from "@threlte/extras";

  type Direction = "north" | "south" | "east" | "west";

  type Props = {
    onDoorClick?: (direction: Direction) => void;
    onFloorClick?: (position: [number, number, number]) => void;
    availableExits?: Direction[];
  };

  let { onDoorClick, onFloorClick, availableExits = [] }: Props = $props();

  const sizeX = 20;
  const sizeZ = 10;
  const tilesX = Array.from({ length: sizeX }, (_, i) => i);
  const tilesZ = Array.from({ length: sizeZ }, (_, i) => i);
  const lastX = (sizeX - 1) * 2;
  const lastZ = (sizeZ - 1) * 2;
  const WALL_N = -1.5;
  const WALL_S = lastZ + 1.5;
  const WALL_W = -1.5;
  const WALL_E = lastX + 1.5;
  const doorXIndex = Math.floor(sizeX / 2);
  const doorZIndex = Math.floor(sizeZ / 2);
  const doorX = doorXIndex * 2;
  const doorZ = doorZIndex * 2;

  const floor = useGltf("room/dungeon/gltf/floor_tile_large.gltf");
  const wall = useGltf("room/dungeon/gltf/wall.gltf");
  const barrel = useGltf("room/dungeon/gltf/barrel_large.gltf");
  const chest = useGltf("room/dungeon/gltf/chest.gltf");
  const torch = useGltf("room/dungeon/gltf/torch_mounted.gltf");
  const door = useGltf("room/dungeon/gltf/wall_doorway.gltf");

  const torchMargin = 2;
  const torchStepX = 6;
  const torchStepZ = 6;
  const torchXs: number[] = [];
  const torchZs: number[] = [];

  for (let x = torchMargin; x <= lastX - torchMargin; x += torchStepX) {
    torchXs.push(x);
  }

  for (let z = torchMargin; z <= lastZ - torchMargin; z += torchStepZ) {
    torchZs.push(z);
  }

  const torchY = 1.6;
  const lightY = 2.35;
  const glowY = 0.35;
  const torchOffset = 1.1;

  const torchMounts = [
    ...torchXs.map((x) => ({
      position: [x, torchY, WALL_N] as [number, number, number],
      rotation: [0, 0, 0] as [number, number, number],
      lightPos: [x, lightY, WALL_N + torchOffset] as [number, number, number],
      glowPos: [x, glowY, WALL_N + torchOffset] as [number, number, number],
    })),
    ...torchXs.map((x) => ({
      position: [x, torchY, WALL_S] as [number, number, number],
      rotation: [0, Math.PI, 0] as [number, number, number],
      lightPos: [x, lightY, WALL_S - torchOffset] as [number, number, number],
      glowPos: [x, glowY, WALL_S - torchOffset] as [number, number, number],
    })),
    ...torchZs.map((z) => ({
      position: [WALL_W, torchY, z] as [number, number, number],
      rotation: [0, Math.PI / 2, 0] as [number, number, number],
      lightPos: [WALL_W + torchOffset, lightY, z] as [number, number, number],
      glowPos: [WALL_W + torchOffset, glowY, z] as [number, number, number],
    })),
    ...torchZs.map((z) => ({
      position: [WALL_E, torchY, z] as [number, number, number],
      rotation: [0, -Math.PI / 2, 0] as [number, number, number],
      lightPos: [WALL_E - torchOffset, lightY, z] as [number, number, number],
      glowPos: [WALL_E - torchOffset, glowY, z] as [number, number, number],
    })),
  ];

  const torchLightMounts = torchMounts.filter((_, index) => index % 2 === 0);

  const doorInset = 0.5;
  const doorLift = 0.02;

  type DoorMount = {
    direction: Direction;
    position: [number, number, number];
    rotation: [number, number, number];
  };

  const doorMounts: DoorMount[] = [
    {
      direction: "north",
      position: [doorX, doorLift, WALL_N + doorInset],
      rotation: [0, 0, 0],
    },
    {
      direction: "south",
      position: [doorX, doorLift, WALL_S - doorInset],
      rotation: [0, Math.PI, 0],
    },
    {
      direction: "west",
      position: [WALL_W + doorInset, doorLift, doorZ],
      rotation: [0, Math.PI / 2, 0],
    },
    {
      direction: "east",
      position: [WALL_E - doorInset, doorLift, doorZ],
      rotation: [0, -Math.PI / 2, 0],
    },
  ];

  type ActorModel = "barrel" | "chest" | "torch";
  type ActorItem = {
    model: ActorModel;
    position: [number, number, number];
    rotation?: [number, number, number];
  };

  const actorProps: ActorItem[] = [
    { model: "barrel", position: [2, 0, 6] },
    { model: "barrel", position: [6, 0, 2], rotation: [0, 0.9, 0] },
    {
      model: "chest",
      position: [lastX / 2, 0, lastZ / 2],
      rotation: [0, -0.4, 0],
    },
  ];

  const sceneryActors: ActorItem[] = [
    ...torchMounts.map(
      (torchMount): ActorItem => ({
        model: "torch",
        position: torchMount.position,
        rotation: torchMount.rotation,
      }),
    ),
    ...actorProps,
  ];

  let hoveredDoor = $state<Direction | null>(null);
</script>

<T.AmbientLight intensity={0.08} color="#0b0b14" />
<T.HemisphereLight intensity={2.0} color="#23335a" groundColor="#120916" />
<T.DirectionalLight
  position={[lastX * 0.4, 6, lastZ * 0.7]}
  intensity={0.25}
  color="#2a3b6a"
/>

{#each torchLightMounts as torchMount}
  <T.PointLight
    position={torchMount.lightPos}
    intensity={11.0}
    distance={24}
    decay={2.4}
    color="#ff7a2a"
  />
{/each}

<T.PerspectiveCamera
  makeDefault
  position={[lastX / 2, 26, lastZ / 2 + 1]}
  fov={50}
>
  <OrbitControls
    enableDamping
    dampingFactor={0.08}
    target={[lastX / 2, 1, lastZ / 2]}
    enableZoom={true}
    enablePan={false}
    maxPolarAngle={Math.PI / 2.5}
    minPolarAngle={Math.PI / 7}
    minDistance={12}
    maxDistance={36}
  />
</T.PerspectiveCamera>

{#await floor then floorModel}
  {#await wall then wallModel}
    {#await barrel then barrelModel}
      {#await chest then chestModel}
        {#await torch then torchModel}
          {#await door then doorModel}
            {#each tilesX as x}
              {#each tilesZ as y}
                <T.Group position={[x * 2, 0, y * 2]}>
                  <T is={floorModel.scene.clone()} />
                </T.Group>
              {/each}
            {/each}

            <T.Mesh
              position={[lastX / 2, 0.05, lastZ / 2]}
              rotation={[-Math.PI / 2, 0, 0]}
              onclick={(e: IntersectionEvent<MouseEvent>) => {
                if (!onFloorClick) return;
                const p = e.point;
                onFloorClick([p.x, 0, p.z]);
              }}
            >
              <T.PlaneGeometry args={[lastX + 4, lastZ + 4]} />
              <T.MeshBasicMaterial transparent opacity={0} />
            </T.Mesh>

            {#each tilesX as i}
              {#if i !== doorXIndex}
                <T.Group position={[i * 2, 0, WALL_N]}>
                  <T is={wallModel.scene.clone()} />
                </T.Group>
                <T.Group
                  position={[i * 2, 0, WALL_S]}
                  rotation={[0, Math.PI, 0]}
                >
                  <T is={wallModel.scene.clone()} />
                </T.Group>
              {/if}
            {/each}

            {#each tilesZ as i}
              {#if i !== doorZIndex}
                <T.Group
                  position={[WALL_W, 0, i * 2]}
                  rotation={[0, Math.PI / 2, 0]}
                >
                  <T is={wallModel.scene.clone()} />
                </T.Group>
                <T.Group
                  position={[WALL_E, 0, i * 2]}
                  rotation={[0, -Math.PI / 2, 0]}
                >
                  <T is={wallModel.scene.clone()} />
                </T.Group>
              {/if}
            {/each}

            {#each doorMounts as dm (dm.direction)}
              {@const enabled = availableExits.includes(dm.direction)}
              <T.Group
                position={dm.position}
                rotation={dm.rotation}
                onclick={(e: IntersectionEvent<MouseEvent>) => {
                  if (!enabled || !onDoorClick) return;
                  e.stopPropagation();
                  onDoorClick(dm.direction);
                }}
                onpointerenter={() => {
                  if (!enabled) return;
                  hoveredDoor = dm.direction;
                  document.body.style.cursor = "pointer";
                }}
                onpointerleave={() => {
                  if (hoveredDoor === dm.direction) hoveredDoor = null;
                  document.body.style.cursor = "default";
                }}
              >
                <T is={doorModel.scene.clone()} />
              </T.Group>
              {#if enabled}
                <T.PointLight
                  position={[
                    dm.position[0],
                    1.5,
                    dm.position[2],
                  ]}
                  intensity={hoveredDoor === dm.direction ? 12 : 5}
                  distance={6}
                  decay={2}
                  color="#f5d177"
                />
              {/if}
            {/each}

            {#each sceneryActors as actor}
              <T.Group
                position={actor.position}
                rotation={actor.rotation ?? [0, 0, 0]}
              >
                {#if actor.model === "barrel"}
                  <T is={barrelModel.scene.clone()} />
                {:else if actor.model === "chest"}
                  <T is={chestModel.scene.clone()} />
                {:else if actor.model === "torch"}
                  <T is={torchModel.scene.clone()} />
                {/if}
              </T.Group>
            {/each}
          {/await}
        {/await}
      {/await}
    {/await}
  {/await}
{/await}

<script>
  import { Canvas } from '@threlte/core'
  import Scene from './Scene.svelte'
</script>

<div class="h-screen w-screen bg-gray-900">
  <Canvas>
    <Scene />
  </Canvas>
</div>

<div class="absolute top-4 left-4 text-white pointer-events-none">
  <h1 class="text-4xl font-bold">The Answer Protocol</h1>
</div>
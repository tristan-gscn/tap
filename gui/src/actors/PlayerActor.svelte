<script lang="ts">
    import { T, useTask } from '@threlte/core';
    import { ActorAnimation } from './ActorAnimation';
    import { resolveCharacterModel } from '../registries/characterModels';
    import { game } from '../state/game.svelte';

    const CharComp = $derived(resolveCharacterModel(game.playerClass));

    let rotationY = $state(Math.PI);

    const SPEED = 6;

    useTask((delta) => {
        const [px, py, pz] = game.playerPos;
        const [tx, , tz] = game.playerTarget;
        const dx = tx - px;
        const dz = tz - pz;
        const dist = Math.hypot(dx, dz);

        if (dist > 0.08) {
            const step = Math.min(dist, SPEED * delta);
            const nx = px + (dx / dist) * step;
            const nz = pz + (dz / dist) * step;
            game.playerPos = [nx, py, nz];
            rotationY = Math.atan2(dx, dz);
            if (!game.attacking && game.playerAnimation !== ActorAnimation.Walking_A) {
                game.playerAnimation = ActorAnimation.Walking_A;
            }
        } else if (!game.attacking && game.playerAnimation !== ActorAnimation.Idle_A) {
            game.playerAnimation = ActorAnimation.Idle_A;
        }
    });
</script>

<T.Group position={game.playerPos} rotation={[0, rotationY, 0]}>
    <CharComp animation={game.playerAnimation} />
</T.Group>

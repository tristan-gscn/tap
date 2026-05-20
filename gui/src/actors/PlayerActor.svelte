<script lang="ts">
    import { T, useTask } from '@threlte/core';
    import { ActorAnimation } from './ActorAnimation';
    import { resolveCharacterModel } from '../registries/characterModels';
    import { game } from '../state/game.svelte';
    import { resolveItemProp } from '../registries/itemProps';

    const CharComp = $derived(resolveCharacterModel(game.playerClass));
    const rightHandProp = $derived(resolveItemProp(game.equippedRight));
    const leftHandProp = $derived(resolveItemProp(game.equippedLeft));

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

        const target = game.attackTarget;
        if (target && !game.attacking) {
            const [ax, , az] = target.pos;
            const adx = ax - px;
            const adz = az - pz;
            const attackDist = Math.hypot(adx, adz);
            if (attackDist <= 1.6) {
                game.attackTarget = null;
                void game.attack(target.type);
            }
        }
    });
</script>

<T.Group position={game.playerPos} rotation={[0, rotationY, 0]}>
    <CharComp
        animation={game.playerAnimation}
        rightHandProp={rightHandProp ?? undefined}
        leftHandProp={leftHandProp ?? undefined}
    />
</T.Group>

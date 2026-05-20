<script lang="ts">
    import { game } from '../../state/game.svelte';
    import { resolveItemIcon } from '../../registries/itemIcons';
    import type { ItemDetail } from '../../state/game.svelte';

    const INVENTORY_SLOTS = 16;

    const slots = $derived(
        Array.from({ length: INVENTORY_SLOTS }, (_, i) => game.inventoryDetail[i] ?? null)
    );
</script>

<div class="tap-panel">
    <div class="tap-panel-title">
        <span>Inventory</span>
        <span class="text-[11px] font-normal text-white/55">{game.inventoryDetail.length}/{INVENTORY_SLOTS}</span>
    </div>

    <div class="grid grid-cols-4 gap-1.5">
        {#each slots as item, i (i)}
            {@const Icon = resolveItemIcon(item?.id)}
            <button
                type="button"
                class="tap-slot {item ? '' : 'tap-slot-empty'}"
                onclick={() => item && game.drop(item.id)}
                title={item ? `${item.name} (click to drop)` : 'empty'}
                disabled={!item}
            >
                <Icon size={26} strokeWidth={1.6} />
                {#if item}
                    <span class="tap-tooltip">{item.name}<br /><span class="text-white/65">click to drop</span></span>
                {/if}
            </button>
        {/each}
    </div>
</div>

<script lang="ts">
    import { game } from '../../state/game.svelte';
    import { resolveItemIcon } from '../../registries/itemIcons';
    import type { ItemDetail } from '../../state/game.svelte';

    const INVENTORY_SLOTS = 16;

    const slots = $derived(
        Array.from({ length: INVENTORY_SLOTS }, (_, i) => game.inventoryDetail[i] ?? null)
    );

    const rightItem = $derived(
        game.inventoryDetail.find((i) => i.id === game.equippedRight) ?? null
    );
    const leftItem = $derived(
        game.inventoryDetail.find((i) => i.id === game.equippedLeft) ?? null
    );
    const RightIcon = $derived(resolveItemIcon(rightItem?.id));
    const LeftIcon = $derived(resolveItemIcon(leftItem?.id));
</script>

<div class="tap-panel">
    <div class="tap-panel-title">
        <span>Inventory</span>
        <span class="text-[11px] font-normal text-white/55">{game.inventoryDetail.length}/{INVENTORY_SLOTS}</span>
    </div>

    <div class="mb-2 grid grid-cols-2 gap-2">
        <div class="rounded-lg border border-white/10 bg-white/5 px-2 py-1.5 text-[11px]">
            <div class="mb-1 text-[10px] uppercase tracking-[0.2em] text-white/55">Right hand</div>
            {#if rightItem}
                <div class="flex items-center justify-between gap-2">
                    <span class="inline-flex items-center gap-2">
                        <RightIcon size={14} class="text-white/85" />
                        <span>{rightItem.name}</span>
                    </span>
                    <button
                        type="button"
                        class="tap-btn px-2 py-0.5 text-[10px]"
                        onclick={() => game.unequip('right')}
                    >unequip</button>
                </div>
            {:else}
                <div class="text-white/55">empty</div>
            {/if}
        </div>
        <div class="rounded-lg border border-white/10 bg-white/5 px-2 py-1.5 text-[11px]">
            <div class="mb-1 text-[10px] uppercase tracking-[0.2em] text-white/55">Left hand</div>
            {#if leftItem}
                <div class="flex items-center justify-between gap-2">
                    <span class="inline-flex items-center gap-2">
                        <LeftIcon size={14} class="text-white/85" />
                        <span>{leftItem.name}</span>
                    </span>
                    <button
                        type="button"
                        class="tap-btn px-2 py-0.5 text-[10px]"
                        onclick={() => game.unequip('left')}
                    >unequip</button>
                </div>
            {:else}
                <div class="text-white/55">empty</div>
            {/if}
        </div>
    </div>

    <div class="grid grid-cols-4 gap-1.5">
        {#each slots as item, i (i)}
            {@const Icon = resolveItemIcon(item?.id)}
            <button
                type="button"
                class="tap-slot {item ? '' : 'tap-slot-empty'}"
                onclick={(e) => {
                    if (!item) return;
                    if (e.shiftKey) {
                        game.equip('left', item.id);
                    } else {
                        game.equip('right', item.id);
                    }
                }}
                oncontextmenu={(e) => {
                    if (!item) return;
                    e.preventDefault();
                    game.drop(item.id);
                }}
                title={item ? `${item.name} (click to equip right, shift-click for left)` : 'empty'}
                disabled={!item}
            >
                <Icon size={26} strokeWidth={1.6} />
                {#if item}
                    <span class="tap-tooltip">{item.name}<br /><span class="text-white/65">click: right hand • shift-click: left hand • right-click: drop</span></span>
                {/if}
            </button>
        {/each}
    </div>

    <div class="mt-2 flex items-center justify-between text-[10px] text-white/55">
        <span>Tip: Shift-click equips to left hand.</span>
        <span>Right-click drops.</span>
    </div>

    {#if game.itemsDetail.length > 0}
        <div class="mt-3 border-t border-white/10 pt-2.5">
            <div class="mb-1.5 text-[11px] text-white/60">On the floor</div>
            <ul class="space-y-1">
                {#each game.itemsDetail as item (item.id)}
                    {@const Icon = resolveItemIcon(item.id)}
                    <li class="flex items-center justify-between rounded-lg border border-white/10 bg-white/5 px-2 py-1.5 text-[12px]">
                        <span class="inline-flex items-center gap-2">
                            <Icon size={14} class="text-white/85" />
                            <span>{item.name}</span>
                        </span>
                        <button
                            type="button"
                            class="tap-btn tap-btn-primary px-2 py-0.5 text-[10px]"
                            onclick={() => game.take(item.id)}
                        >
                            take
                        </button>
                    </li>
                {/each}
            </ul>
        </div>
    {/if}
</div>

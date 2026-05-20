<script lang="ts">
    import LiquidGlassPanel from '../LiquidGlassPanel.svelte';
    import { game } from '../../state/game.svelte';
    import { resolveItemIcon } from '../../registries/itemIcons';
    import type { ItemDetail } from '../../state/game.svelte';

    const INVENTORY_SLOTS = 16;

    const slots = $derived(
        Array.from({ length: INVENTORY_SLOTS }, (_, i) => game.inventoryDetail[i] ?? null)
    );

    const equippedLeft = $derived(game.equippedLeft);
    const equippedRight = $derived(game.equippedRight);
    const displayLeft = $derived(equippedLeft && equippedLeft === equippedRight ? null : equippedLeft);
    const displayRight = $derived(equippedRight);

    async function equipLeft(itemId: string) {
        if (game.equippedRight === itemId) {
            await game.unequip('right');
        }
        await game.equip('left', itemId);
    }

    async function equipRight(itemId: string) {
        if (game.equippedLeft === itemId) {
            await game.unequip('left');
        }
        await game.equip('right', itemId);
    }

    async function unequipLeft() {
        await game.unequip('left');
    }

    async function unequipRight() {
        await game.unequip('right');
    }
</script>

<LiquidGlassPanel>
    <div class="mb-2 flex items-center justify-between text-[0.82rem] font-medium text-white/90">
        <span>Inventory</span>
        <span class="text-[11px] font-normal text-white/55">{game.inventoryDetail.length}/{INVENTORY_SLOTS}</span>
    </div>

    <div class="mb-2 grid grid-cols-2 gap-2 text-[11px]">
        <div class="flex items-center justify-between rounded-lg border border-white/10 bg-white/5 px-2 py-1">
            <span class="text-white/60">Left</span>
            <div class="flex items-center gap-1 text-white/90">
                {#if displayLeft}
                    {@const LeftIcon = resolveItemIcon(displayLeft)}
                    <LeftIcon size={14} class="text-white/85" />
                    <span class="max-w-[80px] truncate">{displayLeft}</span>
                    <button
                            type="button"
                            class="ml-1 inline-flex items-center justify-center rounded-md border border-white/20 bg-white/10 px-1.5 py-0.5 text-[10px] text-white transition hover:bg-white/20"
                            onclick={unequipLeft}
                    >
                        ✕
                    </button>
                {:else}
                    <span class="text-white/40">empty</span>
                {/if}
            </div>
        </div>
        <div class="flex items-center justify-between rounded-lg border border-white/10 bg-white/5 px-2 py-1">
            <span class="text-white/60">Right</span>
            <div class="flex items-center gap-1 text-white/90">
                {#if displayRight}
                    {@const RightIcon = resolveItemIcon(displayRight)}
                    <RightIcon size={14} class="text-white/85" />
                    <span class="max-w-[80px] truncate">{displayRight}</span>
                    <button
                            type="button"
                            class="ml-1 inline-flex items-center justify-center rounded-md border border-white/20 bg-white/10 px-1.5 py-0.5 text-[10px] text-white transition hover:bg-white/20"
                            onclick={unequipRight}
                    >
                        ✕
                    </button>
                {:else}
                    <span class="text-white/40">empty</span>
                {/if}
            </div>
        </div>
    </div>

    <div class="grid w-full grid-cols-4 gap-2">
        {#each slots as item, i (i)}
            {@const Icon = resolveItemIcon(item?.id)}
            <div
                    class={`group relative flex aspect-square w-full items-center justify-center rounded-lg border border-white/10 bg-white/5 text-white/85 shadow-[0_1px_0_rgba(255,255,255,0.06)_inset] transition ${item ? 'cursor-pointer hover:border-white/30 hover:bg-white/15' : 'cursor-default opacity-45'}`}
                    title={item ? item.name : 'empty'}
            >
                <Icon size={30} strokeWidth={1.6} />
                {#if item}
                    <div class="pointer-events-none absolute bottom-[calc(100%+6px)] left-1/2 z-20 w-max -translate-x-1/2 rounded-md border border-white/15 bg-[#120e16]/90 px-2 py-1 text-[10px] text-white opacity-0 shadow-[0_8px_20px_rgba(0,0,0,0.4)] transition group-hover:opacity-100">
                        {item.name}
                    </div>

                    <div class="absolute inset-0 z-10 grid grid-cols-2 grid-rows-2 gap-1 p-1 rounded-lg bg-[#120e16]/85 opacity-0 backdrop-blur-[2px] transition group-hover:opacity-100">
                        <button
                                type="button"
                                class="cursor-pointer col-span-2 flex h-full w-full items-center justify-center rounded border border-white/15 bg-white/5 text-[9px] text-white shadow-[0_1px_0_rgba(255,255,255,0.04)_inset] transition hover:bg-white/20 active:bg-white/30"
                                onclick={() => game.drop(item.id)}
                        >
                            drop
                        </button>
                        <button
                                type="button"
                                class="cursor-pointer flex h-full w-full items-center justify-center rounded border border-white/15 bg-white/5 text-[9px] text-white shadow-[0_1px_0_rgba(255,255,255,0.04)_inset] transition hover:bg-white/20 active:bg-white/30"
                                onclick={() => equipLeft(item.id)}
                        >
                            left
                        </button>
                        <button
                                type="button"
                                class="cursor-pointer flex h-full w-full items-center justify-center rounded border border-white/15 bg-white/5 text-[9px] text-white shadow-[0_1px_0_rgba(255,255,255,0.04)_inset] transition hover:bg-white/20 active:bg-white/30"
                                onclick={() => equipRight(item.id)}
                        >
                            right
                        </button>
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</LiquidGlassPanel>
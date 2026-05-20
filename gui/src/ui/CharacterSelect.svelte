<script lang="ts">
    import { Canvas } from '@threlte/core';
    import { ChevronLeft, ChevronRight } from 'lucide-svelte';
    import { CHARACTER_CLASSES } from '../registries/characterModels';
    import { game } from '../state/game.svelte';
    import CharacterPreview from './CharacterPreview.svelte';

    let name = $state('');
    let index = $state(0);
    let submitting = $state(false);

    const current = $derived(CHARACTER_CLASSES[index]);

    function prev() {
        index = (index - 1 + CHARACTER_CLASSES.length) % CHARACTER_CLASSES.length;
    }
    function next() {
        index = (index + 1) % CHARACTER_CLASSES.length;
    }

    async function submit() {
        if (!name.trim() || submitting) return;
        submitting = true;
        await game.connect(name.trim(), current.id);
        submitting = false;
    }
</script>

<div class="absolute inset-0 z-50 flex items-center justify-center bg-[radial-gradient(ellipse_at_center,rgba(60,40,80,0.45)_0%,rgba(10,8,18,0.85)_100%)]">
    <div class="glass-frame w-[760px] max-w-[94vw] p-7">
        <div class="mb-4 flex items-baseline justify-between border-b border-white/10 pb-3">
            <div>
                <h1 class="font-upheaval text-3xl tracking-wider text-white">
                    The Answer Protocol
                </h1>
                <p class="mt-1 text-xs text-white/60">
                    Choisis ton héros et ton nom pour entrer dans le donjon.
                </p>
            </div>
            <div class="text-right text-xs text-white/50">
                {index + 1} / {CHARACTER_CLASSES.length}
            </div>
        </div>

        <div class="mb-4 flex items-stretch gap-3">
            <button
                type="button"
                class="slider-arrow"
                onclick={prev}
                aria-label="Previous character"
            >
                <ChevronLeft size={28} strokeWidth={2.5} />
            </button>

            <div class="glass-stage relative h-[320px] flex-1 overflow-hidden">
                <Canvas>
                    <CharacterPreview classId={current.id} />
                </Canvas>
                <div class="pointer-events-none absolute bottom-3 left-0 right-0 text-center">
                    <div class="font-upheaval text-2xl tracking-wider text-white drop-shadow-[0_2px_3px_rgba(0,0,0,0.8)]">
                        {current.label}
                    </div>
                    <div class="mt-0.5 text-xs italic text-white/70">
                        {current.description}
                    </div>
                </div>
            </div>

            <button
                type="button"
                class="slider-arrow"
                onclick={next}
                aria-label="Next character"
            >
                <ChevronRight size={28} strokeWidth={2.5} />
            </button>
        </div>

        <div class="mb-4 flex justify-center gap-1.5">
            {#each CHARACTER_CLASSES as klass, i (klass.id)}
                <button
                    type="button"
                    class="h-2 rounded-full transition-all {i === index
                        ? 'w-6 bg-white'
                        : 'w-2 bg-white/20 hover:bg-white/40'}"
                    onclick={() => (index = i)}
                    aria-label={klass.label}
                ></button>
            {/each}
        </div>

        <div class="mb-4">
            <label class="mb-1.5 block text-[10px] uppercase tracking-[0.2em] text-white/55" for="player-name">Nom du héros</label>
            <input
                id="player-name"
                class="glass-input w-full"
                type="text"
                placeholder="ex. Astaroth"
                bind:value={name}
                disabled={submitting}
                onkeydown={(e) => e.key === 'Enter' && submit()}
            />
        </div>

        {#if game.error}
            <div class="glass-alert mb-3">
                {game.error}
            </div>
        {/if}

        <button
            type="button"
            class="glass-button w-full"
            onclick={submit}
            disabled={submitting || !name.trim()}
        >
            {submitting ? 'Connexion…' : 'Entrer dans le donjon'}
        </button>
    </div>
</div>

<style>
    .glass-frame {
        background: rgba(20, 16, 28, 0.32);
        backdrop-filter: blur(24px) saturate(170%);
        -webkit-backdrop-filter: blur(24px) saturate(170%);
        border: 1px solid rgba(255, 255, 255, 0.14);
        border-radius: 20px;
        box-shadow:
            0 1px 0 rgba(255, 255, 255, 0.12) inset,
            0 0 0 1px rgba(255, 255, 255, 0.04) inset,
            0 24px 60px rgba(0, 0, 0, 0.55);
    }

    .glass-stage {
        background: linear-gradient(180deg, rgba(255, 255, 255, 0.05), rgba(255, 255, 255, 0.02));
        border: 1px solid rgba(255, 255, 255, 0.10);
        border-radius: 14px;
        box-shadow:
            0 1px 0 rgba(255, 255, 255, 0.08) inset,
            0 6px 18px rgba(0, 0, 0, 0.3);
    }

    .slider-arrow {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 48px;
        background: rgba(255, 255, 255, 0.06);
        backdrop-filter: blur(10px);
        -webkit-backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.14);
        border-radius: 12px;
        color: #ffffff;
        box-shadow:
            0 1px 0 rgba(255, 255, 255, 0.10) inset,
            0 4px 14px rgba(0, 0, 0, 0.25);
        transition: background 0.15s ease, transform 0.08s ease, border-color 0.15s ease;
    }
    .slider-arrow:hover {
        background: rgba(255, 255, 255, 0.14);
        border-color: rgba(255, 255, 255, 0.32);
    }
    .slider-arrow:active {
        transform: translateY(1px);
    }

    .glass-input {
        background: rgba(255, 255, 255, 0.05);
        backdrop-filter: blur(10px);
        -webkit-backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.14);
        border-radius: 10px;
        color: #ffffff;
        padding: 0.65rem 0.85rem;
        font-size: 0.9rem;
        transition: border-color 0.15s ease, background 0.15s ease, box-shadow 0.15s ease;
    }
    .glass-input:focus {
        outline: none;
        background: rgba(255, 255, 255, 0.09);
        border-color: rgba(255, 255, 255, 0.35);
        box-shadow:
            0 0 0 3px rgba(255, 255, 255, 0.08);
    }

    .glass-button {
        background: rgba(255, 255, 255, 0.14);
        backdrop-filter: blur(10px);
        -webkit-backdrop-filter: blur(10px);
        color: #ffffff;
        font-weight: 500;
        letter-spacing: 0.02em;
        padding: 0.8rem 1rem;
        border: 1px solid rgba(255, 255, 255, 0.26);
        border-radius: 12px;
        box-shadow:
            0 1px 0 rgba(255, 255, 255, 0.14) inset,
            0 8px 22px rgba(0, 0, 0, 0.3);
        transition: background 0.15s ease, transform 0.08s ease, border-color 0.15s ease;
    }
    .glass-button:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.22);
        border-color: rgba(255, 255, 255, 0.4);
    }
    .glass-button:active:not(:disabled) {
        transform: translateY(1px);
    }
    .glass-button:disabled {
        opacity: 0.45;
        cursor: not-allowed;
    }

    .glass-alert {
        background: rgba(180, 60, 60, 0.18);
        backdrop-filter: blur(10px);
        -webkit-backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 120, 120, 0.35);
        border-radius: 10px;
        color: #ffd6d6;
        padding: 0.55rem 0.8rem;
        font-size: 0.85rem;
    }
</style>

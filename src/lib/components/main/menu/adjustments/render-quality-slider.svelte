<script lang="ts">
  import type { RenderQualitySliderProps } from "../types";

  import { ClockIcon } from "@lucide/svelte";

  import { Slider } from "$lib/components/ui/slider";
  import { qualityPresets } from "$lib/constants";
  import { cn } from "$lib/utils";

  let { value = $bindable([50]) }: RenderQualitySliderProps = $props();

  const sliderId = `render-slider-${Math.random().toString(36).substring(2, 9)}`;

  const currentPreset = $derived(
    qualityPresets.find(p => p.value === value[0]) || qualityPresets[2],
  );
</script>

<div class="space-y-12 flex flex-col items-center">
  <div class="flex items-center justify-between w-full">
    <div class="flex items-center gap-4">
      <div class="p-3 bg-sky-500/10 rounded-2xl shadow-sm border border-sky-500/10">
        <ClockIcon class="size-5 text-sky-600 dark:text-sky-400" />
      </div>
      <label for={sliderId} class="text-lg font-bold text-neutral-700 dark:text-neutral-200 tracking-tight">Render Quality</label>
    </div>
    <div class="px-4 py-1.5 rounded-full bg-secondary border border-border shadow-sm">
      <span class="text-[10px] uppercase tracking-[0.2em] text-sky-600 dark:text-sky-400 font-bold">{currentPreset.label}</span>
    </div>
  </div>

  <div class="space-y-8 w-full">
    <Slider type="multiple"
            bind:value
            min={0}
            max={100}
            step={25}
            aria-label="Render quality control" />

    <div class="flex justify-between items-start pt-2">
      {#each qualityPresets as preset}
        <button type="button"
                class={cn(
                  "flex flex-col items-center gap-3 text-[11px] transition-all duration-300 uppercase tracking-tighter",
                  (value[0] === preset.value) && "text-primary font-medium scale-110",
                  (value[0] !== preset.value) && "text-neutral-400 font-light opacity-60 hover:opacity-100",
                )}
                onclick={() => value = [preset.value]}>
          <span>{preset.label}</span>
          <span class="text-[10px] opacity-60 font-mono tracking-normal">{preset.time}</span>
        </button>
      {/each}
    </div>
  </div>

  <p class="text-xs font-light text-neutral-400 leading-relaxed max-w-2xl">
    Processing precision for Glaze, Nightshade, and Noise algorithms. High quality increases protection efficacy but requires more computation time.
  </p>
</div>

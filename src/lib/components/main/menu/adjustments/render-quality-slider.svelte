<script lang="ts">
  import type { RenderQualitySliderProps } from "../types";

  import { Slider } from "$lib/components/ui/slider";
  import { qualityPresets } from "$lib/constants";
  import { cn } from "$lib/utils";

  let { value = $bindable([50]) }: RenderQualitySliderProps = $props();

  const sliderId = `render-slider-${Math.random().toString(36).substring(2, 9)}`;

  const currentPreset = $derived(
    qualityPresets.find(p => p.value === value[0]) || qualityPresets[2],
  );
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <label for={sliderId} class="text-sm font-medium text-neutral-600 dark:text-neutral-300">Render Quality</label>
    <span class="text-xs font-light text-neutral-400 uppercase tracking-widest">{currentPreset.label}</span>
  </div>

  <div class="space-y-6">
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
                  "flex flex-col items-center gap-2 text-[10px] transition-all duration-300 uppercase tracking-tighter",
                  (value[0] === preset.value) && "text-primary font-medium scale-110",
                  (value[0] !== preset.value) && "text-neutral-400 font-light opacity-60 hover:opacity-100",
                )}
                onclick={() => value = [preset.value]}>
          <span>{preset.label}</span>
          <span class="text-[9px] opacity-60 font-mono tracking-normal">{preset.time}</span>
        </button>
      {/each}
    </div>
  </div>

  <p class="text-[11px] font-light text-neutral-400 leading-relaxed max-w-md">
    Processing precision for Glaze, Nightshade, and Noise algorithms. High quality increases protection efficacy but requires more computation time.
  </p>
</div>

<script lang="ts">
  import type { IntensitySliderProps } from "../types";

  import { Slider } from "$lib/components/ui/slider";

  let { value = $bindable([30]) }: IntensitySliderProps = $props();

  const sliderId = `intensity-slider-${Math.random().toString(36).substring(2, 9)}`;

  const intensityDisplay = $derived((value[0] / 100).toFixed(2));
</script>

<div class="space-y-10">
  <div class="flex items-center justify-between">
    <label for={sliderId} class="text-base font-medium text-neutral-600 dark:text-neutral-300">Intensity</label>
    <span class="text-sm font-light text-neutral-400 font-mono tracking-[0.2em]">{intensityDisplay}</span>
  </div>

  <div class="space-y-6">
    <Slider id={sliderId}
            type="multiple"
            bind:value
            min={1}
            max={50}
            step={1}
            aria-label="Intensity control" />

    <div class="flex justify-between items-center text-[10px] font-light text-neutral-400 uppercase tracking-[0.3em] opacity-60">
      <span>Subtle</span>
      <span>Balanced</span>
      <span>Aggressive</span>
    </div>
  </div>
</div>

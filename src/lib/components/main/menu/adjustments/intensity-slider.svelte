<script lang="ts">
  import type { IntensitySliderProps } from "../types";

  import { GaugeIcon } from "@lucide/svelte";

  import { Slider } from "$lib/components/ui/slider";

  let { value = $bindable([30]) }: IntensitySliderProps = $props();

  const sliderId = `intensity-slider-${Math.random().toString(36).substring(2, 9)}`;

  const intensityDisplay = $derived((value[0] / 100).toFixed(2));
</script>

<div class="space-y-12 flex flex-col items-center">
  <div class="flex items-center justify-between w-full">
    <div class="flex items-center gap-4">
      <div class="p-3 bg-amber-500/10 rounded-2xl shadow-sm border border-amber-500/10">
        <GaugeIcon class="size-5 text-amber-600 dark:text-amber-400" />
      </div>
      <label for={sliderId} class="text-lg font-bold text-neutral-700 dark:text-neutral-200 tracking-tight">Intensity</label>
    </div>
    <div class="px-4 py-1.5 rounded-full bg-secondary border border-border shadow-sm">
      <span class="text-sm font-bold text-amber-600 dark:text-amber-400 font-mono tracking-widest">{intensityDisplay}</span>
    </div>
  </div>

  <div class="space-y-6 w-full">
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

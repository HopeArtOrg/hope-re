<script lang="ts">
  import type { IntensitySliderProps } from "../types";

  import { GaugeIcon } from "@lucide/svelte";

  import { Slider } from "$lib/components/ui/slider";
  import { cn } from "$lib/utils";

  let { value = $bindable([30]) }: IntensitySliderProps = $props();

  const sliderId = `intensity-slider-${Math.random().toString(36).substring(2, 9)}`;

  const intensityDisplay = $derived((value[0] / 100).toFixed(2));

  const intensityLevel = $derived(
    value[0] < 15
      ? "Low"
      : value[0] < 35
      ? "Medium"
      : "High",
  );
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="p-2.5 doodle-blob bg-card border-2 border-foreground/10 bg-amber-500/10">
        <GaugeIcon class="size-5 text-amber-600 dark:text-amber-400" />
      </div>
      <label for={sliderId} class="text-base font-bold text-foreground/70">Protect Power</label>
    </div>
    <div class="flex items-center gap-2">
      <span class={cn(
        "text-xs font-bold uppercase tracking-wider doodle-line border-2 px-2 py-0.5",
        intensityLevel === "Low" && "text-emerald-700 border-emerald-500/30",
        intensityLevel === "Medium" && "text-amber-700 border-amber-500/30",
        intensityLevel === "High" && "text-rose-700 border-rose-500/30",
      )}>
        {intensityLevel === "Low" ? "Low Ink" : intensityLevel === "Medium" ? "Nice" : "FULL INK"}
      </span>
      <span class="text-sm text-muted-foreground/60 font-bold">
        {intensityDisplay}
      </span>
    </div>
  </div>

  <div class="py-2">
    <Slider id={sliderId}
            type="multiple"
            bind:value
            min={1}
            max={50}
            step={1}
            aria-label="Intensity control" />
  </div>

  <p class="text-sm text-muted-foreground/70 font-bold px-1 leading-tight">
    Controls how thick the "ink" is. Higher power means stronger protection, but more visible changes to your sketch.
  </p>
</div>

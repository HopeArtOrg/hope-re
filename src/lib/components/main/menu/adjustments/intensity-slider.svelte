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
      <div class="p-2 rounded-lg bg-card shadow-sm border border-foreground/5 bg-amber-500/5">
        <GaugeIcon class="size-4 text-amber-500/60 dark:text-amber-400/60" />
      </div>
      <label for={sliderId} class="text-sm font-medium text-muted-foreground/80">Guidance Focus</label>
    </div>
    <div class="flex items-center gap-2">
      <span class={cn(
        "text-xs font-medium uppercase tracking-wider",
        intensityLevel === "Low" && "text-emerald-500/60 dark:text-emerald-400/60",
        intensityLevel === "Medium" && "text-amber-500/60 dark:text-amber-400/60",
        intensityLevel === "High" && "text-rose-500/60 dark:text-rose-400/60",
      )}>
        {intensityLevel}
      </span>
      <span class="text-xs text-muted-foreground/60 font-medium">
        {intensityDisplay}
      </span>
    </div>
  </div>

  <div class="py-1">
    <Slider id={sliderId}
            type="multiple"
            bind:value
            min={1}
            max={50}
            step={1}
            aria-label="Intensity control" />
  </div>

  <p class="text-xs text-muted-foreground">
    Controls the strength of protection. Higher values &equals; stronger protection but more visible changes.
  </p>
</div>

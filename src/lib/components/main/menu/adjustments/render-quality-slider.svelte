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

<div class="space-y-3">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <div class="p-1.5 rounded-lg bg-sky-500/10">
        <ClockIcon class="size-4 text-sky-600 dark:text-sky-400" />
      </div>
      <label for={sliderId} class="text-sm font-medium">Render Quality</label>
    </div>
    <div class="flex items-center gap-2">
      <currentPreset.icon class={cn("size-3.5", currentPreset.colour)} />
      <span class={cn("text-xs font-medium uppercase tracking-wider", currentPreset.colour)}>
        {currentPreset.label}
      </span>
    </div>
  </div>

  <p class="text-xs text-muted-foreground/80 leading-relaxed">
    Processing time for Glaze &sol; Nightshade &sol; Noise. Higher &equals; better quality but slower.
  </p>

  <div class="space-y-4 py-1">
    <Slider type="multiple"
            bind:value
            min={0}
            max={100}
            step={25}
            aria-label="Render quality control" />

    <div class="flex justify-between items-start">
      {#each qualityPresets as preset}
        {@const Icon = preset.icon}
        <button type="button"
                class={cn(
                  "flex flex-col items-center gap-1.5 text-xs transition-opacity group",
                  (value[0] === preset.value) && "opacity-100",
                  (value[0] !== preset.value) && "opacity-30 hover:opacity-50",
                )}
                onclick={() => value = [preset.value]}>
          <Icon class={cn("size-3.5", preset.colour)} />
          <span class={cn("text-[10px] font-medium uppercase tracking-tighter whitespace-nowrap", preset.colour)}>
            {preset.label}
          </span>
        </button>
      {/each}
    </div>
  </div>

  <div class="p-3 rounded-lg bg-muted/20 border border-foreground/5">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <currentPreset.icon class={cn("size-4", currentPreset.colour)} />
        <span class={cn("text-sm font-medium", currentPreset.colour)}>
          {currentPreset.label}
        </span>
      </div>
      <span class="text-xs text-muted-foreground/60 font-medium italic">
        {currentPreset.time}
      </span>
    </div>
  </div>
</div>

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

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="p-2.5 doodle-blob bg-card border-2 border-foreground/10 bg-sky-500/10">
        <ClockIcon class="size-5 text-sky-600 dark:text-sky-400" />
      </div>
      <label for={sliderId} class="text-base font-bold text-foreground/70">Patience Level</label>
    </div>
    <div class="flex items-center gap-2">
      <currentPreset.icon class={cn("size-4", currentPreset.colour)} />
      <span class={cn("text-xs font-bold uppercase tracking-wider", currentPreset.colour)}>
        {currentPreset.label}
      </span>
    </div>
  </div>

  <p class="text-sm text-muted-foreground/70 font-bold px-1 leading-tight">
    How much time we spend inking. Higher patience means better quality but takes longer.
  </p>

  <div class="space-y-6 py-2">
    <Slider type="multiple"
            bind:value
            min={0}
            max={100}
            step={25}
            aria-label="Render quality control" />

    <div class="flex justify-between items-start px-2">
      {#each qualityPresets as preset}
        {@const Icon = preset.icon}
        <button type="button"
                class={cn(
                  "flex flex-col items-center gap-2 text-xs transition-all group",
                  (value[0] === preset.value) && "opacity-100 scale-110",
                  (value[0] !== preset.value) && "opacity-30 hover:opacity-50",
                )}
                onclick={() => value = [preset.value]}>
          <div class={cn("p-2 doodle-blob border-2 border-transparent transition-all", (value[0] === preset.value) && "border-foreground/10 bg-white/40")}>
            <Icon class={cn("size-4", preset.colour)} />
          </div>
          <span class={cn("text-[10px] font-bold uppercase tracking-tighter whitespace-nowrap", preset.colour)}>
            {preset.label}
          </span>
        </button>
      {/each}
    </div>
  </div>

  <div class="p-4 doodle-line bg-white/20 dark:bg-black/20 border-2 border-foreground/10 relative">
    <div class="absolute -top-3 -left-2 rotate-[-12deg] text-[10px] bg-sky-500 text-white px-2 py-0.5 shadow-sm doodle-line decorative-doodle">NOTE!</div>
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <currentPreset.icon class={cn("size-5", currentPreset.colour)} />
        <span class={cn("text-base font-bold", currentPreset.colour)}>
          {currentPreset.label}
        </span>
      </div>
      <span class="text-xs text-muted-foreground/60 font-bold italic">
        ~{currentPreset.time}
      </span>
    </div>
  </div>
</div>

<script lang="ts">
  import type { OutputQualitySliderProps } from "../types";

  import { ImageIcon } from "@lucide/svelte";

  import { Slider } from "$lib/components/ui/slider";
  import { cn } from "$lib/utils";

  let { value = $bindable([92]) }: OutputQualitySliderProps = $props();

  const qualityLevel = $derived(
    value[0] < 90
      ? "Standard"
      : value[0] < 95
      ? "High"
      : "Best",
  );
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="p-2.5 doodle-blob bg-card border-2 border-foreground/10 bg-emerald-500/10">
        <ImageIcon class="size-6 text-emerald-600 dark:text-emerald-400" />
      </div>
      <span class="text-2xl font-bold text-foreground/90 tracking-tight">Final Shine</span>
    </div>
    <div class="flex items-center gap-2">
      <span class={cn(
        "text-base font-bold uppercase tracking-wider doodle-line border-2 px-3 py-0.5",
        qualityLevel === "Standard" && "text-amber-700 border-amber-500/40",
        qualityLevel === "High" && "text-emerald-700 border-emerald-500/40",
        qualityLevel === "Best" && "text-sky-700 border-sky-500/40",
      )}>
        {qualityLevel === "Standard" ? "Rough" : qualityLevel === "High" ? "Polished" : "PRISTINE"}
      </span>
      <span class="text-xl text-foreground/70 font-bold">
        {value[0]}
      </span>
    </div>
  </div>

  <div class="py-2">
    <Slider type="multiple"
            bind:value
            min={85}
            max={98}
            step={1} />
  </div>

  <p class="text-base text-foreground/60 font-bold px-1 leading-tight">
    How detailed the final sheet should be. Higher shine looks better but creates a heavier file.
  </p>
</div>

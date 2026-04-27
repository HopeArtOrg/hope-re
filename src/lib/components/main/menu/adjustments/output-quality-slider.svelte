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
      <div class="p-2 rounded-lg bg-card shadow-sm border border-foreground/5 bg-emerald-500/5">
        <ImageIcon class="size-4 text-emerald-500/60 dark:text-emerald-400/60" />
      </div>
      <span class="text-sm font-medium text-muted-foreground/80">Detail Refinement</span>
    </div>
    <div class="flex items-center gap-2">
      <span class={cn(
        "text-xs font-medium uppercase tracking-wider",
        qualityLevel === "Standard" && "text-amber-500/60 dark:text-amber-400/60",
        qualityLevel === "High" && "text-emerald-500/60 dark:text-emerald-400/60",
        qualityLevel === "Best" && "text-sky-500/60 dark:text-sky-400/60",
      )}>
        {qualityLevel}
      </span>
      <span class="text-xs text-muted-foreground/60 font-medium">
        {value[0]}
      </span>
    </div>
  </div>

  <div class="py-1">
    <Slider type="multiple"
            bind:value
            min={85}
            max={98}
            step={1} />
  </div>

  <p class="text-xs text-muted-foreground">
    Image compression quality. Higher values &equals; larger file size but better visual quality.
  </p>
</div>

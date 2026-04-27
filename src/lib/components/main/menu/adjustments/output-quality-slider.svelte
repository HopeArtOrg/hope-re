<script lang="ts">
  import type { OutputQualitySliderProps } from "../types";

  import { ImageIcon } from "@lucide/svelte";

  import { Slider } from "$lib/components/ui/slider";

  let { value = $bindable([92]) }: OutputQualitySliderProps = $props();
</script>

<div class="space-y-10">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-neutral-100 dark:bg-neutral-900 zen-dashed rounded-none">
        <ImageIcon class="size-4 opacity-70 text-emerald-600 dark:text-emerald-400" />
      </div>
      <span class="text-base font-medium text-neutral-600 dark:text-neutral-300">Output Quality</span>
    </div>
    <span class="text-sm font-light text-neutral-400 font-mono tracking-[0.2em]">{value[0]}%</span>
  </div>

  <div class="space-y-6">
    <Slider type="multiple"
            bind:value
            min={85}
            max={98}
            step={1} />

    <div class="flex justify-between items-center text-[10px] font-light text-neutral-400 uppercase tracking-[0.3em] opacity-60">
      <span>Compressed</span>
      <span>Balanced</span>
      <span>Lossless</span>
    </div>
  </div>
</div>

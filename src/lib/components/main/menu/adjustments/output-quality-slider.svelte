<script lang="ts">
  import type { OutputQualitySliderProps } from "../types";

  import { Slider } from "$lib/components/ui/slider";

  let { value = $bindable([92]) }: OutputQualitySliderProps = $props();
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <span class="text-sm font-medium text-neutral-600 dark:text-neutral-300">Output Quality</span>
    <span class="text-xs font-light text-neutral-400 font-mono tracking-widest">{value[0]}%</span>
  </div>

  <div class="space-y-4">
    <Slider type="multiple"
            bind:value
            min={85}
            max={98}
            step={1} />

    <div class="flex justify-between items-center text-[9px] font-light text-neutral-400 uppercase tracking-widest opacity-60">
      <span>Compressed</span>
      <span>Balanced</span>
      <span>Lossless</span>
    </div>
  </div>
</div>

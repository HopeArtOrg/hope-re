<script lang="ts">
  import type { OutputQualitySliderProps } from "../types";

  import { ImageIcon } from "@lucide/svelte";

  import { Slider } from "$lib/components/ui/slider";

  let { value = $bindable([92]) }: OutputQualitySliderProps = $props();
</script>

<div class="space-y-12">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-4">
      <div class="p-3 bg-emerald-500/10 rounded-2xl shadow-sm border border-emerald-500/10">
        <ImageIcon class="size-5 text-emerald-600 dark:text-emerald-400" />
      </div>
      <span class="text-lg font-bold text-neutral-700 dark:text-neutral-200 tracking-tight">Output Quality</span>
    </div>
    <div class="px-4 py-1.5 rounded-full bg-secondary border border-border shadow-sm">
      <span class="text-sm font-bold text-emerald-600 dark:text-emerald-400 font-mono tracking-widest">{value[0]}%</span>
    </div>
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

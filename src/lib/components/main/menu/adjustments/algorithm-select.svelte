<script lang="ts">
  import type { AlgorithmSelectProps } from "../types";

  import * as Select from "$lib/components/ui/select";
  import { algorithms } from "$lib/constants";

  let { value = $bindable("noise") }: AlgorithmSelectProps = $props();

  const triggerContent = $derived(
    algorithms.find(algo => algo.value === value)?.label ?? "Select an algorithm",
  );
</script>

<div class="space-y-8">
  <div class="flex items-center justify-between">
    <span class="text-base font-medium text-neutral-600 dark:text-neutral-300">Algorithm</span>
    <span class="text-[11px] uppercase tracking-[0.2em] text-neutral-400 font-light">Target Protection</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full h-16 bg-transparent border-neutral-100 dark:border-neutral-900 rounded-none text-base font-light tracking-widest hover:bg-neutral-50 dark:hover:bg-neutral-900/50 transition-all duration-500" aria-label="Algorithm Selection">
      {triggerContent}
    </Select.Trigger>
    <Select.Content class="rounded-none border-neutral-100 dark:border-neutral-900 bg-background/95 backdrop-blur-xl">
      {#each algorithms as algo}
        <Select.Item value={algo.value} class="rounded-none focus:bg-neutral-50 dark:focus:bg-neutral-900 py-4">
          <div class="flex-1">
            <p class="font-light text-sm tracking-wide">{algo.label}</p>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>

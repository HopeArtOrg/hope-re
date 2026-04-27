<script lang="ts">
  import type { AlgorithmSelectProps } from "../types";

  import * as Select from "$lib/components/ui/select";
  import { algorithms } from "$lib/constants";
  import { cn } from "$lib/utils";

  let { value = $bindable("noise") }: AlgorithmSelectProps = $props();

  const triggerContent = $derived(
    algorithms.find(algo => algo.value === value)?.label ?? "Select an algorithm",
  );

  const currentAlgo = $derived(
    algorithms.find(algo => algo.value === value) ?? algorithms[0],
  );
</script>

<div class="space-y-8">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-neutral-100 dark:bg-neutral-900 zen-dashed rounded-none">
        <currentAlgo.icon class={cn("size-4 opacity-70", currentAlgo.colour)} />
      </div>
      <span class="text-base font-medium text-neutral-600 dark:text-neutral-300">Algorithm</span>
    </div>
    <span class="text-[11px] uppercase tracking-[0.2em] text-neutral-400 font-light">Target Protection</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full h-16 bg-transparent border-neutral-100 dark:border-neutral-900 rounded-none text-base font-light tracking-widest hover:bg-neutral-50 dark:hover:bg-neutral-900/50 transition-all duration-500 hover:zen-dashed" aria-label="Algorithm Selection">
      {triggerContent}
    </Select.Trigger>
    <Select.Content class="rounded-none border-neutral-100 dark:border-neutral-900 bg-background/95 backdrop-blur-xl">
      {#each algorithms as algo}
        <Select.Item value={algo.value} class="rounded-none focus:bg-neutral-50 dark:focus:bg-neutral-900 py-4">
          <div class="flex items-center gap-4">
            <algo.icon class={cn("size-4 opacity-40", algo.colour)} />
            <div class="flex flex-col gap-0.5">
              <span class="font-light text-sm tracking-wide">{algo.label}</span>
              <span class="text-[10px] uppercase tracking-wider text-neutral-400 opacity-70">{algo.description}</span>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>

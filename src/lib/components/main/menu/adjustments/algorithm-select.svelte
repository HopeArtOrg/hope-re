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

<div class="space-y-10">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-4">
      <div class="p-3 bg-indigo-500/10 rounded-2xl shadow-sm border border-indigo-500/10">
        <currentAlgo.icon class={cn("size-5", currentAlgo.colour)} />
      </div>
      <span class="text-lg font-bold text-neutral-700 dark:text-neutral-200 tracking-tight">Algorithm</span>
    </div>
    <div class="px-4 py-1.5 rounded-full bg-secondary border border-border shadow-sm">
      <span class="text-[10px] uppercase tracking-[0.3em] text-muted-foreground font-bold">Safety Mode</span>
    </div>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full h-20 bg-card/50 border-2 border-transparent rounded-[2.5rem] text-lg font-medium tracking-wide hover:bg-card hover:border-primary/20 hover:scale-[1.01] active:scale-[0.99] transition-all duration-300 shadow-sm zen-dashed" aria-label="Algorithm Selection">
      {triggerContent}
    </Select.Trigger>
    <Select.Content class="rounded-[2rem] border-border bg-background/95 backdrop-blur-xl p-2 shadow-2xl">
      {#each algorithms as algo}
        <Select.Item value={algo.value} class="rounded-[1.5rem] focus:bg-secondary py-5 px-6">
          <div class="flex items-center gap-5">
            <div class={cn("p-2 rounded-xl", algo.bgColour)}>
              <algo.icon class={cn("size-5", algo.colour)} />
            </div>
            <div class="flex flex-col gap-1">
              <span class="font-bold text-base tracking-tight">{algo.label}</span>
              <span class="text-xs font-medium text-muted-foreground opacity-80">{algo.description}</span>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>

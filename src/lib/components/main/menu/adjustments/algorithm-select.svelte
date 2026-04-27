<script lang="ts">
  import type { AlgorithmSelectProps } from "../types";

  import { PaletteIcon } from "@lucide/svelte";

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

<div class="space-y-4">
  <div class="flex items-center gap-3">
    <div class={cn("p-2 rounded-lg bg-card shadow-sm border border-foreground/5", currentAlgo.bgColour)}>
      <PaletteIcon class={cn("size-4", currentAlgo.colour)} />
    </div>
    <span class="text-sm font-medium text-muted-foreground/80">Creative Idea</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full px-4 h-12 hover:bg-muted/5 transition-all duration-300 bg-muted/5 border-none shadow-inner" aria-label="Creative Idea Selection">
      {triggerContent}
    </Select.Trigger>
    <Select.Content>
      {#each algorithms as algo}
        <Select.Item value={algo.value}>
          <div class="flex items-center gap-3 py-1">
            <div class={cn("p-1.5 rounded", algo.bgColour)}>
              <PaletteIcon class={cn("size-4", algo.colour)} />
            </div>
            <div class="flex-1">
              <p class="font-medium text-sm">{algo.label}</p>
              <p class="text-xs text-muted-foreground">{algo.description}</p>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>

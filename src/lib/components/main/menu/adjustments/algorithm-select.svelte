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
    <div class={cn("p-2.5 doodle-blob bg-card border-2 border-foreground/10", currentAlgo.bgColour)}>
      <PaletteIcon class={cn("size-5", currentAlgo.colour)} />
    </div>
    <span class="text-xl font-bold text-foreground/80">Creative Idea</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full text-lg" aria-label="Creative Idea Selection">
      {triggerContent}
    </Select.Trigger>
    <Select.Content>
      {#each algorithms as algo}
        <Select.Item value={algo.value}>
          <div class="flex items-center gap-4 py-2">
            <div class={cn("p-2 doodle-blob border border-foreground/5", algo.bgColour)}>
              <PaletteIcon class={cn("size-6", algo.colour)} />
            </div>
            <div class="flex-1">
              <p class="font-bold text-lg">{algo.label}</p>
              <p class="text-sm text-muted-foreground/80">{algo.description}</p>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>

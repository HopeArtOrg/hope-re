<script lang="ts">
  import type { NightshadeTargetSelectProps } from "../types";

  import * as Select from "$lib/components/ui/select";
  import { nightshadeTargets } from "$lib/constants";

  let { value = $bindable("dog") }: NightshadeTargetSelectProps = $props();

  const contentTrigger = $derived(
    nightshadeTargets.find(t => t.value === value)?.label ?? "Select a target",
  );
</script>

<div class="space-y-8">
  <div class="flex items-center justify-between">
    <span class="text-base font-medium text-neutral-600 dark:text-neutral-300">Poison Target</span>
    <span class="text-[11px] uppercase tracking-[0.2em] text-neutral-400 font-light">Concept Misclassification</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full h-16 bg-transparent border-neutral-100 dark:border-neutral-900 rounded-none text-base font-light tracking-widest hover:bg-neutral-50 dark:hover:bg-neutral-900/50 transition-all duration-500 hover:zen-dashed" aria-label="Nightshade target selection">
      {contentTrigger}
    </Select.Trigger>
    <Select.Content class="rounded-none border-neutral-100 dark:border-neutral-900 bg-background/95 backdrop-blur-xl">
      {#each nightshadeTargets as target}
        <Select.Item value={target.value} class="rounded-none focus:bg-neutral-50 dark:focus:bg-neutral-900 py-4">
          <div class="flex flex-col gap-0.5">
            <span class="font-light text-sm tracking-wide">{target.label}</span>
            <span class="text-[10px] uppercase tracking-wider text-neutral-400 opacity-70">{target.description}</span>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>

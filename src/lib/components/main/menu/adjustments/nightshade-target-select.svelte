<script lang="ts">
  import type { NightshadeTargetSelectProps } from "../types";

  import { CrosshairIcon } from "@lucide/svelte";

  import * as Select from "$lib/components/ui/select";
  import { nightshadeTargets } from "$lib/constants";

  let { value = $bindable("dog") }: NightshadeTargetSelectProps = $props();

  const contentTrigger = $derived(
    nightshadeTargets.find(t => t.value === value)?.label ?? "Select a target",
  );
</script>

<div class="space-y-10 flex flex-col items-center">
  <div class="flex items-center justify-between w-full">
    <div class="flex items-center gap-4">
      <div class="p-3 bg-rose-500/10 rounded-2xl shadow-sm border border-rose-500/10">
        <CrosshairIcon class="size-5 text-rose-600 dark:text-rose-400" />
      </div>
      <span class="text-lg font-bold text-neutral-700 dark:text-neutral-200 tracking-tight">Poison Target</span>
    </div>
    <div class="px-4 py-1.5 rounded-full bg-secondary border border-border shadow-sm">
      <span class="text-[10px] uppercase tracking-[0.3em] text-muted-foreground font-bold">Poison Concept</span>
    </div>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full h-20 bg-card/50 border-2 border-transparent rounded-2xl text-lg font-medium tracking-wide hover:bg-card hover:border-primary/20 hover:scale-[1.01] active:scale-[0.99] transition-all duration-300 shadow-sm zen-dashed" aria-label="Nightshade target selection">
      {contentTrigger}
    </Select.Trigger>
    <Select.Content class="rounded-2xl border-border bg-background/95 backdrop-blur-xl p-2 shadow-2xl">
      {#each nightshadeTargets as target}
        <Select.Item value={target.value} class="rounded-xl focus:bg-secondary py-5 px-6">
          <div class="flex items-center gap-5">
            <div class="p-2 rounded-xl bg-rose-500/10">
              <target.icon class="size-5 text-rose-600 dark:text-rose-400" />
            </div>
            <div class="flex flex-col gap-1">
              <span class="font-bold text-base tracking-tight">{target.label}</span>
              <span class="text-xs font-medium text-muted-foreground opacity-80">{target.description}</span>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>

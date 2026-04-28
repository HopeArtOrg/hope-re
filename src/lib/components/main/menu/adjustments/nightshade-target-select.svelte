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

<div class="space-y-4">
  <div class="flex items-center gap-3">
    <div class="p-2.5 doodle-blob bg-card border-2 border-foreground/10 bg-rose-500/10">
      <CrosshairIcon class="size-5 text-rose-600 dark:text-rose-400" />
    </div>
    <span class="text-base font-bold text-foreground/70 tracking-tight">Target Concept</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full" aria-label="Target Concept Selection">
      {contentTrigger}
    </Select.Trigger>
    <Select.Content>
      {#each nightshadeTargets as target}
        <Select.Item value={target.value}>
          <div class="flex items-center gap-4 py-2">
            <div class="p-2 doodle-blob border border-foreground/5 bg-muted/20">
              <target.icon class="size-5 text-muted-foreground" />
            </div>
            <div class="flex flex-col gap-0.5">
              <span class="font-bold text-base">{target.label}</span>
              <span class="text-xs text-muted-foreground/80">{target.description}</span>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>

  <p class="text-sm text-muted-foreground/70 font-bold px-1 leading-tight">
    AI models will think your sketch is this concept instead. Sneaky!
  </p>
</div>

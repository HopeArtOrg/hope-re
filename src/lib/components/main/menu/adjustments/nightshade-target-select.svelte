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

<div class="space-y-3">
  <div class="flex items-center gap-2">
    <div class="p-1.5 rounded-lg bg-rose-500/5">
      <CrosshairIcon class="size-4 text-rose-500/60 dark:text-rose-400/60" />
    </div>
    <span class="text-sm font-medium">Poison Target</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full hover:border-foreground/10 transition-colors bg-muted/10 border-foreground/5" aria-label="Nightshade target selection">
      {contentTrigger}
    </Select.Trigger>
    <Select.Content>
      {#each nightshadeTargets as target}
        <Select.Item value={target.value}>
          <div class="flex items-center gap-3 py-1">
            <target.icon class="size-4 text-muted-foreground/60" />
            <div class="flex flex-col gap-0.5">
              <span class="font-medium text-sm">{target.label}</span>
              <span class="text-xs text-muted-foreground/60">{target.description}</span>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>

  <p class="text-xs text-muted-foreground">
    AI models will misclassify your image as this target concept.
  </p>
</div>

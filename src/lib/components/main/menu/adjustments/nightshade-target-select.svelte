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
    <div class="p-2 rounded-lg bg-card shadow-sm border border-foreground/5 bg-rose-500/5">
      <CrosshairIcon class="size-4 text-rose-500/60 dark:text-rose-400/60" />
    </div>
    <span class="text-sm font-medium text-muted-foreground/80">Style Texture</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full px-4 h-12 hover:bg-muted/5 transition-all duration-300 bg-muted/5 border-none shadow-inner" aria-label="Style Texture Selection">
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

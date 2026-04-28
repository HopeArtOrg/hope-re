<script lang="ts">
  import type { GlazeStyleSelectProps } from "../types";

  import { TargetIcon } from "@lucide/svelte";

  import * as Select from "$lib/components/ui/select";
  import { glazeStyles } from "$lib/constants";

  let { value = $bindable("abstract") }: GlazeStyleSelectProps = $props();

  const contentTrigger = $derived(
    glazeStyles.find(style => style.value === value)?.label ?? "Select a style",
  );
</script>

<div class="space-y-4">
  <div class="flex items-center gap-3">
    <div class="p-2.5 doodle-blob bg-card border-2 border-foreground/10 bg-amber-500/10">
      <TargetIcon class="size-5 text-amber-600 dark:text-amber-400" />
    </div>
    <span class="text-base font-bold text-foreground/70 tracking-tight">Style Vibe</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full" aria-label="Style Vibe Selection">
      {contentTrigger}
    </Select.Trigger>
    <Select.Content>
      {#each glazeStyles as style}
        <Select.Item value={style.value}>
          <div class="flex items-center gap-4 py-2">
            <div class="p-2 doodle-blob border border-foreground/5 bg-muted/20">
              <style.icon class="size-5 text-muted-foreground" />
            </div>
            <div class="flex flex-col gap-0.5">
              <span class="font-bold text-base">{style.label}</span>
              <span class="text-xs text-muted-foreground/80">{style.description}</span>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>

  <p class="text-sm text-muted-foreground/70 font-bold px-1 leading-tight">
    Your sheet will be slightly shifted toward this vibe to confuse AI eyes.
  </p>
</div>

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
    <div class="p-2 rounded-lg bg-card shadow-sm border border-foreground/5 bg-amber-500/5">
      <TargetIcon class="size-4 text-amber-500/60 dark:text-amber-400/60" />
    </div>
    <span class="text-sm font-medium text-muted-foreground/80">Style Texture</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full px-4 h-12 hover:bg-muted/5 transition-all duration-300 bg-muted/5 border-none shadow-inner" aria-label="Style Texture Selection">
      {contentTrigger}
    </Select.Trigger>
    <Select.Content>
      {#each glazeStyles as style}
        <Select.Item value={style.value}>
          <div class="flex items-center gap-3 py-1">
            <style.icon class="size-4 text-muted-foreground/60" />
            <div class="flex flex-col gap-0.5">
              <span class="font-medium text-sm">{style.label}</span>
              <span class="text-xs text-muted-foreground/60">{style.description}</span>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>

  <p class="text-xs text-muted-foreground">
    Your image will be shifted toward this art style to confuse AI scrapers.
  </p>
</div>

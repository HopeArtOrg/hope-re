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

<div class="space-y-10">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-4">
      <div class="p-3 bg-amber-500/10 rounded-2xl shadow-sm border border-amber-500/10">
        <TargetIcon class="size-5 text-amber-600 dark:text-amber-400" />
      </div>
      <span class="text-lg font-bold text-neutral-700 dark:text-neutral-200 tracking-tight">Target Style</span>
    </div>
    <div class="px-4 py-1.5 rounded-full bg-secondary border border-border shadow-sm">
      <span class="text-[10px] uppercase tracking-[0.3em] text-muted-foreground font-bold">Artistic Vibe</span>
    </div>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full h-20 bg-card/50 border-2 border-transparent rounded-[2.5rem] text-lg font-medium tracking-wide hover:bg-card hover:border-primary/20 hover:scale-[1.01] active:scale-[0.99] transition-all duration-300 shadow-sm zen-dashed" aria-label="Glaze style selection">
      {contentTrigger}
    </Select.Trigger>
    <Select.Content class="rounded-[2rem] border-border bg-background/95 backdrop-blur-xl p-2 shadow-2xl">
      {#each glazeStyles as style}
        <Select.Item value={style.value} class="rounded-[1.5rem] focus:bg-secondary py-5 px-6">
          <div class="flex items-center gap-5">
            <div class="p-2 rounded-xl bg-amber-500/10">
              <style.icon class="size-5 text-amber-600 dark:text-amber-400" />
            </div>
            <div class="flex flex-col gap-1">
              <span class="font-bold text-base tracking-tight">{style.label}</span>
              <span class="text-xs font-medium text-muted-foreground opacity-80">{style.description}</span>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>

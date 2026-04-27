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

<div class="space-y-8">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-neutral-100 dark:bg-neutral-900 zen-dashed rounded-none">
        <TargetIcon class="size-4 opacity-70 text-amber-600 dark:text-amber-400" />
      </div>
      <span class="text-base font-medium text-neutral-600 dark:text-neutral-300">Target Style</span>
    </div>
    <span class="text-[11px] uppercase tracking-[0.2em] text-neutral-400 font-light">Artistic Shift</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full h-16 bg-transparent border-neutral-100 dark:border-neutral-900 rounded-none text-base font-light tracking-widest hover:bg-neutral-50 dark:hover:bg-neutral-900/50 transition-all duration-500 hover:zen-dashed" aria-label="Glaze style selection">
      {contentTrigger}
    </Select.Trigger>
    <Select.Content class="rounded-none border-neutral-100 dark:border-neutral-900 bg-background/95 backdrop-blur-xl">
      {#each glazeStyles as style}
        <Select.Item value={style.value} class="rounded-none focus:bg-neutral-50 dark:focus:bg-neutral-900 py-4">
          <div class="flex items-center gap-4">
            <style.icon class="size-4 opacity-40 text-amber-600 dark:text-amber-400" />
            <div class="flex flex-col gap-0.5">
              <span class="font-light text-sm tracking-wide">{style.label}</span>
              <span class="text-[10px] uppercase tracking-wider text-neutral-400 opacity-70">{style.description}</span>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>

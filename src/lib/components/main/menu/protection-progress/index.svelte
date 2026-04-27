<script lang="ts">
  import type { ProtectionProgressProps } from "../types";

  import {
    CircleAlertIcon,
    CircleCheckBigIcon,
    LoaderCircleIcon,
  } from "@lucide/svelte";

  import { Progress } from "$lib/components/ui/progress";
  import { cn } from "$lib/utils";

  const {
    isProcessing = false,
    progress = 0,
    status = "idle",
    message = "",
  }: ProtectionProgressProps = $props();

  const statusColour = $derived(
    status === "processing"
      ? "text-indigo-600 dark:text-indigo-400"
      : status === "success"
      ? "text-emerald-600 dark:text-emerald-400"
      : status === "error"
      ? "text-destructive"
      : "text-muted-foreground",
  );
</script>

{#if isProcessing || status !== "idle"}
  <div class="space-y-4 py-6 animate-in fade-in slide-in-from-top-4 duration-700">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        {#if status === "processing"}
          <LoaderCircleIcon class="size-3.5 text-primary opacity-50 animate-spin" />
          <span class="text-[11px] font-light uppercase tracking-[0.2em] text-neutral-500">Processing</span>
        {:else if status === "success"}
          <CircleCheckBigIcon class="size-3.5 text-emerald-500 opacity-70" />
          <span class="text-[11px] font-light uppercase tracking-[0.2em] text-emerald-500">Protection Complete</span>
        {:else if status === "error"}
          <CircleAlertIcon class="size-3.5 text-rose-500 opacity-70" />
          <span class="text-[11px] font-light uppercase tracking-[0.2em] text-rose-500">Pipeline Error</span>
        {/if}
      </div>

      <span class="text-xs font-light font-mono text-neutral-400">{progress}%</span>
    </div>

    <Progress value={progress} class="h-[1px] bg-neutral-100 dark:bg-neutral-900" />

    {#if message}
      <p class={cn("text-[10px] font-light tracking-wide uppercase opacity-60", statusColour)}>
        {message}
      </p>
    {/if}
  </div>
{/if}

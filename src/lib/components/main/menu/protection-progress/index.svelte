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
      ? "text-indigo-500/60 dark:text-indigo-400/60"
      : status === "success"
      ? "text-emerald-500/60 dark:text-emerald-400/60"
      : status === "error"
      ? "text-destructive/60"
      : "text-muted-foreground/40",
  );
</script>

{#if isProcessing || status !== "idle"}
  <div class="space-y-3 p-4 rounded-lg border border-foreground/5 bg-muted/10 animate-in fade-in slide-in-from-top-2 duration-300">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        {#if status === "processing"}
          <LoaderCircleIcon class="size-4 text-indigo-500/60 dark:text-indigo-400/60 animate-spin" />
          <span class="text-sm font-medium text-indigo-500/60 dark:text-indigo-400/60">Protecting...</span>
        {:else if status === "success"}
          <CircleCheckBigIcon class="size-4 text-emerald-500/60 dark:text-emerald-400/60" />
          <span class="text-sm font-medium text-emerald-500/60 dark:text-emerald-400/60">Complete</span>
        {:else if status === "error"}
          <CircleAlertIcon class="size-4 text-destructive/60" />
          <span class="text-sm font-medium text-destructive/60">Error</span>
        {/if}
      </div>

      <span class="text-xs text-muted-foreground/60 font-medium">
        {progress}&percnt;
      </span>
    </div>

    <Progress value={progress} class="h-1.5" />

    {#if message}
      <p class={cn("text-[10px] uppercase tracking-wider font-medium opacity-60", statusColour)}>
        {message}
      </p>
    {/if}
  </div>
{/if}

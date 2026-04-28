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
  <div class="space-y-4 p-6 doodle-line border-2 border-foreground/10 bg-white shadow-xl animate-in fade-in slide-in-from-bottom-2 duration-500 relative">
    <div class="absolute -top-3 left-4 rotate-[5deg] text-[10px] bg-amber-400 text-amber-900 px-2 py-0.5 shadow-sm doodle-line font-bold decorative-doodle">LIVE STATUS</div>

    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        {#if status === "processing"}
          <LoaderCircleIcon class="size-5 text-amber-600 animate-spin" />
          <span class="text-base font-bold text-amber-700">Protecting...</span>
        {:else if status === "success"}
          <CircleCheckBigIcon class="size-5 text-emerald-600" />
          <span class="text-base font-bold text-emerald-700">Art is Safe!</span>
        {:else if status === "error"}
          <CircleAlertIcon class="size-5 text-destructive" />
          <span class="text-base font-bold text-destructive">Ink Spill! (Error)</span>
        {/if}
      </div>

      <span class="text-sm text-muted-foreground font-bold">
        {progress}%
      </span>
    </div>

    <Progress value={progress} />

    {#if message}
      <p class={cn("text-xs font-bold opacity-80", statusColour)}>
        {message}
      </p>
    {/if}
  </div>
{/if}

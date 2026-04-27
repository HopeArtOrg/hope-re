<script lang="ts">
  import type { BaseImagePlaceholderProps } from "./types";

  import { ImageIcon, UploadIcon } from "@lucide/svelte";
  import { toast } from "svelte-sonner";

  import {
    ACCEPT_IMAGE,
    Root as FileDropZone,
    Trigger as FileDropZoneTrigger,
  } from "$lib/components/ui/file-drop-zone";
  import { cn } from "$lib/utils";

  const {
    imageSrc = null,
    label,
    onUpload,
    readonly = false,
    children,
  }: BaseImagePlaceholderProps = $props();

  function handleFileRejected({ reason, file }: { reason: string; file: File }) {
    toast.error(`${file.name}: ${reason}`);
  }
</script>

<div class="flex flex-col h-full">
  {#if !readonly && onUpload}
    <FileDropZone onUpload={onUpload}
                  maxFiles={1}
                  fileCount={0}
                  accept={ACCEPT_IMAGE}
                  onFileRejected={handleFileRejected}>
      <FileDropZoneTrigger class="flex-1">
        <div class={cn(
          "relative w-full aspect-square overflow-hidden transition-all duration-700",
          !imageSrc && "cursor-pointer hover:bg-neutral-50/50 dark:hover:bg-neutral-900/30",
          imageSrc ? "bg-neutral-100 dark:bg-neutral-900/50" : "bg-neutral-50/30 dark:bg-neutral-950/20 zen-dashed-border",
        )}>
          {#if !imageSrc}
            <div class="absolute inset-0 opacity-40 dark:opacity-20 bg-[radial-gradient(circle_at_center,var(--color-primary)_0%,transparent_70%)]"></div>
          {/if}

          {#if imageSrc}
            <div class="absolute inset-0 flex items-center justify-center p-8">
              <img src={imageSrc}
                   alt={label}
                   class="relative z-10 max-w-full max-h-full object-contain shadow-2xl shadow-black/5" />
            </div>
          {:else}
            <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center">
              <UploadIcon class="size-8 text-neutral-300 dark:text-neutral-700 mb-4 font-light" />
              <p class="text-[11px] uppercase tracking-[0.2em] font-light text-neutral-400">
                <span class="hidden md:inline">Drop to Load</span>
                <span class="md:hidden">Select Image</span>
              </p>
            </div>
          {/if}
        </div>
      </FileDropZoneTrigger>
    </FileDropZone>
  {:else}
    <div class={cn(
      "relative w-full aspect-square overflow-hidden transition-all duration-700",
      imageSrc ? "bg-neutral-100 dark:bg-neutral-900/50" : "bg-neutral-50/30 dark:bg-neutral-950/20 zen-dashed-border",
    )}>
      {#if !imageSrc}
        <div class="absolute inset-0 opacity-40 dark:opacity-20 bg-[radial-gradient(circle_at_center,var(--color-primary)_0%,transparent_70%)]"></div>
      {/if}

      {#if imageSrc}
        <div class="absolute inset-0 flex items-center justify-center p-8">
          <img src={imageSrc}
               alt={label}
               class="relative z-10 max-w-full max-h-full object-contain shadow-2xl shadow-black/5" />
        </div>

        {@render children?.()}
      {:else}
        <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center">
          <ImageIcon class="size-8 text-neutral-200 dark:text-neutral-800 mb-4" />
          <p class="text-[11px] uppercase tracking-[0.2em] font-light text-neutral-400">
            Pending Result
          </p>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .zen-dashed-border {
    background-image:
      repeating-linear-gradient(0deg, var(--color-border), var(--color-border) 8px, transparent 8px, transparent 16px),
      repeating-linear-gradient(90deg, var(--color-border), var(--color-border) 8px, transparent 8px, transparent 16px),
      repeating-linear-gradient(180deg, var(--color-border), var(--color-border) 8px, transparent 8px, transparent 16px),
      repeating-linear-gradient(270deg, var(--color-border), var(--color-border) 8px, transparent 8px, transparent 16px);
    background-size: 1px 100%, 100% 1px, 1px 100%, 100% 1px;
    background-position: 0 0, 0 0, 100% 0, 0 100%;
    background-repeat: no-repeat;
    opacity: 0.8;
  }

  .zen-dashed-border:hover {
    opacity: 1;
  }
</style>

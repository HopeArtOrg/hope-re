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
          !imageSrc && "cursor-pointer hover:bg-neutral-100/50 dark:hover:bg-neutral-900/50",
          imageSrc ? "bg-neutral-100 dark:bg-neutral-900/50" : "bg-neutral-50/20 dark:bg-neutral-950/10 zen-dashed-container",
        )}>
          {#if imageSrc}
            <div class="absolute inset-0 flex items-center justify-center p-8">
              <img src={imageSrc}
                   alt={label}
                   class="relative z-10 max-w-full max-h-full object-contain shadow-2xl shadow-black/5" />
            </div>
          {:else}
            <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center">
              <UploadIcon class="size-8 text-neutral-300 dark:text-neutral-700 mb-4 font-light opacity-50" />
              <p class="text-[10px] uppercase tracking-[0.3em] font-light text-neutral-400">
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
      imageSrc ? "bg-neutral-100 dark:bg-neutral-900/50" : "bg-neutral-50/20 dark:bg-neutral-950/10 zen-dashed-container",
    )}>
      {#if imageSrc}
        <div class="absolute inset-0 flex items-center justify-center p-8">
          <img src={imageSrc}
               alt={label}
               class="relative z-10 max-w-full max-h-full object-contain shadow-2xl shadow-black/5" />
        </div>

        {@render children?.()}
      {:else}
        <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center">
          <ImageIcon class="size-8 text-neutral-200 dark:text-neutral-800 mb-4 opacity-50" />
          <p class="text-[10px] uppercase tracking-[0.3em] font-light text-neutral-400">
            Pending Result
          </p>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .zen-dashed-container::after {
    content: "";
    position: absolute;
    inset: 0;
    pointer-events: none;
    padding: 1px;
    background: linear-gradient(135deg,
      var(--color-border) 0%,
      transparent 25%,
      transparent 75%,
      var(--color-border) 100%
    );
    -webkit-mask:
      repeating-linear-gradient(0deg, black, black 10px, transparent 10px, transparent 20px) left / 1px 100% no-repeat,
      repeating-linear-gradient(90deg, black, black 10px, transparent 10px, transparent 20px) top / 100% 1px no-repeat,
      repeating-linear-gradient(180deg, black, black 10px, transparent 10px, transparent 20px) right / 1px 100% no-repeat,
      repeating-linear-gradient(270deg, black, black 10px, transparent 10px, transparent 20px) bottom / 100% 1px no-repeat;
    mask:
      repeating-linear-gradient(0deg, black, black 10px, transparent 10px, transparent 20px) left / 1px 100% no-repeat,
      repeating-linear-gradient(90deg, black, black 10px, transparent 10px, transparent 20px) top / 100% 1px no-repeat,
      repeating-linear-gradient(180deg, black, black 10px, transparent 10px, transparent 20px) right / 1px 100% no-repeat,
      repeating-linear-gradient(270deg, black, black 10px, transparent 10px, transparent 20px) bottom / 100% 1px no-repeat;    opacity: 0.5;
    transition: opacity 0.5s ease;
  }

  .zen-dashed-container:hover::after {
    opacity: 1;
  }
</style>

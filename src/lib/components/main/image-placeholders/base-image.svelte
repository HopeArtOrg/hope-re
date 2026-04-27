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

<div class="flex flex-col h-full items-center justify-center">
  {#if !readonly && onUpload}
    <FileDropZone onUpload={onUpload}
                  maxFiles={1}
                  fileCount={0}
                  accept={ACCEPT_IMAGE}
                  onFileRejected={handleFileRejected}>
      <FileDropZoneTrigger class="flex-1 group w-full">
        <div class={cn(
          "relative w-full aspect-square overflow-hidden transition-all duration-500 rounded-3xl border-2 border-transparent flex items-center justify-center",
          !imageSrc && "cursor-pointer bg-secondary/30 group-hover:bg-secondary/50 group-hover:border-primary/30 group-hover:scale-[1.01] active:scale-[0.99] zen-dashed",
          imageSrc ? "bg-card shadow-inner" : "shadow-sm",
        )}>
          {#if imageSrc}
            <div class="absolute inset-0 flex items-center justify-center p-10">
              <img src={imageSrc}
                   alt={label}
                   class="relative z-10 max-w-full max-h-full object-contain rounded-xl shadow-2xl shadow-black/10 transition-transform duration-500 group-hover:scale-[1.02]" />
            </div>
          {:else}
            <div class="flex flex-col items-center justify-center p-6 text-center">
              <div class="p-6 rounded-full bg-primary/5 mb-6 transition-transform duration-500 group-hover:rotate-12 group-hover:scale-110">
                <UploadIcon class="size-12 text-primary opacity-40 font-light" />
              </div>
              <p class="text-[12px] uppercase tracking-[0.4em] font-bold text-primary/60">
                Load Artwork
              </p>
            </div>
          {/if}
        </div>
      </FileDropZoneTrigger>
    </FileDropZone>
  {:else}
    <div class={cn(
      "relative w-full aspect-square overflow-hidden transition-all duration-500 rounded-3xl border-2 border-transparent flex items-center justify-center",
      imageSrc ? "bg-card shadow-inner" : "bg-secondary/20 zen-dashed",
    )}>
      {#if imageSrc}
        <div class="absolute inset-0 flex items-center justify-center p-10">
          <img src={imageSrc}
               alt={label}
               class="relative z-10 max-w-full max-h-full object-contain rounded-xl shadow-2xl shadow-black/10" />
        </div>

        {@render children?.()}
      {:else}
        <div class="flex flex-col items-center justify-center p-6 text-center">
          <div class="p-6 rounded-full bg-muted/20 mb-6">
            <ImageIcon class="size-12 text-muted-foreground opacity-30" />
          </div>
          <p class="text-[12px] uppercase tracking-[0.4em] font-bold text-muted-foreground/40">
            Pending...
          </p>
        </div>
      {/if}
    </div>
  {/if}
</div>

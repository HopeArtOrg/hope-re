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

  const labelColour = $derived<string>(
    readonly ? "text-emerald-500/60 dark:text-emerald-400/60" : "text-indigo-500/60 dark:text-indigo-400/60",
  );

  const labelBgColour = $derived<string>(
    readonly ? "bg-emerald-500/5" : "bg-indigo-500/5",
  );
</script>

<div class="flex flex-col h-full">
  <div class="mb-3 flex items-center gap-2">
    <div class={cn("p-1.5 rounded-lg", labelBgColour)}>
      <ImageIcon class={cn("size-4", labelColour)} />
    </div>
    <h3 class={cn("text-sm font-medium", labelColour)}>
      {label}
    </h3>
  </div>

  {#if !readonly && onUpload}
    <FileDropZone onUpload={onUpload}
                  maxFiles={1}
                  fileCount={0}
                  accept={ACCEPT_IMAGE}
                  onFileRejected={handleFileRejected}>
      <FileDropZoneTrigger class="flex-1">
        <div class={cn(
          "relative flex-1 rounded-xl border border-solid overflow-hidden bg-muted/20 min-h-[300px] md:min-h-[400px] transition-colors border-foreground/10",
          !imageSrc && "cursor-pointer hover:bg-muted/30",
        )}>
          {#if imageSrc}
            <div class="absolute inset-0 flex items-center justify-center p-4">
              <div class="absolute inset-0 opacity-10"
                   style="
                     background-image:
                     linear-gradient(45deg, #ccc 25%, transparent 25%),
                     linear-gradient(-45deg, #ccc 25%, transparent 25%),
                     linear-gradient(45deg, transparent 75%, #ccc 75%),
                     linear-gradient(-45deg, transparent 75%, #ccc 75%);
                     background-size: 20px 20px;
                     background-position: 0 0, 0 10px, 10px -10px, -10px 0px;
                   "
              ></div>

              <img src={imageSrc}
                   alt={label}
                   class="relative z-10 max-w-full max-h-full object-contain rounded-lg" />
            </div>
          {:else}
            <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center">
              <div class="p-4 rounded-lg mb-3 bg-indigo-500/5">
                <UploadIcon class="size-10 text-indigo-500/40 dark:text-indigo-400/40" />
              </div>
              <p class="text-sm font-medium mb-1 text-muted-foreground/80">
                Upload your image
              </p>
              <p class="text-xs text-muted-foreground/60 mb-4">
                <span class="hidden md:inline">Drag & drop or click to browse</span>
                <span class="md:hidden">Tap to browse image</span>
              </p>
              <div class="flex gap-3 mt-2">
                <span class="text-[10px] uppercase tracking-wider text-muted-foreground/40 font-medium">JPG</span>
                <span class="text-[10px] uppercase tracking-wider text-muted-foreground/40 font-medium">PNG</span>
                <span class="text-[10px] uppercase tracking-wider text-muted-foreground/40 font-medium">WEBP</span>
              </div>
            </div>
          {/if}
        </div>
      </FileDropZoneTrigger>
    </FileDropZone>
  {:else}
    <div class={cn(
      "relative flex-1 rounded-xl border border-solid overflow-hidden bg-muted/20 min-h-[300px] md:min-h-[400px] transition-colors border-foreground/10",
    )}>
      {#if imageSrc}
        <div class="absolute inset-0 flex items-center justify-center p-4">
          <div class="absolute inset-0 opacity-10"
               style="
                 background-image:
                 linear-gradient(45deg, #ccc 25%, transparent 25%),
                 linear-gradient(-45deg, #ccc 25%, transparent 25%),
                 linear-gradient(45deg, transparent 75%, #ccc 75%),
                 linear-gradient(-45deg, transparent 75%, #ccc 75%);
                 background-size: 20px 20px;
                 background-position: 0 0, 0 10px, 10px -10px, -10px 0px;
               "
          ></div>

          <img src={imageSrc}
               alt={label}
               class="relative z-10 max-w-full max-h-full object-contain rounded-lg" />
        </div>

        {@render children?.()}
      {:else}
        <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center">
          <div class="p-4 rounded-lg mb-3 bg-emerald-500/5">
            <ImageIcon class="size-10 text-emerald-500/40 dark:text-emerald-400/40" />
          </div>
          <p class="text-sm font-medium text-muted-foreground/60 mb-1">
            No rendered image yet
          </p>
          <p class="text-xs text-muted-foreground/40">
            Process an image to see the result here
          </p>
        </div>
      {/if}
    </div>
  {/if}
</div>

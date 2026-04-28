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
    containerClass,
  }: BaseImagePlaceholderProps = $props();

  function handleFileRejected({ reason, file }: { reason: string; file: File }) {
    toast.error(`${file.name}: ${reason}`);
  }

  const labelColour = $derived<string>(
    readonly ? "text-emerald-700/60 dark:text-emerald-400/60" : "text-amber-800/60 dark:text-amber-400/60",
  );

  const labelBgColour = $derived<string>(
    readonly ? "bg-emerald-500/10" : "bg-amber-500/10",
  );

  const isSticky = $derived(containerClass?.includes("sticky-note"));
  const isCanvas = $derived(containerClass?.includes("canvas-sheet"));
</script>

<div class={cn("flex flex-col h-full overflow-hidden", containerClass)}>
  {#if isSticky}
    <div class="absolute top-2 left-1/2 -translate-x-1/2 size-4 rounded-full bg-red-500 shadow-inner z-50">
      <div class="absolute inset-1 rounded-full bg-red-400 opacity-50"></div>
    </div>
  {/if}

  {#if isCanvas}
    <div class="absolute top-0 left-10 w-8 h-4 bg-zinc-400 rounded-b-sm shadow-sm z-50"></div>
    <div class="absolute top-0 right-10 w-8 h-4 bg-zinc-400 rounded-b-sm shadow-sm z-50"></div>
  {/if}

  <div class="mb-3 flex items-center gap-2 relative z-10">
    <div class={cn("p-1.5 rounded-lg doodle-blob", labelBgColour)}>
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
          "relative flex-1 rounded-xl border-2 border-dashed overflow-hidden min-h-[300px] md:min-h-[400px] transition-colors border-foreground/10",
          !imageSrc && "cursor-pointer hover:bg-black/5",
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
                   class="relative z-10 max-w-full max-h-full object-contain rounded-lg shadow-sm" />
            </div>
          {:else}
            <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center">
              <div class="p-4 rounded-lg mb-3 bg-amber-500/10">
                <UploadIcon class="size-10 text-amber-600/40 dark:text-amber-400/40" />
              </div>
              <p class="text-sm font-medium mb-1 text-muted-foreground/80">
                Sketch your image here
              </p>
              <p class="text-xs text-muted-foreground/60 mb-4">
                <span class="hidden md:inline">Drop a sheet or click to browse</span>
                <span class="md:hidden">Tap to browse image</span>
              </p>
            </div>
          {/if}
        </div>
      </FileDropZoneTrigger>
    </FileDropZone>
  {:else}
    <div class={cn(
      "relative flex-1 rounded-xl border-2 border-dashed overflow-hidden min-h-[300px] md:min-h-[400px] transition-colors border-foreground/10",
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
               class="relative z-10 max-w-full max-h-full object-contain rounded-lg shadow-sm" />
        </div>

        {@render children?.()}
      {:else}
        <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center">
          <div class="p-4 rounded-lg mb-3 bg-emerald-500/10">
            <ImageIcon class="size-10 text-emerald-600/40 dark:text-emerald-400/40" />
          </div>
          <p class="text-sm font-medium text-muted-foreground/60 mb-1">
            Canvas is blank
          </p>
          <p class="text-xs text-muted-foreground/40">
            Start drawing to see the result
          </p>
        </div>
      {/if}
    </div>
  {/if}
</div>

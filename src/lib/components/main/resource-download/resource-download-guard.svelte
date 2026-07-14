<script lang="ts">
  import type { Snippet } from "svelte";

  import { DownloadIcon, LoaderCircleIcon, MinusIcon, TriangleAlertIcon } from "@lucide/svelte";
  import { onMount } from "svelte";

  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Progress } from "$lib/components/ui/progress";
  import { MODEL_ALGO_SUFFIX } from "$lib/constants";
  import { t } from "$lib/stores/use-i18n.svelte";
  import { useModelDownload } from "$lib/stores/use-model-download.svelte";
  import { cn } from "$lib/utils";

  type ResourceDownloadGuardProps = {
    children: Snippet;
  };

  const { children }: ResourceDownloadGuardProps = $props();

  const models = useModelDownload();
  let mounted = $state<boolean>(false);

  onMount(() => {
    mounted = true;
  });

  $effect(() => {
    if (mounted && !models.isLoading && !models.allReady && !models.isDownloading && !models.error) {
      models.startDownload();
    }
  });

  function formatBytes(bytes: number): string {
    if (bytes === 0)
      return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / k ** i).toFixed(1)} ${sizes[i]}`;
  }

  function handleInteractOutside(e: Event) {
    if (models.isDownloading) {
      e.preventDefault();
    }
  }

  function handleEscapeKeydown(e: Event) {
    if (models.isDownloading) {
      e.preventDefault();
      models.minimize();
    }
  }
</script>

{@render children()}

<Dialog.Root
  open={models.dialogOpen}
  onOpenChange={() => {}}
>
  <Dialog.Content
    class="max-w-md"
    showCloseButton={false}
    onInteractOutside={handleInteractOutside}
    onEscapeKeydown={handleEscapeKeydown}
  >
    {#if models.isDownloading}
      <button
        onclick={() => models.minimize()}
        class="absolute inset-e-4 top-4 rounded-lg opacity-50 transition-opacity hover:opacity-100 focus-visible:outline-hidden cursor-pointer"
        aria-label={t("resources.minimizeToDock")}
      >
        <MinusIcon class="size-4" />
      </button>
    {/if}
    <Dialog.Header>
      <div class="flex flex-col items-center gap-3">
        <div class="size-12 rounded-lg bg-primary/10 flex items-center justify-center">
          {#if models.error}
            <TriangleAlertIcon class="size-6 text-destructive" />
          {:else}
            <DownloadIcon class="size-6 text-primary" />
          {/if}
        </div>

        <Dialog.DialogTitle class="text-lg font-medium tracking-tight">
          {#if models.isLoading}
            {t("resources.checking")}
          {:else if models.error}
            {t("resources.interrupted")}
          {:else if models.isDownloading}
            {t("resources.preparing")}
          {:else}
            {t("resources.settingUp")}
          {/if}
        </Dialog.DialogTitle>

        <Dialog.DialogDescription class="text-center leading-relaxed max-w-xs">
          {#if models.error}
            {models.error}
          {:else if models.isDownloading && models.currentModelName}
            {t("resources.downloadingModel", { name: models.currentModelName })}
          {:else if models.isLoading}
            {t("resources.verifying")}
          {:else}
            {t("resources.initializing")}
          {/if}
        </Dialog.DialogDescription>
      </div>
    </Dialog.Header>

    {#if models.isDownloading || models.error}
      <div class="space-y-4">
        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <span class="text-xs text-muted-foreground uppercase tracking-wider font-medium">{t("resources.overallProgress")}</span>
            <span class="text-xs text-muted-foreground/60 font-medium tabular-nums">
              {Math.round(models.overallPercent)}&percnt;
            </span>
          </div>
          <Progress value={models.overallPercent}
                    max={100}
                    class="h-1.5" />
        </div>

        <div class="space-y-2">
          {#each models.modelProgress as model (model.name)}
            {const displayName = $derived(model.name.replace(MODEL_ALGO_SUFFIX, ""));}
            <div class="flex items-center gap-3 p-2 rounded-lg bg-muted/20 border border-foreground/5">
              <div class={cn(
                "size-1.5 rounded-full shrink-0",
                model.percent >= 100 ? "bg-emerald-500/40" : model.percent > 0 ? "bg-primary/40 animate-pulse" : "bg-muted-foreground/20",
              )}></div>

              <span class="text-xs text-muted-foreground/80 font-medium capitalize flex-1">
                {displayName}
              </span>

              <span class="text-[10px] text-muted-foreground/40 font-medium tabular-nums uppercase">
                {#if model.percent >= 100}
                  {t("resources.done")}
                {:else if model.totalBytes > 0}
                  {formatBytes(model.downloadedBytes)} / {formatBytes(model.totalBytes)}
                {:else if model.percent > 0}
                  {Math.round(model.percent)}&percnt;
                {:else}
                  {t("resources.waiting")}
                {/if}
              </span>
            </div>
          {/each}
        </div>
      </div>
    {:else if models.isLoading}
      <div class="flex justify-center py-4">
        <LoaderCircleIcon class="size-5 text-muted-foreground animate-spin" />
      </div>
    {/if}

    {#if models.error}
      <Dialog.Footer>
        <Button
          variant="outline"
          size="sm"
          class="gap-2"
          onclick={() => models.startDownload()}
        >
          <DownloadIcon class="size-4" />
          {t("resources.retryDownload")}
        </Button>
      </Dialog.Footer>
    {/if}
  </Dialog.Content>
</Dialog.Root>

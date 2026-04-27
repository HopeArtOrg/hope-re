<script lang="ts">
  import {
    LoaderCircleIcon,
    TriangleAlertIcon,
  } from "@lucide/svelte";
  import { toast } from "svelte-sonner";

  import {
    BaseImagePlaceholder,
    ImageFullscreenDialog,
    ProtectionMenu,
    RenderedImageActions,
  } from "$lib/components";
  import { Button } from "$lib/components/ui/button";
  import { useInferenceCapabilities } from "$lib/queries";
  import { useImage } from "$lib/stores/use-image.svelte";
  import { useProtection } from "$lib/stores/use-protection.svelte";

  const image = useImage();
  const protection = useProtection();

  let isMobile = $state<boolean>(false);

  const {
    data: inferenceData,
    isSuccess,
  } = $derived(useInferenceCapabilities());

  $effect(() => {
    const mediaQuery = window.matchMedia("(max-width: 1023px)");
    isMobile = mediaQuery.matches;

    function handler(e: MediaQueryListEvent) {
      isMobile = e.matches;
    }

    mediaQuery.addEventListener("change", handler);
    return () => mediaQuery.removeEventListener("change", handler);
  });

  function handleProtect() {
    if (!image.originalImage)
      return;

    protection.handleProtect(image.originalImage);
  }

  function handleCancel() {
    if (protection.isProcessing) {
      toast.info("Protection cancelled");
    }

    protection.resetProgress();
    protection.resetSettings();
    image.clear();

    toast.success("All cleared");
  }

  function handleDownload() {
    image.handleDownload(protection.resultImage, protection.algorithm);
  }

  const canProcess = $derived(image.hasImage && !protection.isProcessing);
</script>

<svelte:head>
  <title>Hope:RE</title>
  <meta name="description" content="Protect your images from unauthorized AI training with advanced algorithms like Glaze and Nightshade." />
</svelte:head>

<div class="w-full bg-background selection:bg-primary/20 bg-[radial-gradient(var(--color-secondary)_1px,transparent_1px)] [background-size:32px_32px]">
  <div class="container mx-auto px-8 py-16 h-full max-w-7xl">
    <div class="flex flex-col gap-20 h-full">
      {#if isMobile}
        <div class="flex flex-col gap-16 flex-1 min-h-0">
          {#if protection.hasResult && !protection.modelUsed}
            <div class="flex items-center gap-3 p-5 bg-amber-500/10 rounded-2xl text-xs font-medium text-amber-800 dark:text-amber-200 tracking-tight border border-amber-500/20">
              <TriangleAlertIcon class="size-4 shrink-0 opacity-80" />
              <span>AI models not loaded. Basic fallback protection is active.</span>
            </div>
          {/if}

          {#if protection.resultImage}
            <div class="space-y-6">
              <div class="inline-flex px-4 py-1.5 rounded-full bg-primary/10 border border-primary/20">
                <span class="text-[11px] uppercase tracking-[0.25em] text-primary font-bold">Protected Image</span>
              </div>
              <BaseImagePlaceholder imageSrc={protection.resultImage}
                                    label="Protected Image"
                                    readonly>
                <RenderedImageActions onDownload={handleDownload}
                                      onFullscreen={image.handleFullscreen} />
              </BaseImagePlaceholder>
            </div>
          {:else}
            <div class="space-y-6">
              <div class="inline-flex px-4 py-1.5 rounded-full bg-secondary border border-border">
                <span class="text-[11px] uppercase tracking-[0.25em] text-muted-foreground font-bold">Original Image</span>
              </div>
              <BaseImagePlaceholder imageSrc={image.originalImage}
                                    label="Original Image"
                                    onUpload={image.handleUpload} />
            </div>
          {/if}
        </div>
      {:else}
        {#if protection.hasResult && !protection.modelUsed}
          <div class="flex items-center gap-3 p-5 bg-amber-500/10 rounded-2xl text-xs font-medium text-amber-800 dark:text-amber-200 tracking-tight border border-amber-500/20">
            <TriangleAlertIcon class="size-4 shrink-0 opacity-80" />
            <span>AI models not loaded. Basic fallback protection is active.</span>
          </div>
        {/if}

        <div class="grid grid-cols-2 gap-20 flex-1">
          <div class="space-y-8">
            <div class="inline-flex px-5 py-2 rounded-full bg-secondary border border-border shadow-sm">
              <span class="text-[11px] uppercase tracking-[0.25em] text-muted-foreground font-bold">Original Work</span>
            </div>
            <BaseImagePlaceholder imageSrc={image.originalImage}
                                  label="Original Image"
                                  onUpload={image.handleUpload} />
          </div>

          <div class="space-y-8">
            <div class="inline-flex px-5 py-2 rounded-full bg-primary/10 border border-primary/20 shadow-sm">
              <span class="text-[11px] uppercase tracking-[0.25em] text-primary font-bold">Protected Work</span>
            </div>
            <BaseImagePlaceholder imageSrc={protection.resultImage}
                                  label="Protected Image"
                                  readonly>
              {#if protection.resultImage}
                <RenderedImageActions onDownload={handleDownload}
                                      onFullscreen={image.handleFullscreen} />
              {/if}
            </BaseImagePlaceholder>
          </div>
        </div>
      {/if}

      <div class="w-full max-w-5xl mx-auto bg-card/30 backdrop-blur-sm p-12 rounded-[2rem] border border-border/50">
        <ProtectionMenu bind:algorithm={protection.algorithm}
                        bind:glazeStyle={protection.glazeStyle}
                        bind:nightshadeTarget={protection.nightshadeTarget}
                        bind:intensity={protection.intensity}
                        bind:outputQuality={protection.outputQuality}
                        bind:renderQuality={protection.renderQuality}
                        isProcessing={protection.isProcessing}
                        progress={protection.progress}
                        status={protection.progressStatus}
                        progressMessage={protection.progressMessage} />
      </div>

      {#if isSuccess && inferenceData}
        <div class="text-[10px] text-muted-foreground font-medium tracking-[0.2em] uppercase flex items-center justify-center gap-8 opacity-80">
          <span class="bg-secondary px-3 py-1 rounded-full border border-border">System Active</span>
          <div class="flex flex-wrap gap-6">
            {#each inferenceData.providers as provider}
              <span class="flex items-center gap-2">
                <span class="size-2 bg-primary rounded-full animate-pulse"></span>
                {provider.name}
              </span>
            {/each}
          </div>
        </div>
      {/if}

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-10 pb-16 pt-8 max-w-5xl mx-auto w-full">
        <Button
          size="lg"
          class="h-20 bg-primary text-primary-foreground hover:scale-[1.02] active:scale-[0.98] transition-all duration-300 rounded-3xl border-none shadow-xl shadow-primary/20 text-sm font-bold uppercase tracking-[0.3em] disabled:opacity-30"
          onclick={handleProtect}
          disabled={!canProcess}
        >
          {#if protection.isProcessing}
            <LoaderCircleIcon class="size-5 animate-spin opacity-80 mr-3" />
            <span>Processing</span>
          {:else}
            <span>Protect Artwork</span>
          {/if}
        </Button>

        <Button
          variant="ghost"
          size="lg"
          class="h-20 text-muted-foreground hover:text-foreground hover:bg-secondary/80 transition-all duration-300 rounded-3xl text-sm font-bold uppercase tracking-[0.3em] disabled:opacity-30"
          onclick={handleCancel}
          disabled={!image.hasImage}
        >
          {#if protection.isProcessing}
            <span>Cancel</span>
          {:else}
            <span>Clear Workspace</span>
          {/if}
        </Button>
      </div>
    </div>
  </div>
</div>

<ImageFullscreenDialog bind:open={image.fullscreenOpen}
                       imageSrc={protection.resultImage} />

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

<div class="w-full bg-background selection:bg-primary/10">
  <div class="container mx-auto px-8 py-12 h-full max-w-7xl">
    <div class="flex flex-col gap-16 h-full">
      {#if isMobile}
        <div class="flex flex-col gap-12 flex-1 min-h-0">
          {#if protection.hasResult && !protection.modelUsed}
            <div class="flex items-center gap-3 p-4 bg-amber-500/5 text-xs font-light text-amber-700 dark:text-amber-300 tracking-tight">
              <TriangleAlertIcon class="size-3.5 shrink-0 opacity-70" />
              <span>Basic fallback protection active. Models not loaded.</span>
            </div>
          {/if}

          {#if protection.resultImage}
            <div class="space-y-4">
              <span class="text-[10px] uppercase tracking-[0.2em] text-neutral-400 font-light">Protected Image</span>
              <BaseImagePlaceholder imageSrc={protection.resultImage}
                                    label="Protected Image"
                                    readonly>
                <RenderedImageActions onDownload={handleDownload}
                                      onFullscreen={image.handleFullscreen} />
              </BaseImagePlaceholder>
            </div>
          {:else}
            <div class="space-y-4">
              <span class="text-[10px] uppercase tracking-[0.2em] text-neutral-400 font-light">Original Image</span>
              <BaseImagePlaceholder imageSrc={image.originalImage}
                                    label="Original Image"
                                    onUpload={image.handleUpload} />
            </div>
          {/if}
        </div>
      {:else}
        {#if protection.hasResult && !protection.modelUsed}
          <div class="flex items-center gap-3 p-4 bg-amber-500/5 text-xs font-light text-amber-700 dark:text-amber-300 tracking-tight">
            <TriangleAlertIcon class="size-3.5 shrink-0 opacity-70" />
            <span>Basic fallback protection active. AI models were not loaded.</span>
          </div>
        {/if}

        <div class="grid grid-cols-2 gap-16 flex-1">
          <div class="space-y-4">
            <span class="text-[10px] uppercase tracking-[0.2em] text-neutral-400 font-light pl-1">Original</span>
            <BaseImagePlaceholder imageSrc={image.originalImage}
                                  label="Original Image"
                                  onUpload={image.handleUpload} />
          </div>

          <div class="space-y-4">
            <span class="text-[10px] uppercase tracking-[0.2em] text-neutral-400 font-light pl-1">Protected</span>
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

      <div class="w-full max-w-3xl mx-auto">
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
        <div class="text-[9px] text-neutral-400 font-light tracking-widest uppercase flex items-center gap-6 opacity-60 px-2">
          <span>Inference Pipeline</span>
          <div class="flex flex-wrap gap-4">
            {#each inferenceData.providers as provider}
              <span class="flex items-center gap-1.5">
                {provider.name}
                <span class="size-1 bg-emerald-500/50 rounded-full"></span>
              </span>
            {/each}
          </div>
        </div>
      {/if}

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-8 pb-12 pt-4">
        <Button
          size="lg"
          class="h-16 bg-primary text-primary-foreground hover:opacity-90 transition-all duration-500 rounded-none border-none shadow-none text-xs font-medium uppercase tracking-[0.3em] disabled:opacity-20"
          onclick={handleProtect}
          disabled={!canProcess}
        >
          {#if protection.isProcessing}
            <LoaderCircleIcon class="size-4 animate-spin opacity-50 mr-2" />
            <span>Processing</span>
          {:else}
            <span>Protect Image</span>
          {/if}
        </Button>

        <Button
          variant="ghost"
          size="lg"
          class="h-16 text-neutral-400 hover:text-neutral-900 dark:hover:text-neutral-100 hover:bg-neutral-100 dark:hover:bg-neutral-900 transition-all duration-500 rounded-none text-xs font-light uppercase tracking-[0.3em] disabled:opacity-20"
          onclick={handleCancel}
          disabled={!image.hasImage}
        >
          {#if protection.isProcessing}
            <span>Cancel</span>
          {:else}
            <span>Clear</span>
          {/if}
        </Button>
      </div>
    </div>
  </div>
</div>

<ImageFullscreenDialog bind:open={image.fullscreenOpen}
                       imageSrc={protection.resultImage} />

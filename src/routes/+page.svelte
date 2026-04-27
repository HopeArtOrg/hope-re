<script lang="ts">
  import {
    CpuIcon,
    LoaderCircleIcon,
    RotateCcwIcon,
    ShieldIcon,
    TriangleAlertIcon,
    XIcon,
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

<div class="w-full bg-background transition-colors duration-500">
  <div class="container mx-auto p-6 md:p-10 h-full max-w-7xl">
    <div class="flex flex-col gap-10 h-full">
      {#if isMobile}
        <div class="flex flex-col gap-8 flex-1 min-h-0">
          {#if protection.hasResult && !protection.modelUsed}
            <div class="flex items-center gap-3 p-4 rounded-xl panel-float text-sm text-amber-600/70 dark:text-amber-400/70 animate-in fade-in duration-700">
              <TriangleAlertIcon class="size-4 shrink-0" />
              <span>Basic protection applied. Add AI models in settings for a more refined result.</span>
            </div>
          {/if}

          {#if protection.resultImage}
            <div class="panel-float p-1 rounded-2xl">
              <BaseImagePlaceholder imageSrc={protection.resultImage}
                                    label="Rendered Canvas"
                                    readonly>
                <RenderedImageActions onDownload={handleDownload}
                                      onFullscreen={image.handleFullscreen} />
              </BaseImagePlaceholder>
            </div>
          {:else}
            <div class="panel-float p-1 rounded-2xl">
              <BaseImagePlaceholder imageSrc={image.originalImage}
                                    label="Original Canvas"
                                    onUpload={image.handleUpload} />
            </div>
          {/if}
        </div>
      {:else}
        {#if protection.hasResult && !protection.modelUsed}
          <div class="flex items-center gap-3 p-4 rounded-xl panel-float text-sm text-amber-600/70 dark:text-amber-400/70 animate-in fade-in duration-700">
            <TriangleAlertIcon class="size-4 shrink-0" />
            <span>Basic protection applied. Add AI models in settings for a more refined result.</span>
          </div>
        {/if}

        <div class="grid grid-cols-2 gap-10 flex-1">
          <div class="panel-float p-1 rounded-2xl">
            <BaseImagePlaceholder imageSrc={image.originalImage}
                                  label="Original Canvas"
                                  onUpload={image.handleUpload} />
          </div>

          <div class="panel-float p-1 rounded-2xl">
            <BaseImagePlaceholder imageSrc={protection.resultImage}
                                  label="Rendered Canvas"
                                  readonly>
              {#if protection.resultImage}
                <RenderedImageActions onDownload={handleDownload}
                                      onFullscreen={image.handleFullscreen} />
              {/if}
            </BaseImagePlaceholder>
          </div>
        </div>
      {/if}

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

      {#if isSuccess && inferenceData}
        <div class="flex items-center gap-2 px-2 opacity-30 hover:opacity-100 transition-opacity cursor-help"
             title="Inference Providers: {inferenceData.providers.map(p => p.name).join(", ")}">
          <CpuIcon class="size-3 text-muted-foreground" />
          <div class="text-[10px] text-muted-foreground font-medium uppercase tracking-tighter flex items-center gap-2">
            {#each inferenceData.providers as provider}
              <span class="flex items-center gap-1">
                {provider.name}
                <span class="size-1 rounded-full bg-emerald-500/50"></span>
              </span>
            {/each}
          </div>
        </div>
      {/if}

      <div class="grid grid-cols-2 gap-4 pb-4">
        <Button
          size="lg"
          class="gap-2 h-14 bg-primary hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={handleProtect}
          disabled={!canProcess}
        >
          {#if protection.isProcessing}
            <LoaderCircleIcon class="size-5 animate-spin" />
            <span class="font-medium">Processing...</span>
          {:else}
            <ShieldIcon class="size-5" />
            <span class="font-medium">Protect Image</span>
          {/if}
        </Button>

        <Button
          variant="outline"
          size="lg"
          class="gap-2 h-14 border hover:bg-destructive/10 hover:border-destructive/30 hover:text-destructive transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={handleCancel}
          disabled={!image.hasImage}
        >
          {#if protection.isProcessing}
            <XIcon class="size-5" />
            <span class="font-medium">Cancel</span>
          {:else}
            <RotateCcwIcon class="size-5" />
            <span class="font-medium">Clear</span>
          {/if}
        </Button>
      </div>
    </div>
  </div>
</div>

<ImageFullscreenDialog bind:open={image.fullscreenOpen}
                       imageSrc={protection.resultImage} />

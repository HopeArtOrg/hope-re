<script lang="ts">
  import {
    BrushIcon,
    CpuIcon,
    EraserIcon,
    LoaderCircleIcon,
    NotebookPenIcon,
    PaintbrushIcon,
    PencilIcon,
    RotateCcwIcon,
    ShieldIcon,
    StickyNoteIcon,
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

<div class="w-full bg-background transition-colors duration-500 overflow-x-hidden min-h-screen">
  <div class="container mx-auto p-6 md:p-10 h-full max-w-7xl relative">

    <!-- Desk Decorations -->
    <div class="fixed bottom-10 left-10 opacity-20 decorative-doodle hidden lg:block rotate-12">
      <div class="w-2 h-40 bg-orange-200 rounded-full shadow-md relative">
        <div class="absolute top-0 w-full h-8 bg-black rounded-t-full"></div>
        <div class="absolute bottom-0 w-full h-10 bg-pink-300 rounded-b-full"></div>
      </div>
    </div>

    <div class="fixed top-24 left-12 opacity-10 decorative-doodle hidden xl:block -rotate-6">
      <PencilIcon class="size-32 text-foreground" strokeWidth={1} />
    </div>

    <div class="fixed bottom-24 right-12 opacity-10 decorative-doodle hidden xl:block rotate-12">
      <PaintbrushIcon class="size-40 text-foreground" strokeWidth={1} />
    </div>

    <div class="fixed top-40 right-16 opacity-10 decorative-doodle hidden xl:block -rotate-12">
      <NotebookPenIcon class="size-24 text-foreground" strokeWidth={1} />
    </div>

    <div class="fixed top-20 right-20 opacity-20 decorative-doodle hidden lg:block -rotate-12">
      <div class="w-16 h-8 bg-pink-200 rounded-sm shadow-sm flex items-center justify-center text-xs text-pink-600 font-bold uppercase border-b-2 border-pink-300/50">ERASE</div>
    </div>

    <div class="flex flex-col gap-10 h-full relative z-10">
      {#if isMobile}
        <div class="flex flex-col gap-8 flex-1 min-h-0">
          {#if protection.hasResult && !protection.modelUsed}
            <div class="flex items-center gap-3 p-4 blue-note text-sm text-amber-700 animate-in fade-in duration-700 relative">
              <TriangleAlertIcon class="size-4 shrink-0" />
              <span>Basic protection applied. Add AI models in settings for a more refined result.</span>
              <div class="absolute -top-2 -right-2 rotate-12 text-xs bg-white px-2 py-0.5 shadow-sm border-2 border-foreground/10 doodle-line decorative-doodle">PHEW!</div>
            </div>
          {/if}

          {#if protection.resultImage}
            <BaseImagePlaceholder imageSrc={protection.resultImage}
                                  label="Rendered Canvas"
                                  containerClass="canvas-sheet p-8"
                                  readonly>
              <RenderedImageActions onDownload={handleDownload}
                                    onFullscreen={image.handleFullscreen} />
            </BaseImagePlaceholder>
          {:else}
            <BaseImagePlaceholder imageSrc={image.originalImage}
                                  label="Original Canvas"
                                  containerClass="sticky-note p-8"
                                  onUpload={image.handleUpload} />
          {/if}
        </div>
      {:else}
        {#if protection.hasResult && !protection.modelUsed}
          <div class="flex items-center gap-3 p-4 blue-note text-sm text-amber-700 animate-in fade-in duration-700 max-w-2xl mx-auto w-full relative">
            <TriangleAlertIcon class="size-4 shrink-0" />
            <span>Basic protection applied. Add AI models in settings for a more refined result.</span>
            <div class="absolute -top-2 -right-2 rotate-12 text-xs bg-white px-2 py-0.5 shadow-sm border-2 border-foreground/10 doodle-line uppercase font-bold decorative-doodle">Heads up!</div>
          </div>
        {/if}

        <div class="grid grid-cols-2 gap-10 flex-1">
          <div class="relative group">
            <div class="absolute -top-8 -left-6 rotate-[-5deg] text-base text-muted-foreground/40 font-bold decorative-doodle flex items-center gap-2">
              <BrushIcon class="size-5" />
              Don't forget perspective!
            </div>
            <BaseImagePlaceholder imageSrc={image.originalImage}
                                  label="Original Canvas"
                                  containerClass="sticky-note p-10 h-full"
                                  onUpload={image.handleUpload} />
            <div class="absolute -bottom-4 -left-4 opacity-10 decorative-doodle rotate-12">
              <EraserIcon class="size-12 text-foreground" />
            </div>
          </div>

          <div class="relative">
            <BaseImagePlaceholder imageSrc={protection.resultImage}
                                  label="Rendered Canvas"
                                  containerClass="canvas-sheet p-10 h-full"
                                  readonly>
              {#if protection.resultImage}
                <RenderedImageActions onDownload={handleDownload}
                                      onFullscreen={image.handleFullscreen} />
              {/if}
            </BaseImagePlaceholder>
            <div class="absolute -bottom-4 -right-4 rotate-[15deg] text-base bg-primary text-primary-foreground px-4 py-1 shadow-md border-2 border-primary-foreground/20 doodle-line decorative-doodle">WOW!</div>
            <div class="absolute top-1/2 -right-12 opacity-10 decorative-doodle -rotate-90">
              <StickyNoteIcon class="size-16 text-foreground" />
            </div>
          </div>
        </div>
      {/if}

      <div class="relative">
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
          <div class="absolute -top-14 -left-4 flex items-center gap-3 px-3 opacity-70 hover:opacity-100 transition-opacity cursor-help bg-white/60 backdrop-blur-md p-2 rounded-lg doodle-line border-2 border-foreground/10 shadow-sm"
               title="Inference Providers: {inferenceData.providers.map(p => p.name).join(", ")}">
            <div class="relative group">
              <CpuIcon class="size-5 text-primary/70" />
              <!-- Sleeping Cat Doodle -->
              <div class="absolute -top-8 -left-4 scale-90 opacity-40 group-hover:opacity-100 transition-opacity decorative-doodle">
                <svg width="40"
                     height="40"
                     viewBox="0 0 40 40"
                     fill="none"
                     xmlns="http://www.w3.org/2000/svg">
                  <path d="M10 30Q10 20 20 20Q30 20 30 30"
                        stroke="currentColor"
                        stroke-width="2" />
                  <path d="M12 20L8 15M28 20L32 15"
                        stroke="currentColor"
                        stroke-width="2" />
                  <circle cx="18"
                          cy="25"
                          r="1"
                          fill="currentColor" />
                  <circle cx="22"
                          cy="25"
                          r="1"
                          fill="currentColor" />
                  <path d="M18 28Q20 30 22 28"
                        stroke="currentColor"
                        stroke-width="1.5" />
                </svg>
              </div>
            </div>
            <div class="text-sm text-foreground/80 font-bold flex items-center gap-3">
              {#each inferenceData.providers as provider}
                <span class="flex items-center gap-1.5">
                  {provider.name}
                  <span class="size-2 rounded-full bg-emerald-500/60 doodle-blob"></span>
                </span>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <div class="grid grid-cols-2 gap-6 pb-8">
        <Button
          size="lg"
          class="gap-2 h-16 bg-primary hover:bg-primary/90 transition-all doodle-line text-lg shadow-lg hover:-translate-y-1 active:translate-y-0.5 disabled:opacity-50"
          onclick={handleProtect}
          disabled={!canProcess}
        >
          {#if protection.isProcessing}
            <LoaderCircleIcon class="size-6 animate-spin" />
            <span class="font-bold">Drawing...</span>
          {:else}
            <ShieldIcon class="size-6" />
            <span class="font-bold">Apply Ink</span>
          {/if}
        </Button>

        <Button
          variant="outline"
          size="lg"
          class="gap-2 h-16 border-2 border-foreground/20 hover:bg-destructive/10 hover:border-destructive/30 hover:text-destructive transition-all doodle-line text-lg shadow-md hover:-translate-y-1 active:translate-y-0.5 disabled:opacity-50"
          onclick={handleCancel}
          disabled={!image.hasImage}
        >
          {#if protection.isProcessing}
            <XIcon class="size-6" />
            <span class="font-bold">Stop!</span>
          {:else}
            <RotateCcwIcon class="size-6" />
            <span class="font-bold">New Sheet</span>
          {/if}
        </Button>
      </div>
    </div>
  </div>
</div>

<ImageFullscreenDialog bind:open={image.fullscreenOpen}
                       imageSrc={protection.resultImage} />

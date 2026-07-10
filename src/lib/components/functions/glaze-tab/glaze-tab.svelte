<script lang="ts">
  import {
    BrushIcon,
    EraserIcon,
    StickyNoteIcon,
    TriangleAlertIcon,
  } from "@lucide/svelte";

  import {
    BaseImagePlaceholder,
    RenderedImageActions,
  } from "$lib/components";
  import { useImage } from "$lib/stores/use-image.svelte";
  import { useProtection } from "$lib/stores/use-protection.svelte";

  import GlazeForm from "./glaze-form.svelte";

  const image = useImage();
  const protection = useProtection();

  let isMobile = $state<boolean>(false);

  $effect(() => {
    const mediaQuery = window.matchMedia("(max-width: 1023px)");
    isMobile = mediaQuery.matches;

    function handler(e: MediaQueryListEvent) {
      isMobile = e.matches;
    }

    mediaQuery.addEventListener("change", handler);
    return () => mediaQuery.removeEventListener("change", handler);
  });

  function handleDownload() {
    image.handleDownload(protection.resultImage, protection.algorithm);
  }
</script>

<div class="flex flex-col gap-10 h-full relative z-10 animate-in fade-in duration-500">
  {#if isMobile}
    <div class="flex flex-col gap-8 flex-1 min-h-0">
      {#if protection.hasResult && !protection.modelUsed}
        <div class="flex items-center gap-3 p-4 blue-note text-sm text-amber-700 animate-in fade-in duration-700 relative">
          <TriangleAlertIcon class="size-4 shrink-0" />
          <span>Basic protection applied. Add AI models in settings for a more refined result.</span>
          <div class="absolute -top-2 -right-2 rotate-12 text-xs bg-white text-black px-2 py-0.5 shadow-sm border-2 border-foreground/10 doodle-line decorative-doodle">PHEW!</div>
        </div>
      {/if}

      {#if protection.resultImage}
        <div class="relative group">
          <BaseImagePlaceholder imageSrc={protection.resultImage}
                                label="Rendered Canvas"
                                containerClass="canvas-sheet p-8"
                                readonly>
            <RenderedImageActions onDownload={handleDownload}
                                  onFullscreen={image.handleFullscreen} />
          </BaseImagePlaceholder>
        </div>
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
        <div class="absolute -top-2 -right-2 rotate-12 text-xs bg-white text-black px-2 py-0.5 shadow-sm border-2 border-foreground/10 doodle-line uppercase font-bold decorative-doodle">Heads up!</div>
      </div>
    {/if}

    <div class="grid grid-cols-2 gap-10 flex-1">
      <div class="relative group">
        <div class="absolute -top-8 -left-6 -rotate-5 text-base text-muted-foreground/40 font-bold decorative-doodle flex items-center gap-2">
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

      <div class="relative group">
        <BaseImagePlaceholder imageSrc={protection.resultImage}
                              label="Rendered Canvas"
                              containerClass="canvas-sheet p-10 h-full"
                              readonly>
          {#if protection.resultImage}
            <RenderedImageActions onDownload={handleDownload}
                                  onFullscreen={image.handleFullscreen} />
          {/if}
        </BaseImagePlaceholder>
        <div class="absolute -bottom-4 -right-4 rotate-15 text-base bg-primary text-primary-foreground px-4 py-1 shadow-md border-2 border-primary-foreground/20 doodle-line decorative-doodle">WOW!</div>
        <div class="absolute top-1/2 -right-12 opacity-10 decorative-doodle -rotate-90">
          <StickyNoteIcon class="size-16 text-foreground" />
        </div>
      </div>
    </div>
  {/if}

  <GlazeForm {image} {protection} />
</div>

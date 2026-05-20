<script lang="ts">
  import type { RenderedImageActionsProps } from "./types";

  import { DownloadIcon } from "$lib/components";
  import { Button } from "$lib/components/ui/button";

  const { onDownload, onFullscreen }: RenderedImageActionsProps = $props();
</script>

<div class="absolute inset-0 flex items-center justify-center gap-6 z-20 pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-500">
  <Button variant="ghost"
          size="icon"
          class="size-14 pointer-events-auto doodle-blob bg-white/60 hover:bg-white/80 border-2 border-foreground/10 backdrop-blur-md shadow-lg hover:scale-110"
          onclick={onDownload}
          title="Save Artwork">
    <DownloadIcon class="size-5 text-foreground/80" />
  </Button>

  <Button variant="ghost"
          size="icon"
          class="size-14 pointer-events-auto doodle-blob bg-white/60 hover:bg-white/80 border-2 border-foreground/10 backdrop-blur-md shadow-lg hover:scale-110"
          onclick={onFullscreen}
          title="Inspect Canvas">
    <!-- Magnifying Glass SVG -->
    <svg width="24"
         height="24"
         viewBox="0 0 24 24"
         fill="none"
         xmlns="http://www.w3.org/2000/svg"
         class="text-foreground/80">
      <circle cx="11"
              cy="11"
              r="7"
              stroke="currentColor"
              stroke-width="2" />
      <path d="M16 16L20 20"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round" />
      <path d="M9 11H13M11 9V13"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round" />
    </svg>
  </Button>
</div>

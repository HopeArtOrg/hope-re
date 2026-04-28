<script lang="ts">
  import type { RenderedImageActionsProps } from "./types";

  import { Button } from "$lib/components/ui/button";

  const { onDownload, onFullscreen }: RenderedImageActionsProps = $props();
</script>

<div class="absolute inset-0 flex items-center justify-center gap-6 z-20 pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-500">
  <Button variant="ghost"
          size="icon"
          class="size-14 pointer-events-auto doodle-blob bg-white/60 hover:bg-white/80 border-2 border-foreground/10 backdrop-blur-md shadow-lg hover:scale-110"
          onclick={onDownload}
          title="Save Artwork">
    <!-- Quill SVG -->
    <svg width="24"
         height="24"
         viewBox="0 0 24 24"
         fill="none"
         xmlns="http://www.w3.org/2000/svg"
         class="text-foreground/80">
      <path d="M20 4C18.5 4 14.5 5.5 12.5 9C10.5 12.5 6.5 15.5 4 17.5M4 17.5L6.5 16.5M4 17.5L5 15M12.5 9L14 7.5L16.5 5L18.5 4"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round" />
      <path d="M4 20L6 19"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round" />
    </svg>
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

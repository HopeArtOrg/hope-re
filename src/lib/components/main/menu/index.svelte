<script lang="ts">
  import type { ProtectionMenuProps } from "./types";

  import { SlidersVerticalIcon } from "@lucide/svelte";

  import {
    AlgorithmSelect,
    GlazeStyleSelect,
    IntensitySlider,
    NightshadeTargetSelect,
    OutputQualitySlider,
    RenderQualitySlider,
  } from "./adjustments";
  import { ProtectionProgress } from "./protection-progress";

  let {
    algorithm = $bindable("noise"),
    glazeStyle = $bindable("abstract"),
    nightshadeTarget = $bindable("dog"),
    intensity = $bindable([20]),
    outputQuality = $bindable([92]),
    renderQuality = $bindable([50]),
    isProcessing = false,
    progress = 0,
    status = "idle",
    progressMessage = "",
  }: ProtectionMenuProps = $props();

  const showGlazeStyle = $derived(algorithm === "glaze");
  const showNightshadeTarget = $derived(algorithm === "nightshade");
</script>

<div class="space-y-10">
  <div class="space-y-8 p-10 blue-note relative">
    <!-- Bird Doodle -->
    <div class="absolute -top-6 left-12 scale-110 opacity-60">
      <svg width="30"
           height="30"
           viewBox="0 0 30 30"
           fill="none"
           xmlns="http://www.w3.org/2000/svg">
        <path d="M10 20Q15 15 20 20Q25 15 25 10"
              stroke="currentColor"
              stroke-width="2" />
        <path d="M10 20L8 22M12 20L10 22"
              stroke="currentColor"
              stroke-width="2" />
        <circle cx="18"
                cy="18"
                r="1"
                fill="currentColor" />
      </svg>
    </div>

    <div class="flex items-center gap-4">
      <div class="p-3 rounded-lg bg-primary/10 doodle-blob shadow-sm">
        <SlidersVerticalIcon class="size-5 text-primary/70" />
      </div>
      <h3 class="text-lg font-bold text-foreground/80 tracking-tight">Artistic Guidance</h3>
    </div>

    <div class="space-y-10">
      <AlgorithmSelect bind:value={algorithm} />

      {#if showGlazeStyle}
        <div class="animate-in fade-in slide-in-from-top-2 duration-500">
          <GlazeStyleSelect bind:value={glazeStyle} />
        </div>
      {:else if showNightshadeTarget}
        <div class="animate-in fade-in slide-in-from-top-2 duration-500">
          <NightshadeTargetSelect bind:value={nightshadeTarget} />
        </div>
      {/if}

      <IntensitySlider bind:value={intensity} />

      <div class="grid grid-cols-1 md:grid-cols-2 gap-10">
        <OutputQualitySlider bind:value={outputQuality} />
        <RenderQualitySlider bind:value={renderQuality} />
      </div>
    </div>
  </div>

  <ProtectionProgress {isProcessing}
                      {progress}
                      {status}
                      message={progressMessage} />
</div>

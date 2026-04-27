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

<div class="space-y-8">
  <div class="space-y-8 p-8 rounded-xl panel-float">
    <div class="flex items-center gap-4">
      <div class="p-2.5 rounded-lg bg-muted/10 shadow-sm">
        <SlidersVerticalIcon class="size-5 text-muted-foreground/50" />
      </div>
      <h3 class="text-base font-medium text-muted-foreground/70 tracking-tight">Artistic Guidance</h3>
    </div>

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

    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <OutputQualitySlider bind:value={outputQuality} />
      <RenderQualitySlider bind:value={renderQuality} />
    </div>
  </div>

  <ProtectionProgress {isProcessing}
                      {progress}
                      {status}
                      message={progressMessage} />
</div>

<script lang="ts">
  import type { ProtectionMenuProps } from "./types";

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

<div class="space-y-16 py-12">
  <div class="space-y-12">
    <div class="flex items-center gap-4">
      <h3 class="text-[11px] font-medium tracking-[0.3em] text-neutral-500 uppercase">Protection Settings</h3>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-x-24 gap-y-16">
      <AlgorithmSelect bind:value={algorithm} />

      {#if showGlazeStyle}
        <div class="animate-in fade-in slide-in-from-top-2 duration-500">
          <GlazeStyleSelect bind:value={glazeStyle} />
        </div>
      {:else if showNightshadeTarget}
        <div class="animate-in fade-in slide-in-from-top-2 duration-500">
          <NightshadeTargetSelect bind:value={nightshadeTarget} />
        </div>
      {:else}
        <div class="hidden md:block"></div>
      {/if}

      <IntensitySlider bind:value={intensity} />

      <OutputQualitySlider bind:value={outputQuality} />

      <div class="md:col-span-2 pt-8">
        <RenderQualitySlider bind:value={renderQuality} />
      </div>
    </div>
  </div>

  <ProtectionProgress {isProcessing}
                      {progress}
                      {status}
                      message={progressMessage} />
</div>

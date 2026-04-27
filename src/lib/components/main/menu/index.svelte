<script lang="ts">
  import type { ProtectionMenuProps } from "./types";

  import { SlidersVerticalIcon } from "@lucide/svelte";

  import { Separator } from "$lib/components/ui/separator";

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

<div class="space-y-6">
  <div class="space-y-6 p-6 border border-foreground/5 rounded-xl bg-card/50">
    <div class="flex items-center gap-3">
      <div class="p-2 rounded-lg bg-muted/20">
        <SlidersVerticalIcon class="size-5 text-muted-foreground/60" />
      </div>
      <h3 class="text-base font-medium text-muted-foreground/80">Protection Settings</h3>
    </div>

    <Separator />

    <AlgorithmSelect bind:value={algorithm} />

    <Separator />

    {#if showGlazeStyle}
      <div class="animate-in fade-in slide-in-from-top-2 duration-300">
        <GlazeStyleSelect bind:value={glazeStyle} />
      </div>
    {:else if showNightshadeTarget}
      <div class="animate-in fade-in slide-in-from-top-2 duration-300">
        <NightshadeTargetSelect bind:value={nightshadeTarget} />
      </div>
    {/if}

    {#if showGlazeStyle || showNightshadeTarget}
      <Separator />
    {/if}

    <IntensitySlider bind:value={intensity} />

    <Separator />

    <OutputQualitySlider bind:value={outputQuality} />

    <RenderQualitySlider bind:value={renderQuality} />
  </div>

  <ProtectionProgress {isProcessing}
                      {progress}
                      {status}
                      message={progressMessage} />
</div>

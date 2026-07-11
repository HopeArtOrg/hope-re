<script lang="ts">
  import {
    CpuIcon,
    LoaderCircleIcon,
    RotateCcwIcon,
    ShieldIcon,
    XIcon,
  } from "@lucide/svelte";
  import { toast } from "svelte-sonner";

  import { CatIcon, ProtectionMenu } from "$lib/components";
  import { Button } from "$lib/components/ui/button";
  import { useInferenceCapabilities } from "$lib/queries";
  import { useImage } from "$lib/stores/use-image.svelte";
  import { useProtection } from "$lib/stores/use-protection.svelte";

  type GlazeFormProps = {
    image: ReturnType<typeof useImage>;
    protection: ReturnType<typeof useProtection>;
  };

  const { image, protection }: GlazeFormProps = $props();

  const {
    data: inferenceData,
    isSuccess,
  } = $derived(useInferenceCapabilities());

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

  const canProcess = $derived(image.hasImage && !protection.isProcessing);
</script>

<div class="relative flex flex-col gap-6">
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
      <div class="absolute -top-14 -left-4 flex items-center gap-3 px-3 opacity-70 hover:opacity-100 transition-opacity cursor-help bg-card/60 backdrop-blur-md p-2 rounded-lg doodle-line border-2 border-foreground/10 shadow-sm"
           title="Inference Providers: {inferenceData.providers.map(p => p.name).join(", ")}">
        <div class="relative group">
          <CpuIcon class="size-5 text-primary/70" />
          <div class="absolute -top-10 -left-6 scale-90 opacity-40 group-hover:opacity-100 transition-opacity decorative-doodle">
            <CatIcon class="size-12 text-foreground" />
          </div>
        </div>
        <div class="text-sm text-foreground/80 font-bold flex items-center gap-3">
          {#each inferenceData.providers as provider (provider.name)}
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

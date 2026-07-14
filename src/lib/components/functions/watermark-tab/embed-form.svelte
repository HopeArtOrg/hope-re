<script lang="ts">
  import {
    ChevronDownIcon,
    InfoIcon,
    KeyIcon,
    LoaderCircleIcon,
    RotateCcwIcon,
    ScanLineIcon,
    StampIcon,
    TypeIcon,
  } from "@lucide/svelte";
  import { toast } from "svelte-sonner";

  import {
    BaseImagePlaceholder,
    ImageFullscreenDialog,
    RenderedImageActions,
  } from "$lib/components";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { useEmbedWatermark } from "$lib/queries";
  import { t } from "$lib/stores/use-i18n.svelte";
  import { useImage } from "$lib/stores/use-image.svelte";
  import { useWatermarkState } from "$lib/stores/use-watermark-state.svelte";

  const embedImage = useImage();
  const embedMutation = useEmbedWatermark();
  const wmState = useWatermarkState();

  const SUCCESS_RESET_MS = 3000;

  let watermarkText = $state<string>(t("watermark.embed.defaultText"));
  let useCustomSeed = $state<boolean>(false);
  let seedValue = $state<string>("");
  let embedResultImage = $state<string | null>(null);

  let isProcessing = $state<boolean>(false);
  let progress = $state<number>(0);
  let progressStatus = $state<"idle" | "processing" | "success" | "error">("idle");
  let progressMessage = $state<string>("");

  let runId = 0;
  let resetTimer: ReturnType<typeof setTimeout> | undefined;

  function isValidSeed(value: string): boolean {
    return /^\d+$/.test(value.trim());
  }

  function byteLength(text: string): number {
    return new TextEncoder().encode(text).length;
  }

  function syncScannerState() {
    wmState.scannableImage = embedResultImage;
    wmState.lastWatermarkLength = byteLength(watermarkText);
    wmState.useSeed = useCustomSeed;
    wmState.lastSeed = seedValue;
  }

  async function handleEmbed() {
    if (!embedImage.originalImage) {
      toast.error(t("watermark.embed.uploadToSign"));
      return;
    }

    if (!watermarkText) {
      toast.error(t("watermark.embed.enterText"));
      return;
    }

    if (useCustomSeed && !isValidSeed(seedValue)) {
      toast.error(t("watermark.embed.invalidSeed"));
      return;
    }

    const currentRun = ++runId;
    if (resetTimer) {
      clearTimeout(resetTimer);
    }

    isProcessing = true;
    progressStatus = "processing";

    const steps = [
      { progress: 20, message: t("watermark.embed.stepDecomposing"), duration: 400 },
      { progress: 55, message: t("watermark.embed.stepEmbedding"), duration: 600 },
      { progress: 85, message: t("watermark.embed.stepSynthesizing"), duration: 400 },
    ];

    for (const step of steps) {
      progress = step.progress;
      progressMessage = step.message;
      await new Promise(resolve => setTimeout(resolve, step.duration));
    }

    try {
      progress = 90;
      progressMessage = t("watermark.embed.stepReconstructing");

      const seed = useCustomSeed ? Number(seedValue.trim()) : undefined;
      const result = await embedMutation.mutateAsync({
        imageBase64: embedImage.originalImage,
        watermark: watermarkText,
        seed,
      });

      if (currentRun !== runId) {
        return;
      }

      embedResultImage = result;
      syncScannerState();

      progress = 100;
      progressStatus = "success";
      toast.success(t("watermark.embed.embedded"));
      resetTimer = setTimeout(() => {
        if (progressStatus === "success") {
          progressStatus = "idle";
          progress = 0;
          progressMessage = "";
        }
      }, SUCCESS_RESET_MS);
    }
    catch (error: any) {
      if (currentRun !== runId) {
        return;
      }
      progressStatus = "error";
      toast.error(error instanceof Error ? error.message : String(error));
    }
    finally {
      if (currentRun === runId) {
        isProcessing = false;
      }
    }
  }

  function handleCancel() {
    runId++;
    if (resetTimer) {
      clearTimeout(resetTimer);
    }

    isProcessing = false;
    progress = 0;
    progressStatus = "idle";
    progressMessage = "";

    embedImage.clear();
    embedResultImage = null;
    watermarkText = "";
    useCustomSeed = false;
    seedValue = "";

    toast.success(t("watermark.resetComplete"));
  }

  function handleSendToScanner() {
    syncScannerState();
    wmState.activeSubTab = "extract";
    toast.success(t("watermark.embed.loadedIntoScanner"));
  }

  function handleDownload() {
    embedImage.handleDownload(embedResultImage, "watermark");
  }
</script>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-10">
  <div class="flex flex-col gap-8">
    <div class="relative group">
      <BaseImagePlaceholder
        imageSrc={embedImage.originalImage}
        label={t("placeholders.originalCanvas")}
        containerClass="sticky-note p-8 min-h-[350px]"
        onUpload={embedImage.handleUpload}
      />
    </div>

    <div class="relative group">
      <BaseImagePlaceholder
        imageSrc={embedResultImage}
        label={t("placeholders.signedCanvasOutput")}
        containerClass="canvas-sheet p-8 min-h-[350px]"
        readonly
      >
        {#if embedResultImage}
          <RenderedImageActions
            onDownload={handleDownload}
            onFullscreen={embedImage.handleFullscreen}
          />
        {/if}
      </BaseImagePlaceholder>
    </div>

    {#if embedResultImage}
      <Button
        variant="secondary"
        class="w-full h-12 border-2 border-dashed border-primary/40 text-primary hover:bg-primary/5 transition-all doodle-line text-base font-bold animate-in fade-in slide-in-from-top-2 duration-300"
        onclick={handleSendToScanner}
      >
        <ScanLineIcon class="size-5 mr-2" />
        {t("watermark.embed.testVerification")}
      </Button>
    {/if}
  </div>

  <div class="flex flex-col gap-6">
    <div class="bg-amber-50/70 border-2 border-amber-200/50 dark:bg-amber-950/10 dark:border-amber-900/30 rounded-3xl p-8 flex flex-col gap-6 shadow-xl relative overflow-hidden doodle-line">
      <div class="absolute -right-6 -bottom-6 opacity-[0.04] -rotate-12 text-8xl font-black tracking-widest pointer-events-none select-none text-amber-900 dark:text-amber-100">
        落款
      </div>

      <div class="flex items-center gap-3 bg-linear-to-r from-amber-500/10 to-orange-500/10 text-amber-800 dark:text-amber-300 p-4 rounded-2xl border border-amber-200/30 dark:border-amber-900/20">
        <StampIcon class="size-6 shrink-0" />
        <div>
          <h2 class="text-xl font-bold leading-none">{t("watermark.embed.settingsTitle")}</h2>
          <p class="text-xs opacity-80 mt-1">{t("watermark.embed.settingsSubtitle")}</p>
        </div>
      </div>

      <div class="flex flex-col gap-2">
        <label for="watermark-input" class="text-sm font-bold flex items-center gap-1.5 text-amber-800/80 dark:text-amber-400/80">
          <TypeIcon class="size-4" />
          {t("watermark.embed.signatureText")}
        </label>
        <Input
          id="watermark-input"
          type="text"
          placeholder={t("watermark.embed.signaturePlaceholder")}
          bind:value={watermarkText}
          disabled={isProcessing}
          class="doodle-line border-2 border-amber-200/60 focus-visible:border-amber-500 focus-visible:ring-amber-500/20 dark:border-amber-800/40 h-12 text-base px-4 font-bold bg-background/50 focus:bg-background"
        />
      </div>

      <div class="flex flex-col gap-3">
        <button
          type="button"
          disabled={isProcessing}
          onclick={() => useCustomSeed = !useCustomSeed}
          class="flex items-center justify-between w-full text-left font-bold text-sm text-amber-800/80 dark:text-amber-400/80 hover:opacity-85 transition-opacity cursor-pointer disabled:pointer-events-none"
        >
          <span class="flex items-center gap-1.5">
            <KeyIcon class="size-4" />
            {t("watermark.embed.encryptionKey")}
          </span>
          <ChevronDownIcon class="size-5 transition-transform duration-300 {useCustomSeed ? "rotate-180 text-amber-500" : "text-muted-foreground/60"}" />
        </button>
        {#if useCustomSeed}
          <div class="animate-in slide-in-from-top-2 fade-in duration-300">
            <Input
              id="seed-input"
              type="text"
              placeholder={t("watermark.embed.seedPlaceholder")}
              bind:value={seedValue}
              disabled={isProcessing}
              class="doodle-line border-2 border-amber-200/60 focus-visible:border-amber-500 focus-visible:ring-amber-500/20 dark:border-amber-800/40 h-10 text-base px-4 bg-background/50 font-bold"
            />
          </div>
        {/if}
      </div>

      <div class="mt-2 p-4 bg-amber-500/5 rounded-2xl border border-amber-200/20 dark:border-amber-950/20 text-sm flex gap-3 text-amber-900/70 dark:text-amber-200/70 leading-relaxed">
        <InfoIcon class="size-5 shrink-0 text-amber-600/70 dark:text-amber-400/70" />
        <p>
          {t("watermark.embed.info")}
        </p>
      </div>

      {#if progressStatus === "processing"}
        <div class="flex flex-col gap-2 p-4 bg-amber-500/10 border-2 border-amber-500/20 rounded-xl doodle-line animate-in fade-in duration-300">
          <div class="flex items-center gap-3">
            <LoaderCircleIcon class="size-5 animate-spin text-amber-600 dark:text-amber-400" />
            <span class="text-sm font-bold text-amber-800 dark:text-amber-300">{progressMessage}</span>
          </div>
          <div class="w-full bg-amber-500/10 h-3 rounded-full overflow-hidden border border-amber-200/20">
            <div class="bg-amber-500 h-full transition-all duration-300" style="width: {progress}%"></div>
          </div>
        </div>
      {/if}
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-6 mt-4">
      <Button
        size="lg"
        class="gap-2 h-16 bg-amber-600 hover:bg-amber-500 active:bg-amber-700 text-white transition-all doodle-line text-lg shadow-lg hover:-translate-y-1 active:translate-y-0.5 disabled:opacity-50"
        onclick={handleEmbed}
        disabled={!embedImage.hasImage || !watermarkText || isProcessing}
      >
        {#if isProcessing}
          <LoaderCircleIcon class="size-6 animate-spin" />
          <span class="font-bold">{t("watermark.embed.signing")}</span>
        {:else}
          <StampIcon class="size-6" />
          <span class="font-bold">{t("watermark.embed.signSheet")}</span>
        {/if}
      </Button>

      <Button
        variant="outline"
        size="lg"
        class="gap-2 h-16 border-2 border-foreground/20 hover:bg-destructive/10 hover:border-destructive/30 hover:text-destructive transition-all doodle-line text-lg shadow-md hover:-translate-y-1 active:translate-y-0.5 disabled:opacity-50"
        onclick={handleCancel}
        disabled={!embedImage.hasImage}
      >
        <RotateCcwIcon class="size-6" />
        <span class="font-bold">{t("watermark.resetSheet")}</span>
      </Button>
    </div>
  </div>
</div>

<ImageFullscreenDialog bind:open={embedImage.fullscreenOpen}
                       imageSrc={embedResultImage}
                       onDownload={handleDownload} />

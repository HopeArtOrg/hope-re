<script lang="ts">
  import {
    BinaryIcon,
    ChevronDownIcon,
    KeyIcon,
    LoaderCircleIcon,
    RotateCcwIcon,
    ScanLineIcon,
  } from "@lucide/svelte";
  import { toast } from "svelte-sonner";

  import {
    BaseImagePlaceholder,
  } from "$lib/components";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { useExtractWatermark } from "$lib/queries";
  import { useImage } from "$lib/stores/use-image.svelte";
  import { useWatermarkState } from "$lib/stores/use-watermark-state.svelte";

  import VerificationStatus from "./verification-status.svelte";

  const extractImage = useImage();
  const extractMutation = useExtractWatermark();
  const wmState = useWatermarkState();

  let useCustomExtractSeed = $state<boolean>(false);
  let extractSeedValue = $state<string>("");
  let expectedLength = $state<string>("17");
  let extractedText = $state<string>("");
  let verificationState = $state<"idle" | "verified" | "failed">("idle");

  function isValidWatermark(text: string): boolean {
    if (!text || text.length === 0) {
      return false;
    }
    let printableCount = 0;
    for (let i = 0; i < text.length; i++) {
      const code = text.charCodeAt(i);
      if (code >= 32 && code !== 127) {
        printableCount++;
      }
      else if (code === 9 || code === 10 || code === 13) {
        printableCount++;
      }
    }
    return (printableCount / text.length) >= 0.70;
  }

  $effect(() => {
    if (wmState.scannableImage) {
      extractImage.originalImage = wmState.scannableImage;
    }
  });

  $effect(() => {
    if (wmState.lastWatermarkLength) {
      expectedLength = String(wmState.lastWatermarkLength);
    }
  });

  let isProcessing = $state<boolean>(false);
  let progress = $state<number>(0);
  let progressStatus = $state<"idle" | "processing" | "success" | "error">("idle");
  let progressMessage = $state<string>("");

  async function handleExtract() {
    if (!extractImage.originalImage) {
      toast.error("Please upload an image to scan");
      return;
    }

    isProcessing = true;
    progressStatus = "processing";

    const steps = [
      { progress: 30, message: "Analyzing digital canvas structure...", duration: 400 },
      { progress: 65, message: "Scanning watermark coefficients...", duration: 500 },
    ];

    for (const step of steps) {
      progress = step.progress;
      progressMessage = step.message;
      await new Promise(resolve => setTimeout(resolve, step.duration));
    }

    try {
      progress = 85;
      progressMessage = "Decoding hidden signature bits...";

      const seed = useCustomExtractSeed ? Number(extractSeedValue) : undefined;
      const watermarkLen = Number(expectedLength) || 17;

      const result = await extractMutation.mutateAsync({
        imageBase64: extractImage.originalImage,
        watermarkLen,
        seed,
      });

      if (isValidWatermark(result)) {
        extractedText = result;
        verificationState = "verified";
        toast.success("Signature revealed successfully");
      }
      else {
        extractedText = "";
        verificationState = "failed";
        toast.error("No valid signature detected");
      }
      progress = 100;
      progressStatus = "success";
      setTimeout(() => {
        progressStatus = "idle";
        progress = 0;
        progressMessage = "";
      }, 3000);
    }
    catch (error: any) {
      progressStatus = "error";
      verificationState = "failed";
      toast.error(error instanceof Error ? error.message : String(error));
    }
    finally {
      isProcessing = false;
    }
  }

  function handleCancel() {
    isProcessing = false;
    progress = 0;
    progressStatus = "idle";
    progressMessage = "";

    extractImage.clear();
    extractedText = "";
    verificationState = "idle";
    useCustomExtractSeed = false;
    extractSeedValue = "";
    expectedLength = "17";

    toast.success("Canvas reset complete");
  }
</script>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-10">
  <div class="flex flex-col gap-8">
    <div class="relative group">
      <BaseImagePlaceholder
        imageSrc={extractImage.originalImage}
        label="Watermarked Canvas to Scan"
        containerClass="sticky-note p-8 min-h-[350px]"
        onUpload={extractImage.handleUpload}
      />
    </div>
  </div>

  <div class="flex flex-col gap-6">
    <div class="bg-teal-50/70 border-2 border-teal-200/50 dark:bg-teal-950/10 dark:border-teal-900/30 rounded-3xl p-8 flex flex-col gap-6 shadow-xl relative overflow-hidden doodle-line">
      <div class="absolute -right-6 -bottom-6 opacity-[0.04] -rotate-12 text-8xl font-black tracking-widest pointer-events-none select-none text-teal-900 dark:text-teal-100">
        印章
      </div>

      <div class="flex items-center gap-3 bg-linear-to-r from-teal-500/10 to-emerald-500/10 text-teal-800 dark:text-teal-300 p-4 rounded-2xl border border-teal-200/30 dark:border-teal-900/20">
        <ScanLineIcon class="size-6 shrink-0" />
        <div>
          <h2 class="text-xl font-bold leading-none">Verification Settings</h2>
          <p class="text-xs opacity-80 mt-1">Scan a canvas to verify embedded ownership signature</p>
        </div>
      </div>

      <div class="flex flex-col gap-3">
        <button
          type="button"
          disabled={isProcessing}
          onclick={() => useCustomExtractSeed = !useCustomExtractSeed}
          class="flex items-center justify-between w-full text-left font-bold text-sm text-teal-800/80 dark:text-teal-400/80 hover:opacity-85 transition-opacity cursor-pointer disabled:pointer-events-none"
        >
          <span class="flex items-center gap-1.5">
            <KeyIcon class="size-4" />
            Use Decryption Key
          </span>
          <ChevronDownIcon class="size-5 transition-transform duration-300 {useCustomExtractSeed ? "rotate-180 text-teal-500" : "text-muted-foreground/60"}" />
        </button>
        {#if useCustomExtractSeed}
          <div class="animate-in slide-in-from-top-2 fade-in duration-300">
            <Input
              id="extract-seed-input"
              type="text"
              placeholder="Enter the encryption seed"
              bind:value={extractSeedValue}
              disabled={isProcessing}
              class="doodle-line border-2 border-teal-200/60 focus-visible:border-teal-500 focus-visible:ring-teal-500/20 dark:border-teal-800/40 h-10 text-base px-4 bg-background/50 font-bold"
            />
          </div>
        {/if}
      </div>

      <div class="flex flex-col gap-2">
        <label for="length-input" class="text-sm font-bold flex items-center gap-1.5 text-teal-800/80 dark:text-teal-400/80">
          <BinaryIcon class="size-4" />
          Expected Signature Length (characters)
        </label>
        <Input
          id="length-input"
          type="text"
          bind:value={expectedLength}
          disabled={isProcessing}
          class="doodle-line border-2 border-teal-200/60 focus-visible:border-teal-500 focus-visible:ring-teal-500/20 dark:border-teal-800/40 h-10 text-base px-4 bg-background/50 font-bold"
        />
      </div>

      {#if progressStatus === "processing"}
        <div class="flex flex-col gap-2 p-4 bg-teal-500/10 border-2 border-teal-500/20 rounded-xl doodle-line animate-in fade-in duration-300">
          <div class="flex items-center gap-3">
            <LoaderCircleIcon class="size-5 animate-spin text-teal-600 dark:text-teal-400" />
            <span class="text-sm font-bold text-teal-800 dark:text-teal-300">{progressMessage}</span>
          </div>
          <div class="w-full bg-teal-500/10 h-3 rounded-full overflow-hidden border border-teal-200/20">
            <div class="bg-teal-500 h-full transition-all duration-300" style="width: {progress}%"></div>
          </div>
        </div>
      {/if}

      <VerificationStatus {verificationState} {extractedText} />
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-6 mt-4">
      <Button
        size="lg"
        class="gap-2 h-16 bg-teal-600 hover:bg-teal-500 active:bg-teal-700 text-white transition-all doodle-line text-lg shadow-lg hover:-translate-y-1 active:translate-y-0.5 disabled:opacity-50"
        onclick={handleExtract}
        disabled={!extractImage.hasImage || isProcessing}
      >
        {#if isProcessing}
          <LoaderCircleIcon class="size-6 animate-spin" />
          <span class="font-bold">Verifying...</span>
        {:else}
          <ScanLineIcon class="size-6" />
          <span class="font-bold">Verify Sheet</span>
        {/if}
      </Button>

      <Button
        variant="outline"
        size="lg"
        class="gap-2 h-16 border-2 border-foreground/20 hover:bg-destructive/10 hover:border-destructive/30 hover:text-destructive transition-all doodle-line text-lg shadow-md hover:-translate-y-1 active:translate-y-0.5 disabled:opacity-50"
        onclick={handleCancel}
        disabled={!extractImage.hasImage}
      >
        <RotateCcwIcon class="size-6" />
        <span class="font-bold">Reset Sheet</span>
      </Button>
    </div>
  </div>
</div>

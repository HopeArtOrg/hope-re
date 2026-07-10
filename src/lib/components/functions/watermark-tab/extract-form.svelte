<script lang="ts">
  import {
    BinaryIcon,
    ChevronDownIcon,
    KeyIcon,
    LoaderCircleIcon,
    RotateCcwIcon,
    ScanLineIcon,
    ShieldCheckIcon,
  } from "@lucide/svelte";
  import { toast } from "svelte-sonner";

  import {
    BaseImagePlaceholder,
  } from "$lib/components";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { useExtractWatermark } from "$lib/queries";
  import { useImage } from "$lib/stores/use-image.svelte";

  const extractImage = useImage();
  const extractMutation = useExtractWatermark();

  let useCustomExtractSeed = $state<boolean>(false);
  let extractSeedValue = $state<string>("");
  let expectedLength = $state<string>("20");
  let extractedText = $state<string>("");

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
      const watermarkLen = Number(expectedLength) || 20;

      const result = await extractMutation.mutateAsync({
        imageBase64: extractImage.originalImage,
        watermarkLen,
        seed,
      });

      extractedText = result;
      progress = 100;
      progressStatus = "success";
      toast.success("Signature revealed successfully");
      setTimeout(() => {
        progressStatus = "idle";
        progress = 0;
        progressMessage = "";
      }, 3000);
    }
    catch (error: any) {
      progressStatus = "error";
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
    useCustomExtractSeed = false;
    extractSeedValue = "";
    expectedLength = "20";

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

      <div class="border-2 border-foreground/10 rounded-2xl p-6 relative overflow-hidden bg-background/60 shadow-inner flex items-center justify-between min-h-[140px] doodle-line gap-4">
        {#if extractedText}
          <div class="flex flex-col gap-1.5 animate-in slide-in-from-left duration-500 flex-1">
            <span class="text-xs text-teal-700/70 dark:text-teal-400/70 font-black uppercase tracking-wider flex items-center gap-1.5">
              <ShieldCheckIcon class="size-4 text-teal-600 dark:text-teal-400" />
              Signature Decoded
            </span>
            <p class="text-2xl font-bold text-foreground font-sans border-b-2 border-dashed border-foreground/20 py-1 inline-block">
              {extractedText}
            </p>
          </div>
          <div class="border-4 border-double border-red-500/80 rounded-full size-24 flex flex-col items-center justify-center text-red-500/80 font-bold -rotate-12 shadow-sm select-none pointer-events-none bg-red-500/5 shrink-0 animate-in zoom-in duration-700 relative after:content-[''] after:absolute after:inset-1 after:border after:border-red-500/20 after:rounded-full">
            <span class="text-[9px] tracking-widest leading-none font-black uppercase">Verified</span>
            <span class="text-2xl font-black leading-tight mt-0.5">証</span>
          </div>
        {:else}
          <div class="flex flex-col gap-1 text-muted-foreground/80 flex-1">
            <span class="text-xs font-black uppercase tracking-wider text-muted-foreground/60">Verification Status</span>
            <p class="text-sm font-bold text-muted-foreground/70">No signature scanned yet</p>
            <p class="text-xs text-muted-foreground/50 leading-tight">Upload a canvas, configure settings, and verify.</p>
          </div>
          <div class="border-2 border-dashed border-muted-foreground/30 rounded-full size-24 flex flex-col items-center justify-center text-muted-foreground/30 font-bold -rotate-6 shrink-0 select-none pointer-events-none bg-muted/5">
            <span class="text-[9px] tracking-widest leading-none font-black uppercase">Void</span>
            <span class="text-2xl font-black leading-tight mt-0.5">未</span>
          </div>
        {/if}
      </div>
    </div>

    <div class="grid grid-cols-2 gap-6 mt-4">
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

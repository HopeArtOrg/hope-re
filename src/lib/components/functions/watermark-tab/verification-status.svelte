<script lang="ts">
  import { ShieldCheckIcon } from "@lucide/svelte";

  type VerificationStatusProps = {
    verificationState: "idle" | "verified" | "failed";
    extractedText: string;
  };

  const { verificationState, extractedText }: VerificationStatusProps = $props();
</script>

<div class="border-2 border-foreground/10 rounded-2xl p-6 relative overflow-hidden bg-background/60 shadow-inner flex flex-col sm:flex-row items-center justify-between min-h-[140px] doodle-line gap-6 sm:gap-4 text-center sm:text-left">
  {#if verificationState === "verified"}
    <div class="flex flex-col items-center sm:items-start gap-1.5 animate-in slide-in-from-left duration-500 flex-1">
      <span class="text-xs text-teal-700/70 dark:text-teal-400/70 font-black uppercase tracking-wider flex items-center justify-center sm:justify-start gap-1.5">
        <ShieldCheckIcon class="size-4 text-teal-600 dark:text-teal-400" />
        Signature Decoded
      </span>
      <p class="text-2xl font-bold text-foreground font-sans border-b-2 border-dashed border-foreground/20 py-1 inline-block break-all">
        {extractedText}
      </p>
    </div>
    <div class="border-4 border-double border-red-500/80 rounded-full size-24 flex flex-col items-center justify-center text-red-500/80 font-bold -rotate-12 shadow-sm select-none pointer-events-none bg-red-500/5 shrink-0 animate-in zoom-in duration-700 relative after:content-[''] after:absolute after:inset-1 after:border after:border-red-500/20 after:rounded-full">
      <span class="text-[9px] tracking-widest leading-none font-black uppercase">Verified</span>
      <span class="text-2xl font-black leading-tight mt-0.5">証</span>
    </div>
  {:else if verificationState === "failed"}
    <div class="flex flex-col items-center sm:items-start gap-1 text-muted-foreground/80 flex-1 animate-in fade-in duration-300">
      <span class="text-xs font-black uppercase tracking-wider text-muted-foreground/60">Verification Status</span>
      <p class="text-sm font-bold text-muted-foreground/70">No valid signature detected</p>
      <p class="text-xs text-muted-foreground/50 leading-tight">This canvas does not contain a verified digital signature.</p>
    </div>
    <div class="border-2 border-dashed border-muted-foreground/30 rounded-full size-24 flex flex-col items-center justify-center text-muted-foreground/30 font-bold -rotate-6 shrink-0 select-none pointer-events-none bg-muted/5 animate-in fade-in duration-300">
      <span class="text-[9px] tracking-widest leading-none font-black uppercase">Void</span>
      <span class="text-2xl font-black leading-tight mt-0.5">未</span>
    </div>
  {:else}
    <div class="flex flex-col items-center sm:items-start gap-1 text-muted-foreground/80 flex-1">
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

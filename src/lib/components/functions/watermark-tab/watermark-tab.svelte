<script lang="ts">
  import { ScanLineIcon, StampIcon } from "@lucide/svelte";

  import { useWatermarkState } from "$lib/stores/use-watermark-state.svelte";

  import EmbedForm from "./embed-form.svelte";
  import ExtractForm from "./extract-form.svelte";

  const wmState = useWatermarkState();
</script>

<div class="flex flex-col gap-8 h-full relative z-10 animate-in fade-in duration-500">
  <div class="flex justify-center">
    <div class="flex border-b-2 border-foreground/10 w-full max-w-lg">
      <button
        onclick={() => wmState.activeSubTab = "embed"}
        class="flex-1 py-3 px-6 text-center font-bold text-lg border-2 border-b-0 border-foreground/10 rounded-t-2xl transition-all relative {wmState.activeSubTab === "embed" ? "bg-card text-foreground mb-[-2px] z-10 shadow-[0px_-4px_10px_rgba(0,0,0,0.05)]" : "bg-muted/20 text-muted-foreground border-transparent hover:bg-muted/30"}"
      >
        <StampIcon class="size-4 inline-block mr-2 align-text-bottom" />
        Sign Canvas
      </button>
      <button
        onclick={() => wmState.activeSubTab = "extract"}
        class="flex-1 py-3 px-6 text-center font-bold text-lg border-2 border-b-0 border-foreground/10 rounded-t-2xl transition-all relative {wmState.activeSubTab === "extract" ? "bg-card text-foreground mb-[-2px] z-10 shadow-[0px_-4px_10px_rgba(0,0,0,0.05)]" : "bg-muted/20 text-muted-foreground border-transparent hover:bg-muted/30"}"
      >
        <ScanLineIcon class="size-4 inline-block mr-2 align-text-bottom" />
        Verify Canvas
      </button>
    </div>
  </div>

  {#if wmState.activeSubTab === "embed"}
    <EmbedForm />
  {:else if wmState.activeSubTab === "extract"}
    <ExtractForm />
  {/if}
</div>

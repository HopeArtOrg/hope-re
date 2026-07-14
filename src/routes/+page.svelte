<script lang="ts">
  import { ShieldIcon, StampIcon } from "@lucide/svelte";

  import { Background, GlazeTab, WatermarkTab } from "$lib/components";
  import { t } from "$lib/stores/use-i18n.svelte";

  let activeTab = $state<string>("glaze");
</script>

<svelte:head>
  <title>Hope:RE</title>
  <meta name="description" content={t("app.metaDescription")} />
</svelte:head>

<div class="w-full bg-background transition-colors duration-500 overflow-x-hidden min-h-screen relative">
  <Background />

  <div class="container mx-auto p-6 md:p-10 h-full max-w-7xl relative z-10 flex flex-col lg:flex-row gap-10">
    <div class="flex flex-row lg:flex-col gap-2 shrink-0 lg:w-48 lg:border-r-2 lg:border-dashed lg:border-foreground/20 lg:pr-6 relative border-b-2 lg:border-b-0 pb-4 lg:pb-0">
      <button
        onclick={() => activeTab = "glaze"}
        class="flex-1 lg:flex-initial text-center lg:text-left px-4 py-3 rounded-t-xl lg:rounded-t-none lg:rounded-l-2xl border-2 lg:border-r-0 border-foreground/10 font-bold transition-all flex items-center justify-center lg:justify-start gap-2 relative lg:mr-[-26px] z-10 {activeTab === "glaze" ? "bg-background text-foreground lg:border-r-background shadow-[0px_-4px_10px_rgba(0,0,0,0.05)] lg:shadow-[-4px_2px_10px_rgba(0,0,0,0.05)]" : "bg-muted/10 text-muted-foreground border-transparent hover:bg-muted/20"}"
      >
        <ShieldIcon class="size-4 shrink-0" />
        <span>{t("tabs.cloak")}</span>
      </button>

      <button
        onclick={() => activeTab = "watermark"}
        class="flex-1 lg:flex-initial text-center lg:text-left px-4 py-3 rounded-t-xl lg:rounded-t-none lg:rounded-l-2xl border-2 lg:border-r-0 border-foreground/10 font-bold transition-all flex items-center justify-center lg:justify-start gap-2 relative lg:mr-[-26px] z-10 {activeTab === "watermark" ? "bg-background text-foreground lg:border-r-background shadow-[0px_-4px_10px_rgba(0,0,0,0.05)] lg:shadow-[-4px_2px_10px_rgba(0,0,0,0.05)]" : "bg-muted/10 text-muted-foreground border-transparent hover:bg-muted/20"}"
      >
        <StampIcon class="size-4 shrink-0" />
        <span>{t("tabs.watermark")}</span>
      </button>
    </div>

    <div class="flex-1 min-w-0">
      <div class:hidden={activeTab !== "glaze"}>
        <GlazeTab />
      </div>
      <div class:hidden={activeTab !== "watermark"}>
        <WatermarkTab />
      </div>
    </div>
  </div>
</div>

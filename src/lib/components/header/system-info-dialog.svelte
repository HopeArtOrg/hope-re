<script lang="ts">
  import {
    BoxIcon,
    CircleAlertIcon,
    CpuIcon,
    GlobeIcon,
    HardDriveIcon,
    InfoIcon,
    LoaderCircleIcon,
    MemoryStickIcon,
    MonitorIcon,
    RefreshCwIcon,
    ZapIcon,
  } from "@lucide/svelte";

  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { useSystemInfo } from "$lib/queries/system-info";
  import { cn } from "$lib/utils";

  const {
    data: info,
    isLoading,
    isError,
    error,
    isFetching,
    refetch,
  } = $derived(useSystemInfo());

  let dialogOpen = $state<boolean>(false);

  function handleDialogOpen(open: boolean) {
    dialogOpen = open;
  }
</script>

<Dialog.Root open={dialogOpen} onOpenChange={handleDialogOpen}>
  <Dialog.Trigger>
    <Button variant="outline"
            size="sm"
            class="flex items-center gap-2 px-3 py-2 doodle-blob border-2 border-foreground/10 bg-white/40 dark:bg-black/40 hover:bg-white/60 dark:hover:bg-black/60 hover:cursor-pointer transition-all shadow-sm"
            aria-label="System Info">
      <InfoIcon class="size-4" />
    </Button>
  </Dialog.Trigger>

  <Dialog.Content class="max-w-md blue-note border-2 border-foreground/10 p-10">
    <Dialog.Header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div class="p-3 doodle-blob bg-primary/10 border-2 border-foreground/5">
            <MonitorIcon class="size-5 text-primary" />
          </div>
          <Dialog.DialogTitle class="text-xl font-bold tracking-tight">
            Studio Vitals
          </Dialog.DialogTitle>
        </div>
      </div>
    </Dialog.Header>

    <div class="mt-6">
      {#if isLoading && !info}
        <div class="flex flex-col items-center justify-center py-12">
          <LoaderCircleIcon class="size-10 animate-spin text-primary mb-3" />
          <p class="text-sm font-bold text-muted-foreground">Checking vitals...</p>
        </div>
      {:else if isError}
        <div class="flex items-start gap-4 p-5 doodle-line bg-rose-500/10 border-2 border-rose-500/20">
          <CircleAlertIcon class="size-6 text-rose-600 shrink-0 mt-0.5" />
          <div>
            <p class="text-base font-bold text-rose-700">Studio is offline!</p>
            <p class="text-xs font-bold text-muted-foreground mt-1">{error?.message || "Unknown Error"}</p>
          </div>
        </div>
      {:else if info}
        <div class={cn(isFetching && "opacity-50 transition-opacity")}>
          <div class="doodle-line border-2 border-foreground/10 bg-white/40 dark:bg-black/20 p-8 space-y-6 shadow-inner relative">
            <div class="absolute -top-3 -right-2 rotate-[12deg] text-[10px] bg-sky-500 text-white px-2 py-0.5 shadow-sm doodle-line">SPECS</div>

            <div class="pb-6 border-b-2 border-foreground/5">
              <div class="flex items-center gap-3 mb-2">
                <GlobeIcon class="size-5 text-sky-600" />
                <p class="text-xs font-bold text-muted-foreground uppercase">Environment</p>
              </div>
              <p class="text-base font-bold pl-8">{info.platform.os}</p>
              <p class="text-xs font-bold text-muted-foreground/60 mt-1 pl-8">{info.platform.hostname}</p>
            </div>

            <div class="space-y-4">
              <div class="flex items-start gap-3">
                <CpuIcon class="size-5 text-indigo-600 shrink-0 mt-0.5" />
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-bold">
                    <span class="text-muted-foreground">Brain:</span>
                    <span class="ml-2">{info.cpu}</span>
                  </p>
                </div>
              </div>

              <div class="flex items-start gap-3">
                <MemoryStickIcon class="size-5 text-violet-600 shrink-0 mt-0.5" />
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-bold">
                    <span class="text-muted-foreground">Short-Term:</span>
                    <span class="ml-2">{info.memory}</span>
                  </p>
                </div>
              </div>

              <div class="flex items-start gap-3">
                <ZapIcon class="size-5 text-emerald-600 shrink-0 mt-0.5" />
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-bold">
                    <span class="text-muted-foreground">Brush Power:</span>
                    <span class="ml-2">{info.gpu}</span>
                  </p>
                </div>
              </div>

              <div class="flex items-start gap-3">
                <HardDriveIcon class="size-5 text-amber-600 shrink-0 mt-0.5" />
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-bold">
                    <span class="text-muted-foreground">Storage:</span>
                    <span class="ml-2">{info.storage}</span>
                  </p>
                </div>
              </div>
            </div>

            <div class="pt-6 border-t-2 border-foreground/5">
              <div class="flex items-start gap-3">
                <BoxIcon class="size-5 text-rose-600 shrink-0 mt-0.5" />
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-bold">
                    <span class="text-muted-foreground">Studio Version:</span>
                    <span class="ml-2">v{info.app_version}</span>
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>

    {#if info && !isLoading}
      <div class="flex items-center justify-between mt-8 pt-6 border-t-2 border-foreground/10">
        <p class="text-xs font-bold text-muted-foreground/60 italic">
          Last check: {new Date().toLocaleTimeString()}
        </p>
        <Button
          variant="outline"
          size="sm"
          onclick={refetch}
          disabled={isFetching}
          class="gap-3 doodle-blob border-2 border-foreground/10 bg-white/40 hover:bg-white/60 h-10 px-4"
        >
          <RefreshCwIcon class={cn("size-4", isFetching && "animate-spin")} />
          <span class="font-bold">Check Again</span>
        </Button>
      </div>
    {/if}
  </Dialog.Content>
</Dialog.Root>

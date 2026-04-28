<script lang="ts">
  import { CircleArrowUpIcon, LoaderCircleIcon } from "@lucide/svelte";

  import { Button } from "$lib/components/ui/button";
  import { useUpdater } from "$lib/stores/use-updater.svelte";
  import { cn } from "$lib/utils";

  const updater = useUpdater();
</script>

<Button
  variant="outline"
  size="sm"
  onclick={() => updater.openDialog()}
  disabled={updater.isDownloading || updater.isInstalling}
  class={cn(
    "flex items-center gap-2 px-3 py-2 doodle-blob border-2 border-foreground/10 bg-white/40 dark:bg-black/40 hover:bg-white/60 dark:hover:bg-black/60 hover:cursor-pointer transition-all shadow-sm",
    updater.isUpdateAvailable && "border-primary/60 text-primary bg-primary/10 shadow-md",
  )}
  aria-label="Check for updates"
>
  {#if updater.isChecking}
    <LoaderCircleIcon class="size-4 animate-spin" />
  {:else}
    <CircleArrowUpIcon class={cn("size-4", updater.isUpdateAvailable && "text-primary")} />
  {/if}
</Button>

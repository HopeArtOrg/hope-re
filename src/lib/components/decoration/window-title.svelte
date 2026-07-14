<script lang="ts">
  import {
    MaximizeIcon,
    MinimizeIcon,
    MinusIcon,
    XIcon,
  } from "@lucide/svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  const appWindow = getCurrentWindow();

  let isMaximized = $state<boolean>(false);

  function handleMaximize() {
    appWindow.toggleMaximize();
  }

  $effect(() => {
    let unlisten: (() => void) | undefined;
    let cancelled = false;

    appWindow.isMaximized().then((value) => {
      isMaximized = value;
    });

    appWindow
      .onResized(async () => {
        isMaximized = await appWindow.isMaximized();
      })
      .then((fn) => {
        if (cancelled) {
          fn();
        }
        else {
          unlisten = fn;
        }
      });

    return () => {
      cancelled = true;
      unlisten?.();
    };
  });
</script>

<div
  data-tauri-drag-region
  class="fixed inset-0 inset-x-0 left-0 right-0 top-0 z-100 flex h-[30px] select-none justify-end bg-background border-b border-border/20"
>
  <button
    onclick={() => appWindow.minimize()}
    aria-label="Minimize"
    class="inline-flex h-[30px] w-[30px] items-center justify-center px-2 py-1 transition-colors duration-100 hover:bg-zinc-600/50"
  >
    <MinusIcon class="size-4 text-foreground" />
  </button>
  <button
    onclick={handleMaximize}
    aria-label={isMaximized ? "Restore" : "Maximize"}
    class="inline-flex h-[30px] w-[30px] items-center justify-center px-2 py-1 transition-colors duration-100 hover:bg-zinc-600/50"
  >
    {#if isMaximized}
      <MinimizeIcon class="size-4 text-foreground" />
    {:else}
      <MaximizeIcon class="size-4 text-foreground" />
    {/if}
  </button>
  <button
    onclick={() => appWindow.close()}
    aria-label="Close"
    class="inline-flex h-[30px] w-[30px] items-center justify-center px-2 py-1 transition-colors duration-100 hover:bg-red-500"
  >
    <XIcon class="size-4 text-foreground" />
  </button>
</div>

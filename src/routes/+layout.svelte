<script lang="ts">
  import type { LayoutData } from "./$types";
  import type { Snippet } from "svelte";

  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";
  import { platform } from "@tauri-apps/plugin-os";

  import "../app.css";
  import { onMount } from "svelte";

  import favicon from "$lib/assets/favicon.svg";
  import { Header, MinimizedDock, ResourceDownloadGuard, UpdateDialog, WindowTitle } from "$lib/components";
  import { Toaster } from "$lib/components/ui/sonner";
  import { useTheme } from "$lib/stores/use-theme.svelte";
  import { cn } from "$lib/utils";

  type LayoutDataProps = {
    children: Snippet;
    data: LayoutData;
  };

  const { children, data }: LayoutDataProps = $props();

  let currentPlatform = $state<string>("");
  const isWindows = $derived(currentPlatform === "windows");

  if (typeof window !== "undefined") {
    currentPlatform = platform();
  }

  const theme = useTheme();

  onMount(async () => {
    await theme.initTheme();
    if (isWindows) {
      document.documentElement.dataset.platform = "windows";
    }
  });
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

<QueryClientProvider client={data.queryClient}>
  <Toaster position="top-center" />
  <UpdateDialog />

  {#if isWindows}
    <WindowTitle />
  {/if}

  <div class={cn("h-screen flex flex-col overflow-hidden", isWindows && "pt-[30px]")}>
    <Header />
    <main class="flex-1 overflow-auto">
      <ResourceDownloadGuard>
        {@render children()}
      </ResourceDownloadGuard>
    </main>
  </div>

  <div class="fixed bottom-4 right-4 z-90 flex flex-col items-end gap-2">
    <MinimizedDock />
  </div>

  {#if import.meta.env.DEV}
    <SvelteQueryDevtools initialIsOpen={false} />
  {/if}
</QueryClientProvider>

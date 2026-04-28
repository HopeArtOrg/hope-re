<script lang="ts">
  import { platform } from "@tauri-apps/plugin-os";

  import { cn } from "$lib/utils";

  import SystemInfoDialog from "./system-info-dialog.svelte";
  import ThemeToggle from "./theme-toggle.svelte";
  import UpdateButton from "./update-button.svelte";

  let currentPlatform = $state<string>("");
  const isMobile = $derived(currentPlatform === "android" || currentPlatform === "ios");

  if (typeof window !== "undefined") {
    currentPlatform = platform();
  }
</script>

<header class="border-b-2 border-foreground/10 backdrop-blur-md bg-white/30 dark:bg-black/30 shrink-0 z-50 doodle-line">
  <div class={cn("h-[env(safe-area-inset-top)]", isMobile ? "min-h-6" : "")}></div>
  <div class="h-14 px-8 flex items-center justify-end gap-4">
    <div class="mr-auto">
      <h1 class="text-xl font-bold text-foreground/80 tracking-tighter">Hope:RE <span class="text-xs opacity-50 font-medium">Studio v2</span></h1>
    </div>
    <UpdateButton />
    <SystemInfoDialog />
    <ThemeToggle />
  </div>
</header>

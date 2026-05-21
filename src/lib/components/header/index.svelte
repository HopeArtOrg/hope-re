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
    <div class="mr-auto flex items-center gap-4 group">
      <div class="size-10 p-1.5 doodle-blob bg-primary/10 border-2 border-foreground/5 flex items-center justify-center transition-all duration-500 group-hover:rotate-12 group-hover:scale-110 shadow-sm">
        <img src="/logo.png"
             alt="Hope:RE Studio Logo"
             class="size-full object-contain" />
      </div>
      <h1 class="text-2xl font-bold text-foreground/80 tracking-tighter flex items-center gap-2">
        Hope:RE
        <span class="text-[9px] opacity-40 font-black uppercase tracking-widest bg-foreground/5 px-2 py-0.5 doodle-line self-center translate-y-0.5">Studio v2</span>
      </h1>
    </div>
    <UpdateButton />
    <SystemInfoDialog />
    <ThemeToggle />
  </div>
</header>

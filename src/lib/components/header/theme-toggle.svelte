<script lang="ts">
  import type { Theme } from "$lib/stores/use-theme.svelte";

  import { MoonIcon, SunIcon } from "@lucide/svelte";

  import { Button } from "$lib/components/ui/button";
  import { useTheme } from "$lib/stores/use-theme.svelte";

  const theme = useTheme();

  const currentTheme = $derived<Theme>(theme.getTheme());

  const isDark = $derived(
    currentTheme === "dark" || (currentTheme === "system" && typeof window !== "undefined" && window.matchMedia("(prefers-color-scheme: dark)").matches),
  );

  async function toogleTheme() {
    const newTheme: Theme = isDark ? "light" : "dark";
    await theme.setTheme(newTheme);
  }
</script>

<Button variant="outline"
        size="sm"
        onclick={toogleTheme}
        class="flex items-center gap-2 px-3 py-2 doodle-blob border-2 border-foreground/10 bg-white/40 dark:bg-black/40 hover:bg-white/60 dark:hover:bg-black/60 hover:cursor-pointer transition-all shadow-sm"
        aria-label="Toggle theme">
  {#if isDark}
    <SunIcon class="size-4 text-amber-600 transition-transform duration-300 rotate-0 hover:rotate-90" />
  {:else}
    <MoonIcon class="size-4 text-indigo-600 transition-transform duration-300 rotate-0 hover:-rotate-12" />
  {/if}
</Button>

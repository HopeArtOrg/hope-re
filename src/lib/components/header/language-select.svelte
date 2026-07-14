<script lang="ts">
  import type { Locale } from "$lib/i18n";

  import { LanguagesIcon } from "@lucide/svelte";

  import * as Select from "$lib/components/ui/select";
  import { locales } from "$lib/i18n";
  import { t, useI18n } from "$lib/stores/use-i18n.svelte";

  const i18n = useI18n();

  const value = $derived(i18n.getLocale());
  const currentLabel = $derived(locales.find(item => item.value === value)?.label ?? "English");

  function handleChange(newValue: string) {
    i18n.setLocale(newValue as Locale);
  }
</script>

<Select.Root type="single"
             {value}
             onValueChange={handleChange}>
  <Select.Trigger
    size="sm"
    class="w-auto gap-2 bg-white/40 dark:bg-black/40 hover:bg-white/60 dark:hover:bg-black/60 border-foreground/10 shadow-sm"
    aria-label={t("header.language")}
  >
    <LanguagesIcon class="size-4 shrink-0" />
    <span class="font-bold">{currentLabel}</span>
  </Select.Trigger>
  <Select.Content class="min-w-[8rem]">
    {#each locales as item (item.value)}
      <Select.Item value={item.value} label={item.label} />
    {/each}
  </Select.Content>
</Select.Root>

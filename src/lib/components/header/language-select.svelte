<script lang="ts">
  import type { Locale } from "$lib/i18n";

  import { LanguagesIcon } from "@lucide/svelte";
  import { Select as SelectPrimitive } from "bits-ui";

  import { Button } from "$lib/components/ui/button";
  import * as Select from "$lib/components/ui/select";
  import { locales } from "$lib/i18n";
  import { t, useI18n } from "$lib/stores/use-i18n.svelte";

  const i18n = useI18n();

  const value = $derived(i18n.getLocale());

  function handleChange(newValue: string) {
    i18n.setLocale(newValue as Locale);
  }
</script>

<Select.Root type="single"
             {value}
             onValueChange={handleChange}>
  <SelectPrimitive.Trigger>
    {#snippet child({ props })}
      <Button
        {...props}
        variant="outline"
        size="sm"
        class="flex items-center gap-2 px-3 py-2 doodle-blob border-2 border-foreground/10 bg-white/40 dark:bg-black/40 hover:bg-white/60 dark:hover:bg-black/60 hover:cursor-pointer transition-all shadow-sm"
        aria-label={t("header.language")}
      >
        <LanguagesIcon class="size-4" />
      </Button>
    {/snippet}
  </SelectPrimitive.Trigger>
  <Select.Content class="min-w-[8rem]">
    {#each locales as item (item.value)}
      <Select.Item value={item.value} label={item.label} />
    {/each}
  </Select.Content>
</Select.Root>

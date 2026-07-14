<script lang="ts">
  import type { NightshadeTargetSelectProps } from "../types";

  import { CrosshairIcon } from "@lucide/svelte";

  import * as Select from "$lib/components/ui/select";
  import { nightshadeTargets } from "$lib/constants";
  import { t } from "$lib/stores/use-i18n.svelte";

  let { value = $bindable("dog") }: NightshadeTargetSelectProps = $props();

  const currentTarget = $derived(
    nightshadeTargets.find(target => target.value === value) ?? nightshadeTargets[0],
  );

  const contentTrigger = $derived(t(`nightshadeTargets.${currentTarget.value}.label`));
</script>

<div class="space-y-4">
  <div class="flex items-center gap-3">
    <div class="p-2.5 doodle-blob bg-card border-2 border-foreground/10 bg-rose-500/10">
      <CrosshairIcon class="size-5 text-rose-600 dark:text-rose-400" />
    </div>
    <span class="text-xl font-bold text-foreground/80 tracking-tight">{t("nightshadeSelect.title")}</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full text-lg" aria-label={t("nightshadeSelect.aria")}>
      {contentTrigger}
    </Select.Trigger>
    <Select.Content>
      {#each nightshadeTargets as target (target.value)}
        <Select.Item value={target.value}>
          <div class="flex items-center gap-4 py-2">
            <div class="p-2 doodle-blob border border-foreground/5 bg-muted/20">
              <target.icon class="size-6 text-muted-foreground" />
            </div>
            <div class="flex flex-col gap-0.5">
              <span class="font-bold text-lg">{t(`nightshadeTargets.${target.value}.label`)}</span>
              <span class="text-sm text-muted-foreground/80">{t(`nightshadeTargets.${target.value}.description`)}</span>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>

  <p class="text-sm text-muted-foreground/70 font-bold px-1 leading-tight">
    {t("nightshadeSelect.hint")}
  </p>
</div>

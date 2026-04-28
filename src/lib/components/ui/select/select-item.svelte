<script lang="ts">
  import type { WithoutChild } from "$lib/utils.js";

  import CheckIcon from "@lucide/svelte/icons/check";
  import { Select as SelectPrimitive } from "bits-ui";

  import { cn } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    value,
    label,
    children: childrenProp,
    ...restProps
  }: WithoutChild<SelectPrimitive.ItemProps> = $props();
</script>

<SelectPrimitive.Item
  bind:ref
  {value}
  data-slot="select-item"
  class={cn(
    "relative flex w-full items-center gap-2 px-2 py-1.5 text-base font-bold outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 select-item-hover data-[highlighted]:bg-primary/10 data-[highlighted]:scale-[1.02]",
    className,
  )}
  {...restProps}
>
  {#snippet children({ selected, highlighted })}
    <span class="absolute end-2 flex size-3.5 items-center justify-center">
      {#if selected}
        <CheckIcon class="size-4" />
      {/if}
    </span>
    {#if childrenProp}
      {@render childrenProp({ selected, highlighted })}
    {:else}
      {label || value}
    {/if}
  {/snippet}
</SelectPrimitive.Item>

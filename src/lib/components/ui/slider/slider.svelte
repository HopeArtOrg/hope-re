<script lang="ts">
  import type { WithoutChildrenOrChild } from "$lib/utils.js";

  import { Slider as SliderPrimitive } from "bits-ui";

  import { cn } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    value = $bindable(),
    orientation = "horizontal",
    class: className,
    ...restProps
  }: WithoutChildrenOrChild<SliderPrimitive.RootProps> = $props();
</script>

<!--
Discriminated Unions + Destructing (required for bindable) do not
get along, so we shut typescript up by casting `value` to `never`.
-->
<SliderPrimitive.Root
  bind:ref
  bind:value={value as never}
  data-slot="slider"
  {orientation}
  class={cn(
    "relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50 data-[orientation=vertical]:h-full data-[orientation=vertical]:min-h-44 data-[orientation=vertical]:w-auto data-[orientation=vertical]:flex-col",
    className,
  )}
  {...restProps}
>
  {#snippet children({ thumbs })}
    <span
      data-orientation={orientation}
      data-slot="slider-track"
      class={cn(
        "bg-neutral-200 dark:bg-neutral-800 relative grow overflow-hidden rounded-full data-[orientation=horizontal]:h-[2px] data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-[2px]",
      )}
    >
      <SliderPrimitive.Range
        data-slot="slider-range"
        class={cn(
          "bg-primary/60 absolute data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full",
        )}
      />
    </span>
    {#each thumbs as thumb (thumb)}
      <SliderPrimitive.Thumb
        data-slot="slider-thumb"
        index={thumb}
        class="block size-3 shrink-0 rounded-full bg-primary ring-offset-background transition-colors focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 cursor-pointer"
      />
    {/each}
  {/snippet}
</SliderPrimitive.Root>

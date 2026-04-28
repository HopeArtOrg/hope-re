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
        "bg-foreground/10 relative grow overflow-hidden doodle-line data-[orientation=horizontal]:h-2.5 data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-2.5 shadow-inner",
      )}
    >
      <SliderPrimitive.Range
        data-slot="slider-range"
        class={cn(
          "bg-primary/60 absolute data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full doodle-line transition-none",
        )}
      />
    </span>
    {#each thumbs as thumb (thumb)}
      <SliderPrimitive.Thumb
        data-slot="slider-thumb"
        index={thumb}
        class="border-2 border-foreground/20 ring-ring/20 block size-6 shrink-0 doodle-blob bg-card shadow-md transition-none hover:scale-110 hover:ring-8 focus-visible:ring-8 focus-visible:outline-hidden disabled:pointer-events-none disabled:opacity-50"
      />
    {/each}
  {/snippet}
</SliderPrimitive.Root>

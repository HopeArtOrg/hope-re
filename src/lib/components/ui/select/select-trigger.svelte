<script lang="ts">
  import type { WithoutChild } from "$lib/utils.js";

  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import { Select as SelectPrimitive } from "bits-ui";

  import { cn } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    children,
    size = "default",
    ...restProps
  }: WithoutChild<SelectPrimitive.TriggerProps> & {
    size?: "sm" | "default";
  } = $props();
</script>

<SelectPrimitive.Trigger
  bind:ref
  data-slot="select-trigger"
  data-size={size}
  class={cn(
    "border-2 border-foreground/20 doodle-line data-[placeholder]:text-muted-foreground bg-white/40 dark:bg-black/20 backdrop-blur-sm px-4 py-2 text-sm font-medium transition-all duration-300 outline-hidden select-none disabled:cursor-not-allowed disabled:opacity-50 data-[size=default]:h-12 data-[size=sm]:h-10 hover:bg-white/60 dark:hover:bg-black/40 flex items-center justify-between gap-2",
    className,
  )}
  {...restProps}
>
  {@render children?.()}
  <ChevronDownIcon class="size-4 opacity-50" />
</SelectPrimitive.Trigger>

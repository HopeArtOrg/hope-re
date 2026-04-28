<script lang="ts">
  import type { WithoutChildrenOrChild } from "$lib/utils.js";

  import { Progress as ProgressPrimitive } from "bits-ui";

  import { cn } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    max = 100,
    value,
    ...restProps
  }: WithoutChildrenOrChild<ProgressPrimitive.RootProps> = $props();
</script>

<ProgressPrimitive.Root
  bind:ref
  data-slot="progress"
  class={cn("bg-foreground/10 relative h-3 w-full overflow-hidden doodle-line shadow-inner", className)}
  {value}
  {max}
  {...restProps}
>
  <div
    data-slot="progress-indicator"
    class="bg-primary/60 h-full w-full flex-1 transition-all doodle-line"
    style="transform: translateX(-{100 - (100 * (value ?? 0)) / (max ?? 1)}%)"
  ></div>
</ProgressPrimitive.Root>

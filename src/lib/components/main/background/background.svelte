<script lang="ts">
  import {
    NotebookPenIcon,
    PaintbrushIcon,
    PencilIcon,
  } from "@lucide/svelte";
</script>

<div class="fixed bottom-10 left-10 opacity-20 decorative-doodle hidden lg:block rotate-12">
  <div class="w-2 h-40 bg-orange-200 rounded-full shadow-md relative">
    <div class="absolute top-0 w-full h-8 bg-black rounded-t-full"></div>
    <div class="absolute bottom-0 w-full h-10 bg-pink-300 rounded-b-full"></div>
  </div>
</div>

<div class="fixed top-24 left-12 opacity-10 decorative-doodle hidden xl:block -rotate-6">
  <PencilIcon class="size-32 text-foreground" strokeWidth={1} />
</div>

<div class="fixed bottom-24 right-12 opacity-10 decorative-doodle hidden xl:block rotate-12">
  <PaintbrushIcon class="size-40 text-foreground" strokeWidth={1} />
</div>

<div class="fixed top-40 right-16 opacity-10 decorative-doodle hidden xl:block -rotate-12">
  <NotebookPenIcon class="size-24 text-foreground" strokeWidth={1} />
</div>

<div class="fixed top-20 right-20 opacity-20 decorative-doodle hidden lg:block -rotate-12">
  <div class="w-16 h-8 bg-pink-200 rounded-sm shadow-sm flex items-center justify-center text-xs text-pink-600 font-bold uppercase border-b-2 border-pink-300/50">ERASE</div>
</div>

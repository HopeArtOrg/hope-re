---
name: svelte-component
description: Create a new Svelte 5 component following Hope:RE conventions with runes, Tailwind CSS, and proper barrel exports
---

## Svelte 5 Component Pattern

When creating a new component in Hope:RE, follow these conventions:

### File Structure
```
src/lib/components/<feature>/
  index.svelte
  index.ts
  types.ts
  sub-component.svelte
```

### Component Template
```svelte
<script lang="ts">
  import type { ComponentProps } from "./types";

  let {
    prop1 = "default",
    prop2 = $bindable(false),
    ...restProps
  }: ComponentProps = $props();

  let localState = $state<string>("");
  const derivedValue = $derived(prop1.toUpperCase());

  $effect(() => {
    return () => {};
  });

  function handleAction() {
  }
</script>

<div class="flex items-center gap-2">
  {derivedValue}
</div>
```

### Barrel Export (`index.ts`)
```typescript
export { default as ComponentName } from "./index.svelte";
export type { ComponentProps } from "./types";
```

### Types File (`types.ts`)
```typescript
export type ComponentProps = {
  prop1?: string;
  prop2?: boolean;
  onAction?: () => void;
};
```

### Rules
- Always use Svelte 5 runes (`$props`, `$state`, `$derived`, `$effect`)
- Never use legacy `export let`, `$:`, `$$props`, `$$restProps`
- No `<style>` blocks -- use Tailwind CSS utility classes
- No comments in code -- self-documenting through clear naming
- No emojis anywhere in the codebase
- File names must be kebab-case
- Export with PascalCase name through barrel `index.ts`
- Use `import type` for type-only imports
- Props use `$bindable()` for two-way binding
- Event handlers use `handle*` prefix

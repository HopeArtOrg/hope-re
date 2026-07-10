---
description: Create a new Svelte 5 component with proper conventions
agent: build
---

Create a new Svelte 5 component named $ARGUMENTS following Hope:RE conventions.

Use the `svelte-component` skill for the full pattern. Key requirements:
- Svelte 5 runes only ($props, $state, $derived, $effect)
- Tailwind CSS utility classes only (no <style> blocks)
- kebab-case file names
- PascalCase barrel export in index.ts
- Types in a co-located types.ts file using `type` keyword

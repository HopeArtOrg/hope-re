---
description: Type-check both frontend (svelte-check) and backend (cargo check)
agent: build
---

Run both type-checking commands in parallel:
1. `pnpm check` -- SvelteKit sync + svelte-check for TypeScript/Svelte frontend
2. `cargo check` in the `src-tauri/` directory -- Rust backend type checking

Fix any type errors found. Show a summary of results.

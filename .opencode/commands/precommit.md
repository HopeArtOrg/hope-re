---
description: Pre-commit validation - lint + type-check before committing
agent: build
---

Run the full pre-commit validation pipeline:

1. `pnpm lint` -- ESLint check (this is what the Husky pre-commit hook runs)
2. `pnpm check` -- SvelteKit + svelte-check for TypeScript errors
3. `cargo check` in `src-tauri/` -- Rust compilation check
4. `cargo clippy` in `src-tauri/` -- Rust linting

Fix any errors found. All checks must pass before committing.
Use Angular-style conventional commits: `<type>(<scope>): <subject>`

# Hope:RE - Coding Conventions

Code style standards for the TypeScript/Svelte frontend and the Rust backend. Most of these are enforced by ESLint (frontend) and rustfmt/clippy (backend); the rest are project conventions that reviews hold the line on. For architecture see [CODEBASE.md](CODEBASE.md); for setup, workflow, and releases see [CONTRIBUTE.md](CONTRIBUTE.md).

## Code Style - TypeScript / Svelte

### Formatting (enforced by ESLint, Prettier is disabled)

- 2 spaces indent
- Semicolons always
- Double quotes
- Max 2 attributes per line (single-line Svelte), 1 per line (multi-line)

### File Naming

- **kebab-case** for all `.ts`, `.svelte`, `.css` files (enforced by ESLint)
- Rust files use **snake_case** (excluded from kebab-case rule)
- Stores: `use-*.svelte.ts` pattern
- Component directories: `index.svelte` as entry, `index.ts` for barrel exports

### Type Definitions

- Always use `type` keyword, never `interface` (enforced: `ts/consistent-type-definitions`)
- Always use `import type` for type-only imports
- Export types from co-located `types.ts` files, re-export through barrel `index.ts`
- PascalCase for type names: `ProtectionSettings`, `AlgorithmParams`

### Import Order (enforced by `perfectionist/sort-imports`)

1. `import type { ... }` (type-only, separated by blank line)
2. External packages (`@lucide`, `@tanstack`, `@tauri-apps`, `svelte-sonner`, etc.)
3. Internal `$lib/...` imports
4. Relative imports (`./types`, `../utils`)

### Naming Conventions

- Variables/functions: `camelCase`
- Event handlers: `handle*` prefix (`handleProtect`, `handleDownload`)
- Hooks/stores: `use*` prefix (`useImage`, `useProtection`)
- Types: `PascalCase`
- Constants (scalars): `UPPER_SNAKE_CASE` (`TILE_SIZE`, `SUCCESS_RESET_MS`)
- Constants (arrays/objects): `camelCase` (`algorithms`, `qualityPresets`)
- Components: PascalCase in exports, kebab-case filenames

### Svelte 5 Runes (mandatory, no legacy syntax)

- Props: `$props()` with destructuring and defaults; `$bindable()` for two-way
- State: `$state<T>()` with explicit generic type
- Derived: `$derived(expression)`
- Effects: `$effect(() => { ... return cleanup; })`
- No `<style>` blocks; all styling via Tailwind CSS utility classes

### Styling (Tailwind CSS 4)

- Use logical inset utilities for RTL-aware positioning: prefer `inset-e-*` (inline-end) and `inset-s-*` (inline-start) over `end-*` / `start-*`. Example: `inset-e-4`, not `end-4`.

### State Management

- TanStack Svelte Query for server state (`createQuery`, `createMutation`)
- Svelte 5 rune-based composables for local state (`use-*.svelte.ts`)
- Expose reactive state via getters/setters in returned objects (preserves reactivity)
- Module-level `$state` for singleton stores, function-level for instance stores

### Internationalization (i18n)

- Supported locales: English (`en`), Vietnamese (`vi`), Japanese (`ja`), Chinese (`zh`).
- No i18n library. `lib/i18n/locales/en.ts` is the source of truth and exports
  `type Messages = typeof en`; `vi`/`ja`/`zh` are typed `Messages`, so `pnpm check`
  fails if any locale drifts out of key-sync. Add every new key to all four files.
- Never hardcode user-facing text (template nodes, `placeholder`/`title`/`aria-label`,
  `toast.*`, `sr-only`, progress messages). Use `t("dot.path")` from
  `$lib/stores/use-i18n.svelte`; it is reactive in `.svelte` and callable in `.svelte.ts`.
- Interpolate with `{name}` tokens: `t("image.loaded", { name })`. Unknown keys return
  the key; a locale missing a key falls back to English.
- Structural constants (`algorithms`, `glazeStyles`, `nightshadeTargets`, `qualityPresets`)
  hold only `value`/`key`/icons/colours; their labels/descriptions live in the dictionaries,
  keyed by `value`/`key` (e.g. algorithm labels resolve via `algorithms.<value>.label`).
- Locale is persisted to `settings.json` (via `useI18n().setLocale`), detected from
  `navigator.language` on first run, and switched from the header `LanguageSelect`.
- Decorative, non-informational marks stay untranslated (brand `Hope:RE`, calligraphy
  stamps `落款`/`印章`/`証`/`未`, the `ERASE` doodle).
- CJK glyphs are covered by system-font fallbacks appended to `--font-patrick-hand` in
  `app.css` (Patrick Hand has no CJK coverage).

### Error Handling (TypeScript)

- Wrap in `try/catch`, call `toast.error("message")` + `console.error("context:", error)`
- Extract messages: `e instanceof Error ? e.message : String(e)`
- `else` and `catch` on new line after closing brace (Allman-ish style)

### Comments and Emojis

- **No comments in code.** Do not add inline comments, block comments, or JSDoc to TypeScript,
  Svelte, or Rust files. The code should be self-documenting through clear naming.
- **No emojis in markdown or code.** Never use emoji characters in `.md` files, commit messages,
  toast messages, log messages, or anywhere else in the codebase.

### Restricted Patterns

- `no-console`: warn (prefer toast for user-facing messages)
- `node/no-process-env`: error (use Tauri APIs instead)
- `antfu/no-top-level-await`: off (allowed)

## Code Style - Rust

### Formatting (`rustfmt.toml`)

- Edition 2021, max width 100, 4 spaces, Unix newlines

### Module Organization

- `mod.rs` declares submodules and re-exports public API via `pub use`
- Private submodules: `mod name;` (no `pub`)
- Files: **snake_case**; Types: **PascalCase**; Constants: **UPPER_SNAKE_CASE**

### Tauri Commands

- Annotated with `#[tauri::command]`, always return `Result<T, String>`
- Errors via `.map_err(|e| format!("Descriptive message: {}", e))?`
- No custom error enums; all errors are `String`-typed
- Non-critical event emissions: `let _ = app.emit(...);`

### Structs

- Derive `Debug, Clone, serde::Serialize, serde::Deserialize` for data transfer types
- Use `Option<T>` for optional fields

### Platform-Specific Code

- Use `#[cfg(...)]` for conditional compilation (CUDA/DirectML/CoreML/XNNPACK)
- Stubs in `onnx_stubs.rs` for unsupported platforms

### Logging

- Use `log::info!`, `log::error!`, etc. (via Tauri log plugin)

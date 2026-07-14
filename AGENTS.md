# Hope:RE - Agent Instructions

AI art protection desktop app (adversarial perturbation via ONNX models).
Frontend: SvelteKit 2 + Svelte 5 + TypeScript + Tailwind CSS 4 + shadcn-svelte.
Backend: Rust (Tauri v2) with ONNX Runtime ML inference.
Design philosophy: minimalistic artistic Japanese Zen-like system design.

## Build / Dev / Lint Commands

```bash
pnpm install              # install frontend dependencies
pnpm dev                  # start Vite dev server (port 3000)
pnpm build                # production build (Vite SSG)
pnpm lint                 # ESLint check (strict, replaces Prettier)
pnpm format               # ESLint auto-fix
pnpm check                # svelte-kit sync + svelte-check (type checking)
pnpm check:watch          # same with --watch
pnpm tauri dev             # full desktop app dev (Rust + frontend)
pnpm tauri build           # production desktop build
```

Rust backend (run from `src-tauri/`):

```bash
cargo build               # build Rust backend
cargo check               # type check only
cargo clippy               # lint Rust code
cargo fmt                  # format Rust code (rustfmt)
```

## Testing

No test framework is currently configured. There are no test files in the repo.
If adding tests, use `vitest` for frontend and `cargo test` for Rust.

## Pre-commit Hooks

Husky runs `lint-staged` on pre-commit which executes `pnpm lint` on all staged files.
Always run `pnpm lint` before committing.

## Commit Convention

Angular-style conventional commits. Format: `<type>(<scope>): <subject>`

Types: `feat`, `fix`, `perf` (appear in changelog), `build`, `ci`, `docs`, `style`, `refactor`, `test`.
Subject: imperative present tense, no capital first letter, no trailing period.
Example: `feat(protection): add nightshade intensity control`

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

## Project Structure

```
src/                      # Frontend (SvelteKit + Svelte 5)
  lib/components/         # UI components (shadcn-svelte based)
  lib/queries/            # TanStack Query hooks (by domain)
  lib/stores/             # Svelte 5 rune composables (use-*.svelte.ts)
  lib/constants.ts        # Algorithm/style/preset definitions
  lib/utils.ts            # cn(), parseMarkdown, type helpers
  routes/                 # SvelteKit routes (SPA, SSR disabled)
src-tauri/                # Backend (Rust / Tauri v2)
  src/commands/           # Tauri command handlers
  src/onnx_integration/   # ONNX Runtime: models, protection pipeline
  src/system_info/        # System info gathering (CPU, GPU, memory)
src-models/               # ML training (Python/JAX notebooks, ONNX export)
```

## Key Dependencies

Frontend: `@tauri-apps/api`, `@tanstack/svelte-query`, `bits-ui`, `tailwind-merge`,
`clsx`, `svelte-sonner`, `marked`, `mode-watcher`, `lucide-svelte`.
Backend: `tauri` 2.x, `ort` (ONNX Runtime), `image`, `ndarray`, `reqwest`, `sysinfo`.

## CI/CD

- PR lint: `pnpm lint` on Ubuntu (Node 24, pnpm 10)
- Release: multi-platform Tauri build (macOS ARM, Windows x64, Linux amd64/arm64)
- ONNX models tracked via Git LFS, uploaded to GitHub Releases

## ONNX Model Pipeline

### Model Architecture

Three ONNX models (~350MB each) built on ViT-B/32 CLIP:

- `noise_algorithm.onnx` -- Adversarial noise (input: image only)
- `glaze_algorithm.onnx` -- Style cloaking (input: image + style_index 0-4)
- `nightshade_algorithm.onnx` -- Data poisoning (input: image + target_index 0-7)

All models: NHWC `(1, 224, 224, 3)` float32 in `[0.0, 1.0]`, scalar float32 loss output.
CLIP normalization is baked into the model graph.

### Training Pipeline (Google Colab)

```
0_setup_colab.ipynb    -> GPU check, JAX+CUDA install
1_clip_to_jax.ipynb    -> PyTorch CLIP -> numpy weights, pre-compute text embeddings
2_noise_algorithm.ipynb -> JAX noise model -> .pkl
3_glaze_algorithm.ipynb -> JAX glaze model -> .pkl
4_nightshade_algorithm.ipynb -> JAX nightshade model -> .pkl
5_export_onnx.ipynb    -> .pkl -> ONNX (jax2onnx, onnxsim, validate)
```

### Model Distribution Flow

1. Train in Colab, export `.onnx` to `src-models/models/`
2. Track with Git LFS (`.gitattributes`)
3. `publish.yml` uploads to GitHub Release after Tauri build
4. App downloads at runtime via `model_downloader.rs`
5. Stored in `app_data_dir/models/`

### Inference Pipeline (Rust)

```
Image -> base64 decode -> tile (224x224, overlap 32px)
  -> preprocess (normalize to [0,1]) -> edge weight map
  -> SPSA-PGD optimization (8 directions/iter, momentum 0.85)
  -> blend tiles -> encode output
```

### Config Reference

`src-models/models/hope_config.json` -- input specs, algorithm parameters, presets,
Glaze styles (Abstract/Impressionist/Cubist/Sketch/Watercolor),
Nightshade targets (Dog/Cat/Car/Landscape/Person/Building/Food/Abstract).

## OpenCode Skills and Agents

### Skills (`.opencode/skills/` and `.agents/skills/`)

Use these skills for specialized tasks. Load with `skill(name)` when starting relevant work.

#### Hope:RE Project Skills (`.opencode/skills/`)

| Skill                   | Description                                               |
| ----------------------- | --------------------------------------------------------- |
| `tauri-command`         | Create Rust Tauri v2 commands                             |
| `svelte-component`      | Create Svelte 5 components                                |
| `tanstack-query`        | Create TanStack Svelte Query hooks                        |
| `onnx-rust-integration` | Load and run ONNX models in Rust with `ort` crate         |
| `onnx-export`           | Convert JAX models to ONNX in Python/Colab                |
| `protection-algorithm`  | Implement SPSA-PGD adversarial perturbation pipeline      |
| `model-distribution`    | Handle model distribution via Git LFS and GitHub Releases |

#### General Skills (`.agents/skills/`)

| Skill                       | Description                                                                                     |
| --------------------------- | ----------------------------------------------------------------------------------------------- |
| `chrome-devtools`           | Chrome DevTools via MCP for debugging, browser automation, performance analysis                 |
| `svelte5-best-practices`    | Svelte 5 runes, snippets, SvelteKit patterns, TypeScript                                        |
| `svelte-code-writer`        | CLI tools for Svelte 5 documentation lookup and code analysis                                   |
| `context7`                  | Retrieve up-to-date documentation via Context7 API                                              |
| `rust-best-practices`       | Idiomatic Rust code, borrowing, ownership, error handling                                       |
| `typescript-advanced-types` | Generics, conditional types, mapped types, utility types                                        |
| `ui-ux-pro-max`             | UI/UX design intelligence: 50+ styles, 161 color palettes, typography, accessibility, 10 stacks |

#### Productivity Skills (global)

| Skill      | Description                                                     |
| ---------- | --------------------------------------------------------------- |
| `ponytail` | Laziest solution that works. Stdlib before custom, minimal code |
| `caveman`  | Ultra-compressed communication mode. 65% token reduction        |

### Skill Usage Guide

Always load the appropriate skill when working on related tasks:

#### Frontend Tasks

| Task                                 | Skills to Load                                 |
| ------------------------------------ | ---------------------------------------------- |
| Create Svelte component              | `svelte-component`, `svelte5-best-practices`   |
| Add TanStack Query hook              | `tanstack-query`                               |
| Fix/analyze Svelte code              | `svelte-code-writer`, `svelte5-best-practices` |
| Debug web pages / browser automation | `chrome-devtools`                              |
| TypeScript advanced types            | `typescript-advanced-types`                    |
| UI/UX design decisions               | `ui-ux-pro-max`                                |

#### Backend Tasks

| Task                   | Skills to Load                                  |
| ---------------------- | ----------------------------------------------- |
| Create Tauri command   | `tauri-command`                                 |
| ONNX model integration | `onnx-rust-integration`, `model-distribution`   |
| Protection algorithm   | `protection-algorithm`, `onnx-rust-integration` |
| Rust code review       | `rust-best-practices`                           |

#### ML/Model Tasks

| Task               | Skills to Load       |
| ------------------ | -------------------- |
| Export JAX to ONNX | `onnx-export`        |
| Download models    | `model-distribution` |

#### Productivity Modes

**Ponytail** (laziness mode):

- Load for: any coding task where simplicity is desired
- Use when: user says "ponytail", "be lazy", "simplest solution", "minimal", "yagni"
- Effect: Forces simplest solution, stdlib over dependencies, minimal code

**Caveman** (terse mode):

- Load for: every response when user requests it
- Use when: user says "caveman mode", "be brief", "less tokens"
- Effect: Ultra-compressed prose, same technical accuracy

### Agents (`.opencode/agents/`)

| Agent              | Description                                                        |
| ------------------ | ------------------------------------------------------------------ |
| `review`           | Code review for Svelte/TypeScript and Rust standards               |
| `explore-arch`     | Explore codebase architecture and dependencies                     |
| `ml-pipeline`      | ML notebook pipeline -- JAX training, CLIP extraction, ONNX export |
| `onnx-integration` | Rust ONNX integration -- model loading, inference, SPSA-PGD        |

### Commands (`.opencode/commands/`)

| Command       | Description                              |
| ------------- | ---------------------------------------- |
| `component`   | Create a new Svelte 5 component          |
| `lint`        | Run ESLint and fix errors                |
| `precommit`   | Full pre-commit validation pipeline      |
| `typecheck`   | Type-check frontend and backend          |
| `protect`     | Work with the image protection pipeline  |
| `onnx-export` | Work with the ONNX model export pipeline |

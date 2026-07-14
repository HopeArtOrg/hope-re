# Hope:RE - Agent Instructions

AI art protection desktop app (adversarial perturbation via ONNX models).
Frontend: SvelteKit 2 + Svelte 5 + TypeScript + Tailwind CSS 4 + shadcn-svelte.
Backend: Rust (Tauri v2) with ONNX Runtime ML inference.
Design philosophy: minimalistic artistic Japanese Zen-like system design.

## Documentation Map

The project documentation is split into five feature documents. Read the relevant one before starting work; this file only keeps the agent-specific tooling and a digest of the non-negotiable rules.

| Document                                     | Read it for                                                                                                        |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| [CODEBASE.md](CODEBASE.md)                   | Architecture: project structure, key dependencies, and the ONNX model pipeline (training, distribution, inference) |
| [CORE_FUNCTION.md](CORE_FUNCTION.md)         | Features: the Tauri command surface and how protection, watermarking, models, updates, and system info flow        |
| [CODING_CONVENTION.md](CODING_CONVENTION.md) | Code style: TypeScript/Svelte and Rust conventions, naming, imports, runes, state management, i18n rules           |
| [CONTRIBUTE.md](CONTRIBUTE.md)               | Workflow: build/dev/lint commands, testing, pre-commit hooks, commit convention, CI/CD, and the release flow       |
| [CI_CD_PIPELINE.md](CI_CD_PIPELINE.md)       | Pipelines: the lint gate, the tag-driven release build matrix, model upload, and required secrets                  |

## Non-negotiable Rules

The complete standards live in [CODING_CONVENTION.md](CODING_CONVENTION.md); these are the ones agents most often get wrong:

- No comments in code (TypeScript, Svelte, or Rust) and no emojis anywhere (markdown, commits, toasts, logs).
- kebab-case filenames for `.ts`/`.svelte`/`.css`; snake_case for Rust.
- Svelte 5 runes only (`$props`, `$state`, `$derived`, `$effect`); no legacy syntax, no `<style>` blocks.
- All user-facing text goes through `t("dot.path")` from `$lib/stores/use-i18n.svelte`; every new key is added to all four locale dictionaries (`en`/`vi`/`ja`/`zh`).
- Always run `pnpm lint` before committing; conventional commits (`<type>(<scope>): <subject>`).

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

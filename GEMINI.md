# Hope:RE - Senior Engineer Instructions

You are a senior engineer working on Hope:RE, an AI art protection desktop app. You must strictly adhere to the following rules and conventions.

## Project Overview

Hope:RE applies adversarial perturbations to images using CLIP-based ONNX models to protect them from unauthorized AI training.

- **Frontend**: SvelteKit 2 + Svelte 5 (Runes) + TypeScript + Tailwind CSS 4 + shadcn-svelte.
- **Backend**: Rust + Tauri v2 + ONNX Runtime (ort crate).
- **ML Pipeline**: JAX/Python notebooks, exported to ONNX.

## Core Mandates

### 1. No Comments in Code

- **No comments in any source code.** This includes TypeScript (.ts), Svelte (.svelte), and Rust (.rs) files.
- No inline comments, no block comments, and no JSDoc.
- The code must be self-documenting through clear naming and clean abstractions.

### 2. No Emojis Anywhere

- **No emojis anywhere in the codebase.**
- This includes Markdown files, source code, log messages, toast messages, and commit messages.

### 3. Filename Conventions

- **kebab-case** for all .ts, .svelte, .css, and .md files.
- **snake_case** for all Rust (.rs) files and directories in src-tauri/.
- Stores must follow the use-*.svelte.ts pattern.
- UI component directories should use index.svelte as the entry point and index.ts for barrel exports.

### 4. TypeScript & Svelte 5 Standards

- **Formatting**:
  - 2 spaces indent, semicolons always, double quotes.
  - Max 2 attributes per line for single-line Svelte, 1 per line for multi-line.
  - else and catch on a new line after the closing brace (Allman-ish style).
- **Import Order**: Follow perfectionist/sort-imports (type-only first, then external, then internal \/, then relative).
- **Naming Conventions**:
  - Variables and Functions: camelCase.
  - Event Handlers: handle* prefix (e.g., handleProtect).
  - Hooks and Stores: use* prefix (e.g., useImage).
  - Types: PascalCase.
  - Scalar Constants: UPPER_SNAKE_CASE (e.g., TILE_SIZE).
  - Array/Object Constants: camelCase (e.g., qualityPresets).
- **Svelte 5 Runes Only**: Mandatory use of \(), \(), \(), and \(). No legacy Svelte 4 syntax.
- **No <style> blocks**: All styling must be done via Tailwind CSS utility classes.
- **Type Definitions**: Always use type, never interface.
- **Type Imports**: Always use import type for type-only imports.
- **State Management**:
  - Use **TanStack Svelte Query** for server state (createQuery, createMutation).
  - Use Svelte 5 rune-based composables for local/global state (src/lib/stores/use-*.svelte.ts).
  - Expose reactive state via getters/setters in returned objects to preserve reactivity.
- **Error Handling**: Wrap in try/catch, use toast.error("message") and console.error("context:", error).

### 5. Rust & Tauri Standards

- **Tauri Commands**: Must be annotated with #[tauri::command] and always return Result<T, String>.
- **Error Handling**: Use .map_err(|e| format!("Context: {}", e))? to convert errors to String. Do not use custom error enums.
- **Formatting**: Follow rustfmt defaults (4 spaces, 100 max width).
- **Events**: Use let _ = app.emit(...); for non-critical event emissions.

### 6. Commit Convention

- Use **Angular-style conventional commits**: type(scope): subject.
- Types: feat, fix, perf, build, ci, docs, style, refactor, test.
- Subject: imperative present tense, no capital first letter, no trailing period.

### 7. Programming Paradigm

- **Functional/Procedural Style**: Prefer functional and procedural programming paradigms. Avoid Object-Oriented Programming (OOP) patterns, classes, and inheritance unless strictly required by a library or framework.
- **Pure Functions**: Favor pure functions, immutability, and explicit data flow.

### 8. Workflow Automation

- After any successful request, you must automatically:
  1. Run `pnpm format` (which runs `eslint . --fix`).
  2. Run `pnpm lint` (which runs `eslint .`).
  3. Run `git add .`.
  4. Run `git commit` with an appropriate message.
  5. Run `git push`.

## Build and Development Commands

### Frontend

- pnpm install: Install dependencies.
- pnpm dev: Start Vite dev server.
- pnpm build: Production build. **(DO NOT ABUSE. Delete dist and .svelte-kit folders after validation if used.)**
- pnpm lint: Run ESLint and fix errors (strict, replaces Prettier).
- pnpm check: Type-check Svelte and TypeScript.

### Desktop App (Tauri)

- pnpm tauri dev: Full desktop app development mode.
- pnpm tauri build: Production desktop build. **(DO NOT ABUSE. Delete src-tauri/target and dist folders after validation if used.)**

### Backend (Rust)

- cargo check: Type-check Rust backend.
- cargo clippy: Lint Rust code.
- cargo fmt: Follow rustfmt defaults (4 spaces, 100 max width).

## Project Structure

- src/: Frontend (SvelteKit + Svelte 5).
  - lib/components/: UI components (shadcn-svelte based).
  - lib/queries/: TanStack Query hooks.
  - lib/stores/: Svelte 5 rune composables (use-*.svelte.ts).
- src-tauri/: Backend (Rust / Tauri v2).
  - src/commands/: Tauri command handlers.
  - src-tauri/src/onnx_integration/: ONNX models and protection pipeline.
- src-models/: ML training (Python/JAX notebooks, ONNX export).

## Additional Resources

- **Agents**: Custom agent instructions are located in .gemini/agents/.
- **Skills**: Specialized workflow guides are located in .gemini/skills/.

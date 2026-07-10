---
name: hope-re
description: Primary skill for the Hope:RE AI art protection desktop application. Covers project overview, Zen design language, key conventions, and common workflows.
---

# Hope:RE

Primary skill for the Hope:RE AI art protection desktop application.

## When to Use

Load this skill when working on Hope:RE features, fixes, or general development tasks.

## Project Overview

Hope:RE is an AI art protection desktop app using adversarial perturbation via ONNX models.

- **Frontend**: SvelteKit 2 + Svelte 5 + TypeScript + Tailwind CSS 4 + shadcn-svelte
- **Backend**: Rust (Tauri v2) with ONNX Runtime ML inference
- **Design**: Minimalistic artistic Japanese Zen-like system design

## Design Philosophy

**Zen Artistic Design Language** - "digitized art studio" aesthetic

### Color Palette

Light Mode:
- Background: soft cream `oklch(0.94 0.015 80)`
- Foreground: charcoal gray `oklch(0.15 0.02 80)`
- Primary: deep violet `oklch(0.35 0.06 80)`

Dark Mode:
- Background: deep charcoal `oklch(0.25 0.02 80)`
- Foreground: soft white `oklch(0.98 0.01 80)`
- Primary: light violet `oklch(0.9 0.05 80)`

### Visual Elements

- **Doodle lines**: `border-radius: 255px 15px 225px 15px/15px 225px 15px 255px`
- **Doodle blobs**: `border-radius: 60% 40% 30% 70% / 60% 30% 70% 40%`
- **Textures**: Sticky notes, canvas sheets, dot grid background

### Artistic Metaphors

- "Original Canvas" / "Rendered Canvas" for image panels
- "Apply Ink" instead of "Protect"
- "New Sheet" instead of "Reset"
- "Protect Power" / "Patience Level" for sliders
- "Creative Idea" for algorithm selection
- "Style Vibe" for glaze styles

### Typography

- Font: Patrick Hand (handwritten, artistic feel)
- Tight letter spacing on headings
- Generous line height

## Key Commands

```bash
pnpm dev              # Vite dev server (port 3000)
pnpm tauri dev        # Full desktop app development
pnpm lint             # ESLint check
pnpm check            # Type check
```

Rust (from `src-tauri/`):
```bash
cargo build           # Build backend
cargo clippy          # Lint Rust
```

## Important Conventions

### Code Style

- Use `type`, never `interface`
- Svelte 5 runes only (no legacy syntax)
- No `<style>` blocks - Tailwind CSS only
- kebab-case for .ts/.svelte files
- No comments in code
- No emojis anywhere

### Commit Convention

Format: `<type>(<scope>): <subject>`

Types: `feat`, `fix`, `perf`, `build`, `ci`, `docs`, `style`, `refactor`, `test`

## Project Structure

```
src/                  # Frontend (SvelteKit + Svelte 5)
src-tauri/            # Backend (Rust / Tauri v2)
src-models/           # ML training notebooks
```

## Available Skills

Load specialized skills for specific tasks:

| Task | Skills |
|------|--------|
| Create Svelte component | `svelte-component`, `svelte5-best-practices` |
| Create Tauri command | `tauri-command` |
| ONNX model integration | `onnx-rust-integration` |
| Protection algorithm | `protection-algorithm` |
| Debug web pages | `chrome-devtools` |
| UI/UX design | `ui-ux-pro-max` |

See `AGENTS.md` for full skill list and usage guide.

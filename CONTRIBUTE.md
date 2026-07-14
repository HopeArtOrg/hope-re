# Hope:RE - Contributing Guide

How to set up, develop, test, commit, and release. For architecture see [CODEBASE.md](CODEBASE.md); for code style see [CODING_CONVENTION.md](CODING_CONVENTION.md).

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

## CI/CD

Full pipeline reference: [CI_CD_PIPELINE.md](CI_CD_PIPELINE.md).

- Lint (`lint.yml`): runs `pnpm lint` on every pull request into `main` (Ubuntu, Node 24, pnpm 11).
- Release (`publish.yml`): triggered by pushing a version tag matching `v*` (e.g. `v2.5.0`).
  Builds the multi-platform Tauri bundles (macOS ARM, Windows x64, Linux amd64/arm64),
  creates the GitHub release named after the tag with notes extracted from the matching
  `CHANGELOG.md` section, and uploads the ONNX models (Git LFS) to the release.

## Cutting a Release

Releases follow semantic versioning and are driven by `v*` tags, not by pushes to `main`.

1. Bump the version in `package.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`
   (`hope-ad` package), and `src-tauri/tauri.conf.json`. All four must match.
2. Add a `## [x.y.z]` section to `CHANGELOG.md`; the workflow extracts the release notes
   from it (tag `v2.5.0` maps to section `## [2.5.0]`).
3. Land the bump on `main` through a pull request.
4. Tag the release commit and push the tag:

```bash
git tag v2.5.0
git push origin v2.5.0
```

The tag push triggers `publish.yml`. Keep the embedded app version in sync with the tag
so the updater manifest matches the released build.

# Hope:RE - CI/CD Pipeline

Reference for the GitHub Actions pipelines: what runs, when, and what they produce. For the step-by-step release procedure see [CONTRIBUTE.md](CONTRIBUTE.md); for what the released app does at runtime see [CORE_FUNCTION.md](CORE_FUNCTION.md).

## Workflows

| Workflow                        | Trigger                             | Purpose                                            |
| ------------------------------- | ----------------------------------- | -------------------------------------------------- |
| `.github/workflows/lint.yml`    | Pull request into `main`            | Lint gate (`pnpm lint`)                            |
| `.github/workflows/publish.yml` | Push of a version tag matching `v*` | Multi-platform build, GitHub release, model upload |

## Lint Pipeline (`lint.yml`)

Runs on every pull request into `main`:

1. Checkout.
2. Node.js 24 and pnpm 11.11.0 (with cache).
3. `pnpm install --frozen-lockfile`.
4. `pnpm lint` (strict ESLint; it replaces Prettier and also covers JSON, YAML, and Markdown).

The same check runs locally before every commit: Husky's pre-commit hook executes `lint-staged`, which runs `pnpm lint` on staged files.

## Release Pipeline (`publish.yml`)

Triggered by pushing a semver tag such as `v2.5.0`. Releases are driven by tags, not by pushes to `main`.

### Job: `publish-tauri`

Build matrix:

| Runner                | Target                 |
| --------------------- | ---------------------- |
| `macos-latest`        | `aarch64-apple-darwin` |
| `windows-2025-vs2026` | Windows x86_64         |
| `ubuntu-24.04`        | Linux amd64            |
| `ubuntu-24.04-arm`    | Linux arm64            |

Each matrix leg:

1. Checkout, Node.js 24, pnpm 11.11.0, stable Rust (plus the macOS ARM target); Linux legs install the WebKitGTK/GTK system dependencies.
2. `pnpm install`.
3. Derives the version from the tag (`v2.5.0` becomes `2.5.0`).
4. Extracts the release notes from the matching `## [2.5.0]` section of `CHANGELOG.md` (falls back to a generic body if the section is missing).
5. `tauri-apps/tauri-action` builds the bundles and creates (or updates) the GitHub release named `Hope v2.5.0` on the pushed tag, uploading the installers and the signed updater artifacts (`latest.json`, NSIS-preferred on Windows).

### Job: `upload-models`

Runs after all `publish-tauri` legs succeed:

1. Checkout with Git LFS and pull the LFS objects.
2. Validate that every `src-models/models/*.onnx` is a real binary, not an LFS pointer file.
3. `gh release upload` attaches the ONNX models to the same tag's release.

The desktop app later downloads these models at first run; see [CORE_FUNCTION.md](CORE_FUNCTION.md).

## Required Secrets

| Secret                               | Used for                                             |
| ------------------------------------ | ---------------------------------------------------- |
| `GITHUB_TOKEN`                       | Creating the release and uploading assets (built in) |
| `TAURI_SIGNING_PRIVATE_KEY`          | Signing the updater artifacts                        |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Unlocking the signing key                            |

## Versioning Rules

- Semantic versioning; the `v*` tag is the single release trigger.
- The tag must match the version embedded in `package.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, and `src-tauri/tauri.conf.json`, and a matching `CHANGELOG.md` section must exist. The full checklist lives in [CONTRIBUTE.md](CONTRIBUTE.md).

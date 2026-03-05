# Hope:RE

<p align="center">
  <img src="src/lib/assets/favicon.svg" alt="Hope:RE" width="128" height="128" />
</p>

Hope:RE is an open-source desktop application that protects artwork from unauthorized AI training
and style mimicry. It applies adversarial perturbations to images using CLIP-based ONNX models,
making protected artwork resistant to generative AI systems that attempt to learn from or reproduce
an artist's style without consent.

The application ships as a native desktop app for Windows, macOS, and Linux.

Hope:RE is a tribute to the original [Hope](https://github.com/HopeADeff/Hope) app (version 1),
rebuilt from the ground up with a new architecture and modern toolchain. The original project is
also archived at [HopeArtOrg/hope-archive](https://github.com/HopeArtOrg/hope-archive).

## What It Does

Hope:RE offers three distinct protection algorithms, each targeting a different threat vector:

- **Noise** -- applies adversarial noise that disrupts AI feature extraction, making the image
  difficult for models to learn meaningful representations from.
- **Glaze** -- applies style-transfer perturbations that cloak the artist's visual style, causing
  AI models to misidentify the stylistic features of the work. Supports five target styles:
  Abstract, Impressionist, Cubist, Sketch, and Watercolor.
- **Nightshade** -- applies data poisoning perturbations that cause AI models trained on the
  protected image to associate it with an incorrect concept. Supports eight target concepts:
  Dog, Cat, Car, Landscape, Person, Building, Food, and Abstract.

All three algorithms operate on the same underlying pipeline: the image is tiled into overlapping
224x224 patches, each patch is optimized through an SPSA-PGD adversarial loop guided by a
CLIP-based ONNX model, and the results are blended back into a seamless full-resolution output.
Users control the tradeoff between protection strength and visual impact through intensity and
render quality sliders.

A detailed explanation of the adversarial perturbation mechanism, the SPSA gradient estimation
approach, and the CLIP loss formulation will be covered in a separate blog post.

## Technology Stack

### Frontend: SvelteKit + Svelte 5 + TypeScript + Tailwind CSS 4

The user interface is built with [SvelteKit 2](https://svelte.dev/docs/kit) and
[Svelte 5](https://svelte.dev/docs/svelte), compiled as a static single-page application via
`@sveltejs/adapter-static`. The UI component library is
[shadcn-svelte](https://www.shadcn-svelte.com/) (built on [bits-ui](https://bits-ui.com/)),
styled with [Tailwind CSS 4](https://tailwindcss.com/).

**Why Svelte 5.** Svelte compiles components to minimal imperative DOM operations at build time,
eliminating the virtual DOM diffing overhead found in React and Vue. Svelte 5's runes system
(`$state`, `$derived`, `$effect`, `$props`) provides fine-grained reactivity with explicit
opt-in semantics -- there is no hidden dependency tracking or stale closure footgun. The
compiled output is small (typically 30-50% less JavaScript than equivalent React bundles),
which matters for a desktop app where startup latency is noticeable.

**Why SvelteKit.** SvelteKit provides file-based routing, build tooling, and a plugin ecosystem
out of the box. Even though Hope:RE runs as a single-page app with SSR disabled, SvelteKit's
project structure and Vite integration offer a clean development experience with hot module
replacement and TypeScript path aliases (`$lib/`).

**Why Tailwind CSS 4.** Tailwind's utility-first approach eliminates the need for separate
stylesheet files and naming conventions. Version 4 introduces a Rust-based compiler (Oxide)
that is significantly faster than v3, and first-class CSS cascade layers that reduce specificity
conflicts. Co-locating styles as class names in markup keeps component files self-contained
and makes visual changes immediately greppable.

**Why TypeScript.** Static typing catches integration errors between frontend and backend early.
TypeScript's structural type system pairs well with Tauri's `invoke` API -- response types from
Rust commands are typed at the call site, ensuring frontend code stays in sync with backend
data shapes.

Async state (model status, system info, protection results) is managed through
[TanStack Svelte Query](https://tanstack.com/query/latest/docs/framework/svelte/overview),
which provides caching, deduplication, and mutation lifecycle management without hand-rolled
state machines. The app is fully static and client-side -- there is no backend server or network
API; all TanStack Query operations invoke local Tauri commands via IPC. Local UI state uses
Svelte 5 rune-based composables (`use-*.svelte.ts` files) that expose reactive getters and
setters.

### Backend: Rust + Tauri v2 + ONNX Runtime

The backend is written in [Rust](https://www.rust-lang.org/) and packaged as a native desktop
application using [Tauri v2](https://v2.tauri.app/). Machine learning inference runs through
[ONNX Runtime](https://onnxruntime.ai/) via the [`ort`](https://crates.io/crates/ort) crate.

**Why Rust.** The adversarial perturbation pipeline is computationally intensive -- each
protected image requires thousands of ONNX inference calls (two per direction per iteration
per tile). Rust's zero-cost abstractions and lack of garbage collector pauses make it suitable
for this kind of sustained numerical workload. Memory safety guarantees also prevent the class
of buffer overflow and use-after-free bugs that are common in C/C++ numerical code.

**Why Tauri v2.** Tauri produces significantly smaller application bundles than Electron
(typically 5-15 MB vs 150+ MB) because it uses the operating system's native webview instead
of shipping a full Chromium instance. Tauri v2 adds first-class plugin support, multi-window
management, and mobile platform targets (iOS and Android). The `#[tauri::command]` macro
provides a clean RPC boundary between the Rust backend and the JavaScript frontend -- functions
are annotated, automatically serialized via serde, and callable from the frontend through
`invoke()`.

**Why ONNX Runtime.** ONNX Runtime provides a single inference API that works across hardware
backends. Hope:RE uses platform-specific execution providers for GPU acceleration where
available:

| Platform        | Execution Providers |
| --------------- | ------------------- |
| Windows         | CUDA, DirectML, CPU |
| macOS / iOS     | CoreML, CPU         |
| Linux           | CUDA, XNNPACK, CPU  |
| Android (ARM64) | XNNPACK, CPU        |

This means the same ONNX model files work on all platforms without modification, and users
automatically benefit from GPU acceleration without configuration. The `ort` crate provides
Rust-native bindings with proper ownership semantics for sessions and tensors.

### ML Pipeline: JAX + Google Colab

The three ONNX models are trained in [JAX](https://jax.readthedocs.io/) on Google Colab with
GPU acceleration, then exported to ONNX format via `jax2onnx` and optimized with `onnxsim`.
The training pipeline is structured as a sequence of numbered Jupyter notebooks:

1. Environment setup and GPU verification
2. PyTorch CLIP weights converted to JAX/numpy format
3. Noise algorithm training
4. Glaze algorithm training
5. Nightshade algorithm training
6. ONNX export, simplification, and validation

Models are tracked with Git LFS, uploaded to GitHub Releases during CI, and downloaded by the
app at first launch.

**Why JAX.** JAX's functional transformation system (`jit`, `grad`, `vmap`) maps naturally to
the adversarial training loop, where gradients must be computed through both the perturbation
and the CLIP feature extractor. XLA compilation produces efficient GPU kernels without manual
CUDA programming. The numpy-compatible API also makes it straightforward to convert trained
parameters to ONNX via the standard export path.

## Project Structure

```
src/                          SvelteKit frontend
  lib/components/             UI components (shadcn-svelte based)
  lib/queries/                TanStack Query hooks (protection, models, system info)
  lib/stores/                 Svelte 5 rune composables (use-*.svelte.ts)
  lib/constants.ts            Algorithm definitions, presets, UI configuration
  routes/                     SvelteKit routes (single-page, SSR disabled)

src-tauri/                    Rust backend (Tauri v2)
  src/commands/               Tauri command handlers
  src/onnx_integration/       ONNX model loading, SPSA-PGD pipeline, tiling, encoding
  src/system_info/            Platform, CPU, GPU, and memory detection

src-models/                   ML training pipeline
  notebooks/                  Colab notebooks (JAX training, ONNX export)
  models/                     Trained ONNX model files (Git LFS)
```

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) 24+ with [pnpm](https://pnpm.io/) 10+
- [Rust](https://www.rust-lang.org/tools/install) 1.93+
- Platform-specific Tauri dependencies
  ([Windows](https://v2.tauri.app/start/prerequisites/#windows),
  [macOS](https://v2.tauri.app/start/prerequisites/#macos),
  [Linux](https://v2.tauri.app/start/prerequisites/#linux))

### Commands

```bash
pnpm install            # install frontend dependencies
pnpm dev                # start Vite dev server (port 3000)
pnpm build              # production frontend build
pnpm lint               # run ESLint
pnpm format             # run ESLint with auto-fix
pnpm check              # type-check Svelte and TypeScript

pnpm tauri dev          # full desktop app (Rust + frontend)
pnpm tauri build        # production desktop build
```

Rust-specific commands (run from `src-tauri/`):

```bash
cargo check             # type-check Rust backend
cargo clippy            # lint Rust code
cargo fmt               # format Rust code
```

### Release Builds

The CI pipeline (`publish.yml`) builds signed desktop installers on every push to `main`:

| Platform      | Target                      | Installer             |
| ------------- | --------------------------- | --------------------- |
| macOS         | `aarch64-apple-darwin`      | `.dmg`                |
| Windows       | `x86_64-pc-windows-msvc`    | `.msi`, `.exe` (NSIS) |
| Linux (amd64) | `x86_64-unknown-linux-gnu`  | `.deb`, `.AppImage`   |
| Linux (arm64) | `aarch64-unknown-linux-gnu` | `.deb`, `.AppImage`   |

ONNX models are uploaded to GitHub Releases alongside the installers. The built-in auto-updater
checks for new versions on launch and supports in-app download and install.

## License

[MIT](LICENSE) -- Copyright (c) 2025 HopeArtOrg

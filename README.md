# Hope:RE

<p align="center">
  <img src="src/lib/assets/favicon.svg" alt="Hope:RE" width="128" height="128" />
</p>

Hope:RE is an open-source desktop application designed to protect artwork from unauthorized generative AI training and style mimicry. It embeds adversarial perturbations into images using CLIP-based ONNX models, making protected artwork resistant to generative AI models that attempt to scrape and learn from an artist's style or content without consent.

The application compiles as a native desktop client for Windows, macOS, and Linux.

This project is a tribute to the original [Hope](https://github.com/HopeADeff/Hope) app (version 1), rewritten from the ground up with a new architecture and modern toolchain. The original project is archived at [HopeArtOrg/hope-archive](https://github.com/HopeArtOrg/hope-archive).

## Technical Problem and Solution

### The Threat: Unauthorized Style Mimicry and Concept Poisoning

Generative AI models are trained on billions of scraped web images, often without the consent of the original creators. This leads to two major problems for artists:

1. **Artistic Style Mimicry**: Generative models (often fine-tuned via LoRA) can extract the unique artistic representation, brushwork, color palette, and shading of an artist, allowing users to generate new works mimicking their style.
2. **Concept Association Corruption (Concept Poisoning)**: Text-to-image models associate images with surrounding text labels. Automated scrapers feed these pairs into training runs. If unchecked, this allows models to learn incorrect concepts (e.g. training on a poisoned image labeled "dog" makes the model generate cat-like features when prompted for a "dog").

### The Solution: Adversarial Perturbations in CLIP Space

Hope:RE mitigates these risks using **adversarial perturbations** that target the feature extraction stage of text-to-image models (specifically, Contrastive Language-Image Pretraining, or CLIP):

1. **CLIP Feature Disruption**: CLIP maps images and text into a shared embedding space. Hope:RE uses CLIP-based ONNX models to compute a loss function in the embedding space. The goal is to maximize the distance between the perturbed image's embedding and the original image's embedding, or to push the embedding toward a specific target style or concept.
2. **Optimization via SPSA-PGD**:
   - **PGD (Projected Gradient Descent)**: Iteratively updates pixels in the direction that maximizes the loss, then projects the updated image back onto an \epsilon-ball (bounding box) around the original image. This ensures the perturbation remains visually imperceptible.
   - **SPSA (Simultaneous Perturbation Stochastic Approximation)**: Because the ONNX model is treated as a black-box or requires gradient estimation over large dimensions on client hardware, SPSA approximates the gradient directions efficiently using a random Rademacher distribution. It requires only two model evaluations per direction per iteration, significantly reducing memory and compute overhead.
3. **Tiling and Blending**: To process high-resolution images under client VRAM/RAM constraints, the image is decomposed into overlapping 224x224 patches (tiles). Each patch is optimized independently, and they are combined back using a feathered blending mask to eliminate edge seams.

## Core Features

- **Noise**: Applies adversarial noise that disrupts AI feature extraction, making the image difficult for models to extract meaningful representations from.
- **Glaze**: Applies style-transfer perturbations that cloak the artist's visual style, causing AI models to misidentify the stylistic features of the work. Supports five target styles: Abstract, Impressionist, Cubist, Sketch, and Watercolor.
- **Nightshade**: Applies data poisoning perturbations that cause AI models trained on the protected image to associate it with an incorrect concept. Supports eight target concepts: Dog, Cat, Car, Landscape, Person, Building, Food, and Abstract.
- **Signature Ink (Blind Watermark)**: Embeds a hidden, robust digital signature within the image's frequency coefficients (using a hybrid DWT-DCT-SVD algorithm). The signature is imperceptible to the human eye but remains decodable after cropping, compression, or screenshotting.

## Signature Ink Mechanism

The **Signature Ink (Blind Watermark)** feature uses a Rust-native implementation of a robust blind watermarking algorithm from the [blind-watermark-rust](https://github.com/naganohara-yoshino/blind-watermark-rust) repository (originally ported from Guo Fei's Python implementation).

### How It Works

This feature utilizes a hybrid DWT-DCT-SVD (Discrete Wavelet Transform, Discrete Cosine Transform, and Singular Value Decomposition) pipeline to embed digital signatures invisibly and robustly:

1. **Discrete Wavelet Transform (DWT)**:
   The cover image is decomposed using a 2D wavelet transform to split the image into frequency sub-bands (approximation, horizontal, vertical, and diagonal coefficients).
2. **Discrete Cosine Transform (DCT)**:
   A DCT is applied to the low-frequency approximation sub-band. This concentrates the visual energy of the image into a few low-frequency coefficients, making modifications more resistant to lossy compression.
3. **Singular Value Decomposition (SVD)**:
   An SVD is performed on the DCT coefficient blocks to factorize them into three matrices ($U, \Sigma, V^T$). The diagonal matrix $\Sigma$ containing the singular values represents the fundamental structural properties of the image block.
4. **Watermark Perturbation**:
   The watermark string is converted into a binary bit-stream (which can be pseudo-randomly scrambled using a custom numeric seed key for cryptographic security). The singular values in $\Sigma$ are slightly perturbed to encode these watermark bits.
5. **Reconstruction**:
   The inverse singular value decomposition, inverse discrete cosine transform, and inverse discrete wavelet transform are applied sequentially to reconstruct the signed canvas.

During verification, the reverse transformation is performed on the signed image. The singular values are evaluated to reconstruct the bit-stream, which is decrypted using the same seed key to reveal the signature text.

### Verification Integrity

Because blind watermarking decodes coefficients from any image frequency domain, analyzing an unwatermarked image would normally output random garbage characters and falsely trigger a "Verified" status. To prevent false positives:

- The app prepends a secure prefix (`"HOPE:"`) to the watermark text before embedding.
- During verification, the app validates the presence of this prefix using a similarity check (permitting a 1-character Hamming distance to stay robust against noise/compression).
- If the prefix is absent or corrupted beyond the threshold, the verification fails and is marked as "Void".

Because the signature is embedded within the core structural singular values of the low-frequency coefficients, it is highly resilient against common edits, resizing, cropping, screenshotting, and lossy JPEG compression.

## Technology Stack

### Frontend: SvelteKit + Svelte 5 + TypeScript + Tailwind CSS 4

The user interface is built with SvelteKit 2 and Svelte 5, compiled as a static single-page application via `@sveltejs/adapter-static`. The UI component library is shadcn-svelte (built on bits-ui), styled with Tailwind CSS 4.

- **Svelte 5 Runes**: Svelte compiles components to minimal, fine-grained imperative DOM updates at build time, eliminating virtual DOM diffing. Svelte 5's runes system (`$state`, `$derived`, `$effect`, `$props`) provides reactive state management with clean scoping, reducing client bundle size and latency.
- **SvelteKit**: Manages routing, build configurations, and path aliases (`$lib/`), compiling down to a static single-page app with SSR disabled.
- **Tailwind CSS 4**: Utilizes a fast Rust-based compiler (Oxide) and native CSS cascade layers, keeping styling co-located in markups.
- **TypeScript**: Ensures type safety across frontend and backend boundaries. Response shapes from Tauri Rust commands are typed at the invocation call sites.

Async state (model status, system info, protection results) is managed via TanStack Svelte Query, wrappered around local Tauri commands via IPC. Local UI state uses Svelte 5 rune-based composables (`use-*.svelte.ts` files) that expose reactive getters and setters.

### Backend: Rust + Tauri v2 + ONNX Runtime

The backend is written in Rust and packaged as a native desktop application using Tauri v2. Machine learning inference runs through ONNX Runtime via the `ort` crate.

- **Rust**: Provides the raw execution speed required for thousands of ONNX inference calls per protection session. Zero-cost abstractions and memory safety guarantees ensure stable numerical operations without GC pauses.
- **Tauri v2**: Uses the operating system's native webview to produce small bundle sizes (5-15 MB vs Electron's 150+ MB). The `#[tauri::command]` macro provides a serialization boundary for IPC calls.
- **ONNX Runtime**: Enables platform-specific execution providers for hardware acceleration:
  - Windows: CUDA, DirectML, CPU
  - macOS / iOS: CoreML, CPU
  - Linux: CUDA, XNNPACK, CPU
  - Android (ARM64): XNNPACK, CPU

### ML Pipeline: JAX + Google Colab

The three ONNX models are built in JAX on Google Colab with GPU acceleration, then exported to ONNX format via `jax2onnx` and optimized with `onnxsim`. The training pipeline is structured as a sequence of numbered Jupyter notebooks:

1. Environment setup and GPU verification
2. PyTorch CLIP weights converted to JAX/numpy format
3. Noise algorithm training
4. Glaze algorithm training
5. Nightshade algorithm training
6. ONNX export, simplification, and validation

Models are tracked with Git LFS, uploaded to GitHub Releases during CI, and downloaded by the app at first launch.

- **JAX**: Provides native automatic differentiation (`grad`) and XLA compilation to compile CLIP gradients through the SPSA-PGD perturbation steps.

## Project Structure

- [src/](file:///D:/git-projects/hope-re/src) -- SvelteKit frontend
  - [lib/components/](file:///D:/git-projects/hope-re/src/lib/components) -- UI components (shadcn-svelte based)
  - [lib/queries/](file:///D:/git-projects/hope-re/src/lib/queries) -- TanStack Query hooks (protection, watermark, models, system info)
  - [lib/stores/](file:///D:/git-projects/hope-re/src/lib/stores) -- Svelte 5 rune composables and shared states (use-*.svelte.ts)
  - [lib/constants.ts](file:///D:/git-projects/hope-re/src/lib/constants.ts) -- Algorithm definitions, presets, UI configuration
  - [routes/](file:///D:/git-projects/hope-re/src/routes) -- SvelteKit routes (single-page, SSR disabled)
- [src-tauri/](file:///D:/git-projects/hope-re/src-tauri) -- Rust backend (Tauri v2)
  - [src/commands/](file:///D:/git-projects/hope-re/src-tauri/src/commands) -- Tauri command handlers
  - [src/onnx_integration/](file:///D:/git-projects/hope-re/src-tauri/src/onnx_integration) -- ONNX model loading, SPSA-PGD pipeline, tiling, encoding
  - [src/blind_watermark/](file:///D:/git-projects/hope-re/src-tauri/src/blind_watermark) -- DWT-DCT-SVD robust blind watermarking implementation
  - [src/system_info/](file:///D:/git-projects/hope-re/src-tauri/src/system_info) -- Platform, CPU, GPU, and memory detection
- [src-models/](file:///D:/git-projects/hope-re/src-models) -- ML training pipeline
  - [notebooks/](file:///D:/git-projects/hope-re/src-models/notebooks) -- Colab notebooks (JAX training, ONNX export)
  - [models/](file:///D:/git-projects/hope-re/src-models/models) -- Trained ONNX model files (Git LFS)

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) 24+ with [pnpm](https://pnpm.io/) 11+
- [Rust](https://www.rust-lang.org/tools/install) 1.93+
- Platform-specific Tauri dependencies ([Windows](https://v2.tauri.app/start/prerequisites/#windows), [macOS](https://v2.tauri.app/start/prerequisites/#macos), [Linux](https://v2.tauri.app/start/prerequisites/#linux))

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

ONNX models are uploaded to GitHub Releases alongside the installers. The built-in auto-updater checks for new versions on launch and supports in-app download and install.

## License

[MIT](LICENSE) -- Copyright (c) 2025 HopeArtOrg

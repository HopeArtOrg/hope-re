# Hope:RE - Codebase Guide

AI art protection desktop app (adversarial perturbation via ONNX models).
Frontend: SvelteKit 2 + Svelte 5 + TypeScript + Tailwind CSS 4 + shadcn-svelte.
Backend: Rust (Tauri v2) with ONNX Runtime ML inference.
Design philosophy: minimalistic artistic Japanese Zen-like system design.

This document covers the architecture: how the repository is laid out, what the app depends on, and how the ONNX model pipeline works end to end. For feature-level behavior and the Tauri command surface see [CORE_FUNCTION.md](CORE_FUNCTION.md); for code style see [CODING_CONVENTION.md](CODING_CONVENTION.md); for setup, workflow, and releases see [CONTRIBUTE.md](CONTRIBUTE.md).

## Project Structure

```
src/                      # Frontend (SvelteKit + Svelte 5)
  lib/components/         # UI components (shadcn-svelte based)
  lib/queries/            # TanStack Query hooks (by domain)
  lib/stores/             # Svelte 5 rune composables (use-*.svelte.ts)
  lib/i18n/               # Message dictionaries (en/vi/ja/zh) + Messages type
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

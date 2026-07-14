# Hope:RE - Core Functions

What the app does and how each feature flows from the Svelte UI through the Tauri command layer into Rust. For the underlying architecture and ONNX pipeline internals see [CODEBASE.md](CODEBASE.md); for code style see [CODING_CONVENTION.md](CODING_CONVENTION.md).

## Tauri Command Surface

All backend calls go through `invoke()` wrappers in `src/lib/queries/` (TanStack Query hooks). Registered commands (`src-tauri/src/lib.rs`):

| Command                      | Input                                  | Output                  | Purpose                                 |
| ---------------------------- | -------------------------------------- | ----------------------- | --------------------------------------- |
| `protect_image`              | `imageBase64`, `settings`              | `ProtectionResult`      | Run the protection pipeline             |
| `cancel_protection`          | none                                   | none                    | Cancel a running protection job         |
| `embed_watermark`            | `imageBase64`, `watermark`, `seed?`    | signed image (base64)   | Embed a blind watermark                 |
| `extract_watermark`          | `imageBase64`, `watermarkLen`, `seed?` | extracted text          | Recover an embedded watermark           |
| `check_models_status`        | none                                   | `ModelsCheckResult`     | Report which ONNX models exist locally  |
| `download_model`             | `modelName`                            | status string           | Download one ONNX model                 |
| `get_inference_capabilities` | none                                   | `InferenceCapabilities` | List available execution providers      |
| `get_system_info`            | none                                   | `SystemInfo`            | CPU/GPU/memory/storage/platform details |

## Image Protection (Cloak Canvas)

Adversarial perturbation that disrupts AI training on the artwork.

- Algorithms: `noise` (adversarial noise), `glaze` (style cloaking with a style index), `nightshade` (data poisoning toward a target concept).
- UI: `glaze-tab` + `ProtectionMenu` collect algorithm, intensity, output quality, render quality, and the conditional glaze style or nightshade target. `buildProtectionSettings()` normalizes them (intensity slider value divided by 100) into `ProtectionSettings`.
- Execution: `use-protection.svelte.ts` calls `protect_image` and subscribes to the `protection-progress` event (`stage`, `tile_current`/`tile_total`, `iteration_current`/`iteration_total`, `percent`); stages are `loading`, `processing`, `encoding`, `complete`.
- Result: `ProtectionResult { image_base64, success, message, model_used }`. When the ONNX models are unavailable the backend falls back to basic protection and `model_used` is `false`; the UI shows a fallback warning.
- Cancellation: `cancel_protection` stops the run; the store treats user cancellation as a non-error reset.

## Blind Watermarking (Signature Ink)

Frequency-domain signature hiding via the `blind_watermark` crate; robust to cropping, compression, and screenshots.

- Embed (`embed_watermark`): hides UTF-8 signature text, optionally keyed by a numeric seed. The UI records the signature length in UTF-8 bytes and can hand the signed canvas straight to the verifier (`use-watermark-state.svelte.ts` carries image, length, and seed between the two sub-tabs).
- Extract (`extract_watermark`): recovers `watermarkLen` bytes, optionally with the seed. The frontend validates the result with a printability check (at least 70 percent printable characters) before declaring the signature verified; garbage output is reported as "no valid signature".

## Model Management

The three ONNX models (~350MB each) are not bundled; they are fetched at runtime.

- Source: `https://github.com/HopeArtOrg/hope-re/releases/download` (uploaded by the release pipeline; see [CI_CD_PIPELINE.md](CI_CD_PIPELINE.md)).
- `check_models_status` reports per-model existence; `ResourceDownloadGuard` wraps the app and auto-starts `download_model` for anything missing, streaming `model-download-progress` events (bytes and percent per model).
- Storage: `app_data_dir/models/`. Downloads use timeouts, clean up temp files on failure, and the dialog can be minimized to a dock chip while downloads continue.

## Inference Capabilities

`get_inference_capabilities` returns the active ONNX Runtime execution providers for the current platform (CUDA, DirectML, CoreML, XNNPACK, or CPU fallback, selected via `#[cfg(...)]`). The UI surfaces them as a badge next to the protection menu.

## System Info (Studio Vitals)

`get_system_info` gathers OS/hostname, CPU, memory, GPU, storage, and the app version via the `system_info` module; the header dialog renders it with a manual refresh.

## Auto-Update

`tauri-plugin-updater` drives updates: `use-updater.svelte.ts` checks for a newer release, shows the release notes (markdown rendered with `marked` and sanitized with DOMPurify via `parseMarkdown`), then downloads, installs, and relaunches. Active downloads can be minimized to the dock; failures surface in the dialog and as toasts.

## Localization and Theming

- Four locales (`en`, `vi`, `ja`, `zh`) with a header language switcher; every user-facing string resolves through `t()` (rules in [CODING_CONVENTION.md](CODING_CONVENTION.md)).
- Light/dark/system theme via `use-theme.svelte.ts`.
- Both preferences persist to the Tauri store file `settings.json`.

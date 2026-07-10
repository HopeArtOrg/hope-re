---
name: onnx-rust-integration
description: Load and run ONNX models in the Rust Tauri backend using the ort crate with platform-specific execution providers
---

## ONNX Rust Integration Pattern

When working with ONNX models in the Hope:RE Rust backend, follow these conventions:

### Model Specification

Three ONNX models exist, each ~350MB, built on ViT-B/32 CLIP:

| Model | File | Inputs | Output |
|-------|------|--------|--------|
| Noise | `noise_algorithm.onnx` | `input: (1,224,224,3) f32` | scalar f32 loss |
| Glaze | `glaze_algorithm.onnx` | `input: (1,224,224,3) f32` + `style_index: (1,) i32` | scalar f32 loss |
| Nightshade | `nightshade_algorithm.onnx` | `input: (1,224,224,3) f32` + `target_index: (1,) i32` | scalar f32 loss |

All models expect NHWC format, float32 in `[0.0, 1.0]` range. CLIP normalization is baked into the model -- Rust only provides raw pixel values.

### Loading a Model

```rust
use ort::session::Session;

pub fn load_model(model_path: &std::path::Path) -> Result<Session, String> {
    let model_bytes = std::fs::read(model_path)
        .map_err(|e| format!("Failed to read model at {:?}: {}", model_path, e))?;

    if model_bytes.starts_with(b"version https://git-lfs.github.com/spec/") {
        return Err(format!(
            "Model at {:?} is a Git LFS pointer, not an actual ONNX file. Run `git lfs pull`",
            model_path
        ));
    }

    Session::builder()
        .map_err(|e| format!("Failed to create session builder: {}", e))?
        .with_optimization_level(ort::session::builder::GraphOptimizationLevel::Level3)
        .map_err(|e| format!("Failed to set optimization level: {}", e))?
        .commit_from_memory(&model_bytes)
        .map_err(|e| format!("Failed to load model from {:?}: {}", model_path, e))
}
```

### Resolving Model Paths

Models are resolved with a priority chain:
1. Downloaded models in `app_data_dir/models/` (from GitHub Releases)
2. Bundled resource directory candidates
3. Dev-mode paths relative to `CARGO_MANIFEST_DIR` (debug builds only)

```rust
use std::path::PathBuf;

pub fn resolve_model_path(app: &tauri::AppHandle, model_filename: &str) -> Result<PathBuf, String> {
    use tauri::Manager;

    if let Some(downloaded) = resolve_downloaded_model(app, model_filename) {
        return Ok(downloaded);
    }

    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to resolve resource directory: {}", e))?;

    let candidates = [
        resource_dir.join(model_filename),
        resource_dir.join("models").join(model_filename),
    ];

    for path in &candidates {
        if path.exists() {
            return Ok(path.clone());
        }
    }

    Err(format!("Model file {} not found", model_filename))
}
```

### Running Inference (Single Input)

```rust
use ndarray::Array4;
use ort::session::Session;
use ort::value::Tensor;

pub fn run_noise_model(session: &mut Session, input: &Array4<f32>) -> Result<f32, String> {
    let shape = input.shape();
    let data: Vec<f32> = input.iter().copied().collect();
    let input_tensor = Tensor::from_array((
        [shape[0], shape[1], shape[2], shape[3]],
        data.into_boxed_slice(),
    ))
    .map_err(|e| format!("Failed to create input tensor: {}", e))?;

    let outputs = session
        .run(ort::inputs![input_tensor])
        .map_err(|e| format!("Failed to run model: {}", e))?;

    outputs[0]
        .try_extract_scalar::<f32>()
        .map_err(|e| format!("Failed to extract output: {}", e))
}
```

### Running Inference (With Extra Index Input)

```rust
pub fn run_glaze_model(
    session: &mut Session,
    input: &Array4<f32>,
    style_index: i32,
) -> Result<f32, String> {
    let shape = input.shape();
    let data: Vec<f32> = input.iter().copied().collect();
    let input_tensor = Tensor::from_array((
        [shape[0], shape[1], shape[2], shape[3]],
        data.into_boxed_slice(),
    ))
    .map_err(|e| format!("Failed to create input tensor: {}", e))?;

    let style_tensor = Tensor::from_array(([1_usize], vec![style_index]))
        .map_err(|e| format!("Failed to create style index tensor: {}", e))?;

    let outputs = session
        .run(ort::inputs![input_tensor, style_tensor])
        .map_err(|e| format!("Failed to run model: {}", e))?;

    outputs[0]
        .try_extract_scalar::<f32>()
        .map_err(|e| format!("Failed to extract output: {}", e))
}
```

### Platform-Specific Execution Providers

Use `#[cfg(...)]` for conditional compilation. Priority order per platform:

| Platform | Providers (priority order) |
|----------|---------------------------|
| Windows | CUDA > DirectML > CPU |
| macOS/iOS | CoreML > CPU |
| Linux (x86_64/aarch64) | CUDA > XNNPACK > CPU |
| Android | XNNPACK > CPU |

```rust
#[cfg(target_os = "windows")]
pub fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
    use ort::ep::{DirectML, ExecutionProvider, CUDA};

    let mut eps: Vec<ort::ep::ExecutionProviderDispatch> = Vec::new();

    if has_nvidia_gpu() && CUDA::default().is_available().unwrap_or(false) {
        eps.push(CUDA::default().build());
    }
    if DirectML::default().is_available().unwrap_or(false) {
        eps.push(DirectML::default().build());
    }

    eps
}
```

### Index Mappings

Glaze styles: `abstract=0, impressionist=1, cubist=2, sketch=3, watercolor=4`

Nightshade targets: `dog=0, cat=1, car=2, landscape=3, person=4, building=5, food=6, abstract=7`

### Key Crate Dependencies

```toml
ort = { version = "2", features = ["cuda", "directml", "coreml", "xnnpack"] }
ndarray = "0.16"
image = "0.25"
```

### Rules

- Always detect Git LFS pointers before loading -- fail with a descriptive error
- Use `.map_err(|e| format!("Descriptive message: {}", e))?` for all fallible operations
- Return `Result<T, String>` from all public functions -- no custom error enums
- Use `log::info!`, `log::error!` for logging
- Use `Option<T>` for optional fields in structs
- Derive `Debug, Clone, serde::Serialize, serde::Deserialize` on data transfer structs
- Files are snake_case, types are PascalCase, constants are UPPER_SNAKE_CASE
- No comments in code -- self-documenting through clear naming
- No emojis anywhere in the codebase
- Provide fallback behavior when models are unavailable
- Config reference: `src-models/models/hope_config.json`

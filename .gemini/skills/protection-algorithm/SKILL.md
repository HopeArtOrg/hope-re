---
name: protection-algorithm
description: Implement the SPSA-PGD adversarial perturbation pipeline for image protection in Rust
---

## Protection Algorithm Pattern

When working with the SPSA-PGD adversarial perturbation pipeline in Hope:RE, follow these conventions:

### Pipeline Overview

```
Image (any size)
  -> Decode base64
  -> Tile into 224x224 overlapping patches (stride = TILE_SIZE - TILE_OVERLAP)
  -> For each tile:
       -> Preprocess: crop, resize to 224x224, normalize to [0.0, 1.0]
       -> Compute edge weight map (Sobel-based)
       -> Run SPSA-PGD optimization loop against ONNX model
       -> Produce perturbed tile
  -> Blend tiles back (weighted overlap blending)
  -> Encode to base64 (JPEG or PNG)
```

### Constants

```rust
pub const TILE_SIZE: u32 = 224;
pub const TILE_OVERLAP: u32 = 32;
pub const SPSA_DIRECTIONS_PER_ITER: usize = 8;
```

### Core Types

```rust
pub type ModelRunFn = dyn FnMut(&mut Session, &Array4<f32>) -> Result<f32, String>;

#[derive(Debug, Clone)]
pub struct AlgorithmParams {
    pub epsilon: f32,
    pub max_iterations: u32,
    pub alpha_multiplier: f32,
    pub perceptual_weight: f32,
}
```

### Algorithm Parameter Functions

Each algorithm computes `AlgorithmParams` from a user-facing intensity slider (0.0-1.0):

```rust
pub fn get_noise_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.08 / 0.5,
        max_iterations: 250,
        alpha_multiplier: 3.0,
        perceptual_weight: 0.4,
    }
}

pub fn get_glaze_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.05 / 0.5,
        max_iterations: 350,
        alpha_multiplier: 2.5,
        perceptual_weight: 0.8,
    }
}

pub fn get_nightshade_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.045 / 0.5,
        max_iterations: 500,
        alpha_multiplier: 3.0,
        perceptual_weight: 1.2,
    }
}
```

### SPSA-PGD Optimization Loop

The core optimization runs per-tile with these steps each iteration:

1. Compute decaying step sizes: `ck = ck_initial / (k+1)^0.101`, `ak = alpha / (k+1)^0.602`
2. For each of `SPSA_DIRECTIONS_PER_ITER` (8) random directions:
   a. Generate random Rademacher direction vector (+1/-1 per element)
   b. Create `plus = base + perturbation + ck * direction` and `minus = base + perturbation - ck * direction`
   c. Clamp both to `[0.0, 1.0]`
   d. Compute perceptual loss difference (weighted by inverse edge map)
   e. Run ONNX model on both `plus` and `minus` tiles
   f. Accumulate gradient estimate: `(loss_plus - loss_minus + p_loss_diff) / (2 * ck) * direction`
3. Average gradient over valid directions
4. Apply momentum (beta = 0.85): `momentum = beta * momentum + (1 - beta) * weighted_grad`
5. Update perturbation with sign-based step: `perturbation -= ak * sign(momentum)`
6. Clamp perturbation to `[-epsilon, epsilon]`

```rust
pub fn spsa_pgd_on_tile(
    session: &mut Session,
    base_tile: &Array4<f32>,
    params: &AlgorithmParams,
    iterations: u32,
    run_model: &mut ModelRunFn,
    edge_weights: &[f32],
    progress: &TileProgress,
) -> Result<Array4<f32>, String>
```

### Preprocessing

Tiles are extracted and normalized to `[0.0, 1.0]` NHWC format:

```rust
pub fn preprocess_tile(img: &DynamicImage, x: u32, y: u32, w: u32, h: u32) -> Array4<f32> {
    let cropped = img.crop_imm(x, y, w, h);
    let resized = cropped.resize_exact(TILE_SIZE, TILE_SIZE, image::imageops::FilterType::Triangle);
    let rgba = resized.to_rgba8();

    let data: Vec<f32> = rgba
        .pixels()
        .flat_map(|p| {
            let Rgba([r, g, b, _]) = *p;
            [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
        })
        .collect();

    Array::from_shape_vec((1, TILE_SIZE as usize, TILE_SIZE as usize, 3), data)
        .unwrap_or_else(|_| Array4::zeros((1, TILE_SIZE as usize, TILE_SIZE as usize, 3)))
}
```

### Edge Weight Map

Edge detection guides perturbation strength -- more perturbation in high-detail areas:

```rust
pub fn compute_edge_weight_map(tile: &Array4<f32>) -> Vec<f32>
```

Returns per-pixel weights in `[0.3, 1.0]` range. Used to:
- Weight the gradient in SPSA (more aggressive near edges)
- Modulate the perceptual loss penalty (less penalty near edges)

### Tile Blending

Overlapping tiles are blended with distance-to-edge weights:

```rust
fn blend_tile_direct(
    protected_tile: &Array4<f32>,
    region: &TileRegion,
    width: u32,
    result_accum: &mut [f32],
    weight_accum: &mut [f32],
)
```

Blending weight: `min(edge_x, edge_y)` where `edge_x = min(px, w-1-px, TILE_OVERLAP) / TILE_OVERLAP`. Supports bilinear interpolation when tile dimensions differ from `TILE_SIZE`.

### Tauri Command Integration

```rust
#[tauri::command]
pub async fn protect_image(
    app: tauri::AppHandle,
    image_base64: String,
    settings: ProtectionSettings,
) -> Result<ProtectionResult, String>
```

The command:
1. Decodes base64 image (strips data URI prefix)
2. Resolves algorithm params from `settings.intensity` and `settings.render_quality`
3. Attempts to load ONNX model; falls back to procedural noise if unavailable
4. Emits `protection-progress` events throughout (stages: loading, processing, encoding, complete)
5. Encodes result as JPEG (quality < 100) or PNG (quality = 100)

### Fallback Noise

When the ONNX model is unavailable, `apply_fallback_noise` applies procedural multi-scale noise with frequency-domain wave patterns. Not ML-based but provides visual perturbation.

### Progress Events

Emit `ProtectionProgress` via `app.emit("protection-progress", ...)`:

```rust
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProtectionProgress {
    pub stage: String,
    pub tile_current: u32,
    pub tile_total: u32,
    pub iteration_current: u32,
    pub iteration_total: u32,
    pub percent: f64,
}
```

Stages: `"loading"` (2%) -> `"processing"` (0-95%) -> `"encoding"` (96%) -> `"complete"` (100%)

### Module Structure

```
src-tauri/src/onnx_integration/protection/
  mod.rs          -- protect_image command, algorithm dispatch
  types.rs        -- ProtectionSettings, ProtectionResult, AlgorithmParams, constants
  algorithms.rs   -- get_*_params(), get_*_index(), run_*_model()
  preprocessing.rs -- preprocess_tile(), compute_edge_weight_map()
  spsa.rs         -- spsa_pgd_on_tile(), Xoshiro128 RNG
  tiling.rs       -- apply_model_protection(), blend_tile_direct()
  model.rs        -- load_model(), resolve_model_path()
  encoding.rs     -- encode_image_to_base64(), apply_fallback_noise()
```

### Rules

- Render quality scales iterations: `iterations = max_iterations * render_factor`
- Always clamp pixel values to `[0.0, 1.0]` and perturbation to `[-epsilon, epsilon]`
- Tolerate up to 5 consecutive model inference failures before aborting
- Emit progress events every 20 iterations (not every iteration -- throttle for performance)
- Use `let _ = app.emit(...)` for non-critical progress emissions
- Preserve alpha channel from the original image
- No comments in code -- self-documenting through clear naming
- No emojis anywhere in the codebase

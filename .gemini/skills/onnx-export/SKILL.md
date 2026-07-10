---
name: onnx-export
description: Convert JAX/Python ML models to ONNX format following the Hope:RE notebook pipeline for Google Colab
---

## ONNX Export Pipeline

When creating or modifying the ML training-to-ONNX export pipeline in Hope:RE, follow these conventions:

### Pipeline Overview (6 Notebooks, Google Colab)

```
0_setup_colab.ipynb    -> GPU check, JAX+CUDA install
1_clip_to_jax.ipynb    -> PyTorch CLIP weights -> numpy, pre-compute text embeddings
2_noise_algorithm.ipynb -> JAX PGD noise protection, export to .pkl
3_glaze_algorithm.ipynb -> JAX PGD style cloaking, export to .pkl
4_nightshade_algorithm.ipynb -> JAX PGD data poisoning, export to .pkl
5_export_onnx.ipynb    -> Convert all .pkl -> ONNX, consolidate, simplify, validate
```

### Step 1: CLIP Weight Extraction (`1_clip_to_jax.ipynb`)

Extract PyTorch CLIP ViT-B/32 weights to numpy and pre-compute text embeddings:
- Load `openai/clip-vit-base-patch32` from HuggingFace
- Extract vision encoder weights (patch embedding, transformer blocks, LN, projection)
- Pre-compute text embeddings for all algorithm targets:
  - Noise: "chaos destruction" (target) + "a photo" (reference)
  - Glaze: 5 style descriptions (Abstract, Impressionist, Cubist, Sketch, Watercolor)
  - Nightshade: 8 target class descriptions (Dog, Cat, Car, Landscape, Person, Building, Food, Abstract)
- Save as `clip_jax_weights.pkl`

### Step 2: Algorithm Training (Notebooks 2-4)

Each algorithm notebook builds a JAX model that:
1. Takes image input `(1, 224, 224, 3)` in `[0.0, 1.0]` range
2. Applies CLIP preprocessing internally (normalize with mean/std)
3. Runs through the CLIP vision encoder (ViT-B/32)
4. Computes a scalar loss against pre-computed text embeddings
5. Saves as `{algorithm}_state.pkl`

Loss functions:
- **Noise**: `cosine_similarity(image_features, chaos_embedding) - cosine_similarity(image_features, normal_embedding)`
- **Glaze**: `cosine_similarity(image_features, style_embedding[style_index])`
- **Nightshade**: `cosine_similarity(image_features, target_embedding[target_index])`

### Step 3: ONNX Export (`5_export_onnx.ipynb`)

```python
import jax
import jax.numpy as jnp
import jax2onnx
import onnx
from onnxsim import simplify

def export_algorithm(name, apply_fn, params, input_specs):
    model = jax2onnx.to_onnx(
        apply_fn,
        params,
        input_specs=input_specs,
        model_name=name,
    )

    onnx.save(model, f"{name}_raw.onnx")

    model = onnx.load(f"{name}_raw.onnx")
    onnx.save(
        model,
        f"{name}_consolidated.onnx",
        save_as_external_data=False,
    )

    model = onnx.load(f"{name}_consolidated.onnx")
    model_simplified, check = simplify(model)
    assert check, f"Simplification failed for {name}"

    from onnxconverter_common import float16
    # Keep as float32 -- do NOT convert to float16 for Rust ort compatibility

    onnx.save(model_simplified, f"{name}.onnx")
```

### Input Specifications per Algorithm

```python
noise_input_specs = [
    ("input", jnp.float32, (1, 224, 224, 3)),
]

glaze_input_specs = [
    ("input", jnp.float32, (1, 224, 224, 3)),
    ("style_index", jnp.int32, (1,)),
]

nightshade_input_specs = [
    ("input", jnp.float32, (1, 224, 224, 3)),
    ("target_index", jnp.int32, (1,)),
]
```

### Validation Against JAX

After export, validate ONNX output matches JAX within tolerance:

```python
import onnxruntime as ort
import numpy as np

def validate_onnx(onnx_path, jax_fn, jax_params, test_input):
    session = ort.InferenceSession(onnx_path)
    onnx_result = session.run(None, {"input": test_input})[0]

    jax_result = jax_fn(jax_params, test_input)
    jax_result = np.array(jax_result)

    max_diff = np.max(np.abs(onnx_result - jax_result))
    assert max_diff < 1e-4, f"ONNX/JAX mismatch: max diff {max_diff}"
```

### Post-Export Checklist

1. Verify all models are float32 (not float16) -- `ort` crate expects float32
2. Run `onnxsim` simplification on each model
3. Consolidate external data into single file (`save_as_external_data=False`)
4. Validate against JAX reference outputs
5. Each model should be ~350MB (ViT-B/32 CLIP backbone)
6. Place exported `.onnx` files in `src-models/models/`
7. Track with Git LFS (`.gitattributes` already configured)
8. Update `hope_config.json` if input/output specs changed

### Key Python Dependencies

```
jax[cuda12]
jaxlib
jax2onnx
onnx
onnxsim
onnxruntime
transformers
torch
numpy
```

### Config File Reference

`src-models/models/hope_config.json` contains:
- Input specs (size, format, dtype, range)
- Per-algorithm parameters (intensity, iterations, alpha_multiplier)
- Style names (Glaze) and target names (Nightshade)
- Preset configurations (invisible/balanced/strong for Glaze, subtle/balanced/strong for Nightshade)

### Rules

- Models must remain float32 -- the Rust `ort` crate and SPSA optimization require float32
- CLIP normalization is baked into the model -- Rust receives raw `[0.0, 1.0]` pixels
- Input format is NHWC `(1, 224, 224, 3)`, not NCHW
- All models output a single scalar float32 loss value
- Models are >100MB each and must be tracked with Git LFS
- Models are uploaded to GitHub Releases via `publish.yml` and downloaded at runtime
- Always validate ONNX output against JAX before committing a new model

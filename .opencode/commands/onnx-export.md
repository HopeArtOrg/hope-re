---
description: Work with the ONNX model export pipeline in src-models
agent: build
---

Work with the ONNX export pipeline for $ARGUMENTS. Use the `onnx-export` skill for the full pattern.

Key files:

- `src-models/notebooks/5_export_onnx.ipynb` -- JAX to ONNX conversion
- `src-models/notebooks/1_clip_to_jax.ipynb` -- CLIP weight extraction
- `src-models/notebooks/2_noise_algorithm.ipynb` -- Noise algorithm training
- `src-models/notebooks/3_glaze_algorithm.ipynb` -- Glaze algorithm training
- `src-models/notebooks/4_nightshade_algorithm.ipynb` -- Nightshade algorithm training
- `src-models/models/hope_config.json` -- Model configuration and parameters

Models must be float32, NHWC format, with CLIP normalization baked in.
Output goes to `src-models/models/` (tracked by Git LFS).

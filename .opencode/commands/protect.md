---
description: Run the full image protection pipeline with a specific algorithm
agent: build
---

Protect an image using the $ARGUMENTS algorithm. Use the `protection-algorithm` skill for the full pipeline pattern.

Key files:
- `src-tauri/src/onnx_integration/protection/mod.rs` -- protect_image command
- `src-tauri/src/onnx_integration/protection/algorithms.rs` -- algorithm params and model runners
- `src-tauri/src/onnx_integration/protection/spsa.rs` -- SPSA-PGD optimization
- `src-tauri/src/onnx_integration/protection/tiling.rs` -- tile-based processing

Algorithms: `noise`, `glaze`, `nightshade`

Verify with `cargo check` in `src-tauri/` after any changes.

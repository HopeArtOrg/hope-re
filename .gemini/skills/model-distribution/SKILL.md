---
name: model-distribution
description: Handle ONNX model distribution via Git LFS, GitHub Releases, and runtime downloading in Hope:RE
---

## Model Distribution Pattern

When working with ONNX model distribution in Hope:RE, follow these conventions:

### Distribution Flow

```
Developer workflow:
  src-models/notebooks/ (train in Colab)
    -> src-models/models/*.onnx (export)
    -> git lfs track (via .gitattributes)
    -> git push (LFS pointers go to repo, binaries to LFS storage)
    -> publish.yml CI (uploads .onnx files to GitHub Release as assets)

User workflow:
  App launch -> check_models_status command
    -> Models missing? -> Settings page shows download buttons
    -> download_model command -> streams from GitHub Release
    -> Stored in app_data_dir/models/
    -> protect_image resolves models from this directory
```

### Git LFS Configuration

`.gitattributes` tracks ONNX models:
```
src-models/models/*.onnx filter=lfs diff=lfs merge=lfs -text
```

Three models (~350MB each):
- `noise_algorithm.onnx`
- `glaze_algorithm.onnx`
- `nightshade_algorithm.onnx`

### GitHub Actions: Model Upload (`publish.yml`)

The `upload-models` job runs after the `publish-tauri` job:

1. Checkout with `lfs: true` and run `git lfs pull`
2. Read version from `src-tauri/tauri.conf.json`
3. Validate `.onnx` files are real binaries (not LFS pointers)
4. Upload to release: `gh release upload "$TAG" src-models/models/*.onnx --clobber`

Validation check:
```bash
for f in src-models/models/*.onnx; do
    if head -c 40 "$f" | grep -q "version https://git-lfs"; then
        echo "::error::$f is an LFS pointer, not a real binary"
        exit 1
    fi
done
```

### Runtime Model Download

Download URL pattern:
```
https://github.com/HopeArtOrg/hope-re/releases/download/{version}/{model_name}
```

Version comes from `env!("CARGO_PKG_VERSION")` (compile-time from `Cargo.toml`).

### Tauri Commands

#### Check Model Status

```rust
#[tauri::command]
pub async fn check_models_status(app: AppHandle) -> Result<ModelsCheckResult, String>
```

Returns per-model status (exists, size_bytes) and `all_ready` flag. A model "exists" only if the file is present and larger than 1024 bytes (to reject empty/corrupt files).

#### Download Model

```rust
#[tauri::command]
pub async fn download_model(app: AppHandle, model_name: String) -> Result<String, String>
```

Streaming download with:
- `reqwest` client with 10-redirect limit
- Temp file (`.onnx.tmp`) renamed to final path on completion
- Progress events emitted every 0.5% via `model-download-progress`
- Skip if already downloaded (file exists and >1024 bytes)
- Validate model name against known list

### Progress Events

```rust
#[derive(Debug, Clone, Serialize)]
struct DownloadProgress {
    model_name: String,
    downloaded_bytes: u64,
    total_bytes: u64,
    progress_percent: f64,
}
```

Event name: `"model-download-progress"`, emitted via `let _ = app.emit(...)`.

### Model Resolution Priority

When `protect_image` needs a model, resolution order is:

1. **Downloaded models**: `app_data_dir/models/{model_name}` (from runtime download)
2. **Resource directory**: `resource_dir/{model_name}` or `resource_dir/models/{model_name}`
3. **Dev mode only**: `CARGO_MANIFEST_DIR/../src-models/models/{model_name}`

### LFS Pointer Detection

Before loading any model, check if the file is a Git LFS pointer:

```rust
fn is_git_lfs_pointer(bytes: &[u8]) -> bool {
    bytes.starts_with(b"version https://git-lfs.github.com/spec/")
}
```

If detected, return a descriptive error telling the user to run `git lfs pull`.

### Storage Location

Models are stored in the platform-specific app data directory:

| Platform | Path |
|----------|------|
| Windows | `C:\Users\{user}\AppData\Roaming\{app}\models\` |
| macOS | `~/Library/Application Support/{app}/models/` |
| Linux | `~/.local/share/{app}/models/` |

### Key Dependencies

```toml
reqwest = { version = "0.12", features = ["stream"] }
futures-util = "0.3"
tokio = { version = "1", features = ["fs", "io-util"] }
```

### Rules

- Always validate model names against the known `MODELS` array before downloading
- Use temp files with atomic rename to prevent corrupt partial downloads
- Emit progress events at most every 0.5% to avoid flooding the frontend
- Use `let _ = app.emit(...)` for non-critical progress emissions
- Check file size > 1024 bytes to validate model presence (rejects corrupt/empty files)
- Detect Git LFS pointers early and provide actionable error messages
- Version-lock downloads to `CARGO_PKG_VERSION` for release consistency
- No comments in code -- self-documenting through clear naming
- No emojis anywhere in the codebase

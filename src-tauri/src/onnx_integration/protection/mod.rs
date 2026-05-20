mod algorithms;
mod encoding;
mod model;
mod preprocessing;
mod spsa;
mod tiling;
mod types;

use ndarray::Array4;
use ort::session::Session;
use std::sync::atomic::Ordering;

use algorithms::{
    get_glaze_params, get_glaze_style_index, get_nightshade_params, get_nightshade_target_index,
    get_noise_params, run_glaze_model, run_nightshade_model, run_noise_model,
};
use encoding::{apply_fallback_noise, encode_image_to_base64};
use model::{load_model, resolve_model_path};
use tiling::apply_model_protection;
pub use types::{ProtectionResult, ProtectionSettings, ProtectionState};
use types::{AlgorithmParams, ProtectionProgress};

use tauri::Emitter;

#[tauri::command]
pub fn cancel_protection(state: tauri::State<'_, ProtectionState>) {
    state.is_cancelled.store(true, Ordering::SeqCst);
    log::info!("Protection cancellation requested");
}

#[tauri::command]
pub async fn protect_image(
    app: tauri::AppHandle,
    image_base64: String,
    settings: ProtectionSettings,
    state: tauri::State<'_, ProtectionState>,
) -> Result<ProtectionResult, String> {
    state.is_cancelled.store(false, Ordering::SeqCst);

    let protected = decode_image(&image_base64)
        .and_then(|img| {
            emit_progress(&app, "loading", 2.0);
            execute_protection(&app, &state, &img, &settings)
        })
        .and_then(|(img, message, model_used)| {
            if state.is_cancelled.load(Ordering::SeqCst) {
                return Err("Protection cancelled".to_string());
            }
            emit_progress(&app, "encoding", 96.0);
            let quality = settings.output_quality.clamp(1, 100);
            let base64 = encode_image_to_base64(&img, quality)?;
            emit_progress(&app, "complete", 100.0);
            Ok(ProtectionResult {
                image_base64: base64,
                success: true,
                message,
                model_used,
            })
        })?;

    Ok(protected)
}

fn decode_image(base64_str: &str) -> Result<image::DynamicImage, String> {
    let cleaned = base64_str
        .trim_start_matches("data:image/png;base64,")
        .trim_start_matches("data:image/jpeg;base64,")
        .trim_start_matches("data:image/jpg;base64,");

    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, cleaned)
        .map_err(|e| format!("Failed to decode base64: {}", e))
        .and_then(|data| {
            image::load_from_memory(&data).map_err(|e| format!("Failed to load image: {}", e))
        })
}

fn emit_progress(app: &tauri::AppHandle, stage: &str, percent: f64) {
    let _ = app.emit(
        "protection-progress",
        ProtectionProgress {
            stage: stage.to_string(),
            tile_current: 0,
            tile_total: 0,
            iteration_current: 0,
            iteration_total: 0,
            percent,
        },
    );
}

fn execute_protection(
    app: &tauri::AppHandle,
    state: &ProtectionState,
    img: &image::DynamicImage,
    settings: &ProtectionSettings,
) -> Result<(image::DynamicImage, String, bool), String> {
    let render_factor = settings.render_quality.clamp(0, 100) as f32 / 100.0;

    match settings.algorithm.as_str() {
        "noise" => {
            let params = get_noise_params(settings.intensity);
            let iterations = (params.max_iterations as f32 * render_factor).max(1.0) as u32;
            run_with_model(
                app,
                state,
                img,
                "noise_algorithm.onnx",
                params,
                iterations,
                |s, i| run_noise_model(s, i),
                |img, i, iter| apply_fallback_noise(img, i, get_now_seed(), iter),
                "Noise",
                settings.intensity,
            )
        }
        "glaze" => {
            let style = settings.glaze_style.as_deref().unwrap_or("abstract");
            let style_index = get_glaze_style_index(style);
            let params = get_glaze_params(settings.intensity);
            let iterations = (params.max_iterations as f32 * render_factor).max(1.0) as u32;
            run_with_model(
                app,
                state,
                img,
                "glaze_algorithm.onnx",
                params,
                iterations,
                |s, i| run_glaze_model(s, i, style_index),
                |img, i, iter| apply_fallback_noise(img, i * 0.8, style_index as u32, iter),
                &format!("Glaze ({})", style),
                settings.intensity,
            )
        }
        "nightshade" => {
            let target = settings.nightshade_target.as_deref().unwrap_or("dog");
            let target_index = get_nightshade_target_index(target);
            let params = get_nightshade_params(settings.intensity);
            let iterations = (params.max_iterations as f32 * render_factor).max(1.0) as u32;
            run_with_model(
                app,
                state,
                img,
                "nightshade_algorithm.onnx",
                params,
                iterations,
                |s, i| run_nightshade_model(s, i, target_index),
                |img, i, iter| apply_fallback_noise(img, i * 1.2, target_index as u32 + 100, iter),
                &format!("Nightshade ({})", target),
                settings.intensity,
            )
        }
        _ => Err(format!("Unknown algorithm: {}", settings.algorithm)),
    }
}

fn run_with_model<F, G>(
    app: &tauri::AppHandle,
    state: &ProtectionState,
    img: &image::DynamicImage,
    model_name: &str,
    params: AlgorithmParams,
    iterations: u32,
    mut run_fn: F,
    fallback_fn: G,
    label: &str,
    intensity: f32,
) -> Result<(image::DynamicImage, String, bool), String>
where
    F: FnMut(&mut Session, &Array4<f32>) -> Result<f32, String> + 'static,
    G: FnOnce(&image::DynamicImage, f32, u32) -> image::DynamicImage,
{
    let model_result = resolve_model_path(app, model_name).and_then(|path| load_model(&path));

    match model_result {
        Ok(mut session) => {
            log::info!("{} ONNX model loaded", label);
            let protected = apply_model_protection(
                img,
                &mut session,
                &params,
                iterations,
                &mut run_fn,
                app,
                state,
            )?;
            Ok((
                protected,
                format!("{} protection applied with ONNX model", label),
                true,
            ))
        }
        Err(e) => {
            log::error!("{} model failed to load: {}", label, e);
            if state.is_cancelled.load(Ordering::SeqCst) {
                return Err("Protection cancelled".to_string());
            }
            let protected = fallback_fn(img, intensity, iterations);
            Ok((
                protected,
                format!("{} protection applied with fallback ({})", label, e),
                false,
            ))
        }
    }
}

fn get_now_seed() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
        .as_secs() as u32
}

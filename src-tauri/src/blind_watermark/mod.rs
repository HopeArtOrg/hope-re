use blind_watermark::prelude::{embed_watermark_string, extract_watermark_string, get_wm_len};
use std::env;
use std::fs;
use std::path::PathBuf;

#[tauri::command]
pub async fn embed_watermark(
    image_base64: String,
    watermark: String,
    seed: Option<u64>,
) -> Result<String, String> {
    let temp_input = decode_base64_to_temp_png(&image_base64)?;

    let mut temp_output = env::temp_dir();
    temp_output.push(format!("hope_watermark_out_{}.png", uuid::Uuid::new_v4()));

    let res = embed_watermark_string(
        temp_input.to_str().ok_or("invalid input path")?,
        temp_output.to_str().ok_or("invalid output path")?,
        &watermark,
        seed,
    );

    let _ = fs::remove_file(&temp_input);

    res.map_err(|e| {
        let _ = fs::remove_file(&temp_output);
        format!("failed to embed watermark: {}", e)
    })?;

    let output_bytes = fs::read(&temp_output).map_err(|e| {
        let _ = fs::remove_file(&temp_output);
        format!("failed to read output file: {}", e)
    })?;

    let _ = fs::remove_file(&temp_output);

    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, output_bytes);

    Ok(format!("data:image/png;base64,{}", encoded))
}

#[tauri::command]
pub async fn extract_watermark(
    image_base64: String,
    watermark_len: usize,
    seed: Option<u64>,
) -> Result<String, String> {
    let temp_input = decode_base64_to_temp_png(&image_base64)?;

    let dummy_string = "A".repeat(watermark_len);
    let bit_len = get_wm_len(dummy_string.as_bytes());

    let res = extract_watermark_string(
        temp_input.to_str().ok_or("invalid input path")?,
        bit_len,
        seed,
    );

    let _ = fs::remove_file(&temp_input);

    res.map_err(|e| format!("failed to extract watermark: {}", e))
}

fn decode_base64_to_temp_png(base64_str: &str) -> Result<PathBuf, String> {
    let cleaned = base64_str
        .trim_start_matches("data:image/png;base64,")
        .trim_start_matches("data:image/jpeg;base64,")
        .trim_start_matches("data:image/jpg;base64,")
        .trim_start_matches("data:image/webp;base64,");

    let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, cleaned)
        .map_err(|e| format!("failed to decode base64: {}", e))?;

    let img = image::load_from_memory(&bytes)
        .map_err(|e| format!("failed to decode image bytes: {}", e))?;

    let mut temp_path = env::temp_dir();
    temp_path.push(format!("hope_watermark_in_{}.png", uuid::Uuid::new_v4()));

    img.save_with_format(&temp_path, image::ImageFormat::Png)
        .map_err(|e| format!("failed to save temp PNG file: {}", e))?;

    Ok(temp_path)
}

use crate::onnx_integration::capabilities::build_execution_providers;

#[tauri::command]
pub fn create_ort_session(model_path: String) -> Result<String, String> {
    use ort::session::Session;

    let eps = build_execution_providers();

    if eps.is_empty() {
        return Err("No execution providers available".to_string());
    }

    use std::fs;

    let model_bytes = fs::read(model_path).map_err(|e| format!("Failed to read model: {}", e))?;
    let builder = Session::builder().map_err(|e| format!("Failed to create session builder: {}", e))?;
    let mut builder = builder
        .with_execution_providers(eps)
        .map_err(|e| format!("Failed to set execution providers: {}", e))?;
    builder
        .commit_from_memory(&model_bytes)
        .map_err(|e| format!("Failed to create session: {}", e))?;

    Ok("Session created successfully".to_string())
}

mod commands;
#[cfg(not(all(target_os = "android", not(target_arch = "aarch64"))))]
mod onnx_integration;
#[cfg(all(target_os = "android", not(target_arch = "aarch64")))]
mod onnx_stubs;
mod system_info;

use commands::{
    cancel_protection, check_models_status, create_ort_session, download_model,
    get_inference_capabilities, get_system_info, protect_image,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_inference_capabilities,
            create_ort_session,
            protect_image,
            #[cfg(not(all(target_os = "android", not(target_arch = "aarch64"))))]
            cancel_protection,
            check_models_status,
            download_model
        ])
        .setup(|app| {
            use tauri::Manager;
            #[cfg(not(all(target_os = "android", not(target_arch = "aarch64"))))]
            app.manage(onnx_integration::protection::ProtectionState::default());

            #[cfg(target_os = "windows")]
            {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_decorations(false);
                }
            }

            let log_level = if cfg!(debug_assertions) {
                log::LevelFilter::Info
            } else {
                log::LevelFilter::Warn
            };
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .level(log_level)
                    .build(),
            )?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

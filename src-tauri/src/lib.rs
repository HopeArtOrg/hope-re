mod blind_watermark;
mod commands;
#[cfg(not(all(target_os = "android", not(target_arch = "aarch64"))))]
mod onnx_integration;
#[cfg(all(target_os = "android", not(target_arch = "aarch64")))]
mod onnx_stubs;
mod system_info;

use commands::{
    cancel_protection, check_models_status, download_model, embed_watermark, extract_watermark,
    get_inference_capabilities, get_system_info, protect_image,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init());

    #[cfg(desktop)]
    let builder = builder
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init());

    builder
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_inference_capabilities,
            protect_image,
            embed_watermark,
            extract_watermark,
            cancel_protection,
            check_models_status,
            download_model
        ])
        .setup(|app| {
            #[cfg(not(all(target_os = "android", not(target_arch = "aarch64"))))]
            {
                use tauri::Manager;
                app.manage(onnx_integration::protection::ProtectionState::default());
            }

            #[cfg(target_os = "windows")]
            {
                use tauri::Manager;
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

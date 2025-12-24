// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod context; // M4: Context store
mod core;
mod parser;
mod renderer; // M3: Rendering engine
mod rules; // M4: Rules processor
mod storage; // Package library storage
mod validator; // M6: Package validator

use commands::*;
use tauri::{Emitter, EventTarget};
use tauri_plugin_deep_link::DeepLinkExt;

fn main() {
    tauri::Builder::default()
        .manage(commands::oauth::OAuthCallbackState::default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            let handle = app.handle().clone();

            // Initialize package library
            if let Err(e) = tauri::async_runtime::block_on(init_library(handle.clone())) {
                eprintln!("[library] Failed to initialize package library: {}", e);
            }

            #[cfg(any(windows, target_os = "linux"))]
            {
                if let Err(e) = app.deep_link().register("promptgen") {
                    eprintln!("[deep-link] register(promptgen) failed: {e}");
                }
            }

            app.deep_link().on_open_url(move |event| {
                for url in event.urls() {
                    eprintln!("[deep-link] received url: {url}");
                    // Emit to all windows / listeners.
                    let _ = handle.emit_to(EventTarget::any(), "oauth-callback", url.to_string());
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_package,
            load_package_with_dependencies, // M9 Phase 2.5: Load with dependencies
            save_package,                   // M7: Save package
            create_package,                 // M7: Create new package
            validate_package,
            validate_package_with_dependencies, // M9 Phase 3: Validate with dependencies
            get_package_info,
            render_prompt, // M3: Rendering command (legacy, without dependencies)
            render_prompt_with_dependencies, // M9 Phase 3: Render with dependencies
            render_from_rulebook, // M9: Rulebook rendering (legacy)
            render_from_rulebook_with_dependencies, // M9 Phase 3: Rulebook with dependencies
            render_from_rulebook_batch, // M9: Batch rulebook rendering (legacy)
            render_from_rulebook_batch_with_dependencies, // M9 Phase 3: Batch with dependencies
            oauth_start_loopback,
            oauth_cancel_loopback,
            oauth_exchange_code,
            oauth_probe,
            http_request,
            debug_info,
            // Library commands
            init_library,
            list_library_packages,
            install_package_to_library,
            remove_package_from_library,
            load_package_from_library,
            load_all_library_packages,
            get_library_path,
            refresh_library,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

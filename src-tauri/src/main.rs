// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod parser;
mod commands;
mod renderer; // M3: Rendering engine
mod context;  // M4: Context store
mod rules;    // M4: Rules processor
mod validator; // M6: Package validator

use commands::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            load_package,
            load_package_with_dependencies, // M9 Phase 2.5: Load with dependencies
            save_package, // M7: Save package
            create_package, // M7: Create new package
            validate_package,
            validate_package_with_dependencies, // M9 Phase 3: Validate with dependencies
            get_package_info,
            render_prompt, // M3: Rendering command (legacy, without dependencies)
            render_prompt_with_dependencies, // M9 Phase 3: Render with dependencies
            render_from_rulebook, // M9: Rulebook rendering (legacy)
            render_from_rulebook_with_dependencies, // M9 Phase 3: Rulebook with dependencies
            render_from_rulebook_batch, // M9: Batch rulebook rendering (legacy)
            render_from_rulebook_batch_with_dependencies, // M9 Phase 3: Batch with dependencies
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


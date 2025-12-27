// Library Commands - Package library management

use crate::core::Package;
use crate::storage::{LibraryEntry, PackageLibrary, PackageSource};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

/// Global library state
pub struct LibraryState(pub Mutex<Option<PackageLibrary>>);

/// Initialize library (called on app startup)
#[tauri::command]
pub async fn init_library(app: AppHandle) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    let library = PackageLibrary::load(&app_data_dir)?;

    // Store in app state
    app.manage(LibraryState(Mutex::new(Some(library))));

    Ok(())
}

/// List all packages in library
#[tauri::command]
pub async fn list_library_packages(app: AppHandle) -> Result<Vec<LibraryEntry>, String> {
    let state = app.state::<LibraryState>();
    let library_lock = state.0.lock().unwrap();
    let library = library_lock.as_ref().ok_or("Library not initialized")?;

    Ok(library.list_packages().to_vec())
}

/// Install a package to the library
#[tauri::command]
pub async fn install_package_to_library(
    app: AppHandle,
    package: Package,
    yaml_content: String,
    source: String,
) -> Result<LibraryEntry, String> {
    let source = match source.as_str() {
        "marketplace" => PackageSource::Marketplace,
        "local" => PackageSource::Local,
        "imported" => PackageSource::Imported,
        _ => return Err(format!("Invalid source: {}", source)),
    };

    let state = app.state::<LibraryState>();
    let mut library_lock = state.0.lock().unwrap();
    let library = library_lock.as_mut().ok_or("Library not initialized")?;

    library.install_package(&package, &yaml_content, source)
}

/// Remove a package from library
#[tauri::command]
pub async fn remove_package_from_library(
    app: AppHandle,
    package_id: String,
    version: String,
) -> Result<(), String> {
    let state = app.state::<LibraryState>();
    let mut library_lock = state.0.lock().unwrap();
    let library = library_lock.as_mut().ok_or("Library not initialized")?;

    library.remove_package(&package_id, &version)
}

/// Load a package from library
#[tauri::command]
pub async fn load_package_from_library(
    app: AppHandle,
    package_id: String,
    version: String,
) -> Result<Package, String> {
    let state = app.state::<LibraryState>();
    let mut library_lock = state.0.lock().unwrap();
    let library = library_lock.as_mut().ok_or("Library not initialized")?;

    library.load_package(&package_id, &version)
}

/// Load all packages from library
#[tauri::command]
pub async fn load_all_library_packages(app: AppHandle) -> Result<HashMap<String, Package>, String> {
    let state = app.state::<LibraryState>();
    let library_lock = state.0.lock().unwrap();
    let library = library_lock.as_ref().ok_or("Library not initialized")?;

    library.load_all_packages()
}

/// Get library path
#[tauri::command]
pub async fn get_library_path(app: AppHandle) -> Result<String, String> {
    let state = app.state::<LibraryState>();
    let library_lock = state.0.lock().unwrap();
    let library = library_lock.as_ref().ok_or("Library not initialized")?;

    Ok(library.library_path().display().to_string())
}

/// Refresh library (re-scan filesystem)
#[tauri::command]
pub async fn refresh_library(app: AppHandle) -> Result<Vec<LibraryEntry>, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    let library = PackageLibrary::load(&app_data_dir)?;

    // Update state
    let state = app.state::<LibraryState>();
    let mut library_lock = state.0.lock().unwrap();
    *library_lock = Some(library.clone());

    Ok(library.list_packages().to_vec())
}

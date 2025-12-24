use tauri::AppHandle;
use tauri::Manager;

/// Return a helpful debug payload (paths) so release builds can be debugged
/// even when DevTools are not available.
#[tauri::command]
pub fn debug_info(app: AppHandle) -> Result<String, String> {
    let paths = app.path();

    let app_data_dir = paths
        .app_data_dir()
        .map_err(|e| format!("failed to resolve app_data_dir: {e}"))?;

    let local_data_dir = paths
        .local_data_dir()
        .map_err(|e| format!("failed to resolve local_data_dir: {e}"))?;

    let log_dir = paths
        .app_log_dir()
        .map_err(|e| format!("failed to resolve app_log_dir: {e}"))?;

    Ok(format!(
        "app_data_dir={}\nlocal_data_dir={}\napp_log_dir={}",
        app_data_dir.to_string_lossy(),
        local_data_dir.to_string_lossy(),
        log_dir.to_string_lossy()
    ))
}

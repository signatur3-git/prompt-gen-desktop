// M3: Tauri Render Commands
// M9 Task 1.5: Added rulebook rendering commands
// M9 Phase 3: Added dependencies support for cross-package rendering
// Bridge between Vue frontend and Rust rendering engine

use crate::core::Package;
use crate::renderer::Renderer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Render a promptsection from a package (without dependencies - legacy)
#[tauri::command]
pub async fn render_prompt(
    package: Package,
    promptsection: String,
    seed: u64,
) -> Result<RenderResult, String> {
    let renderer = Renderer::new(&package, seed);
    renderer.render(&promptsection)
        .map_err(|e| format!("Render error: {}", e))
}

/// M9 Phase 3: Render a promptsection with dependencies
#[tauri::command]
pub async fn render_prompt_with_dependencies(
    package: Package,
    dependencies: HashMap<String, Package>,
    promptsection: String,
    seed: u64,
) -> Result<RenderResult, String> {
    let renderer = Renderer::new_with_dependencies(&package, &dependencies, seed);
    renderer.render(&promptsection)
        .map_err(|e| format!("Render error: {}", e))
}

/// M9: Render from a rulebook (without dependencies - legacy)
#[tauri::command]
pub async fn render_from_rulebook(
    package: Package,
    rulebook_ref: String,
    seed: u64,
) -> Result<RenderResult, String> {
    let renderer = Renderer::new(&package, seed);
    renderer.render_from_rulebook(&rulebook_ref)
        .map_err(|e| format!("Render error: {}", e))
}

/// M9 Phase 3: Render from a rulebook with dependencies
#[tauri::command]
pub async fn render_from_rulebook_with_dependencies(
    package: Package,
    dependencies: HashMap<String, Package>,
    rulebook_ref: String,
    seed: u64,
) -> Result<RenderResult, String> {
    let renderer = Renderer::new_with_dependencies(&package, &dependencies, seed);
    renderer.render_from_rulebook(&rulebook_ref)
        .map_err(|e| format!("Render error: {}", e))
}

/// M9: Render multiple prompts from a rulebook with batch variety (without dependencies)
#[tauri::command]
pub async fn render_from_rulebook_batch(
    package: Package,
    rulebook_ref: String,
    count: usize,
    start_seed: u64,
) -> Result<BatchRenderResponse, String> {
    let mut results = Vec::new();
    let mut used_entry_points = Vec::new();

    for i in 0..count {
        let seed = start_seed.wrapping_add(i as u64);
        let renderer = Renderer::new(&package, seed);

        match renderer.render_from_rulebook_with_options(&rulebook_ref, Some(&mut used_entry_points)) {
            Ok(result) => {
                results.push(BatchRenderResult {
                    output: result.output,
                    seed: result.seed,
                    index: i,
                });
            }
            Err(e) => {
                return Err(format!("Render error at index {}: {}", i, e));
            }
        }
    }

    Ok(BatchRenderResponse { results })
}

/// M9 Phase 3: Render multiple prompts from a rulebook with batch variety and dependencies
#[tauri::command]
pub async fn render_from_rulebook_batch_with_dependencies(
    package: Package,
    dependencies: HashMap<String, Package>,
    rulebook_ref: String,
    count: usize,
    start_seed: u64,
) -> Result<BatchRenderResponse, String> {
    let mut results = Vec::new();
    let mut used_entry_points = Vec::new();

    for i in 0..count {
        let seed = start_seed.wrapping_add(i as u64);
        let renderer = Renderer::new_with_dependencies(&package, &dependencies, seed);

        match renderer.render_from_rulebook_with_options(&rulebook_ref, Some(&mut used_entry_points)) {
            Ok(result) => {
                results.push(BatchRenderResult {
                    output: result.output,
                    seed: result.seed,
                    index: i,
                });
            }
            Err(e) => {
                return Err(format!("Render error at index {}: {}", i, e));
            }
        }
    }

    Ok(BatchRenderResponse { results })
}

/// Response wrapper for batch render
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRenderResponse {
    pub results: Vec<BatchRenderResult>,
}

/// Result for a single item in a batch render
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRenderResult {
    pub output: String,
    pub seed: u64,
    pub index: usize,
}

/// Simple type alias for the render result (for serialization)
pub use crate::renderer::RenderResult;

#[cfg(test)]
mod tests {
    // Tests will use the renderer tests directly
}


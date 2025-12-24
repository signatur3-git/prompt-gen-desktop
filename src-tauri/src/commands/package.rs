// M2: Tauri Commands - Package operations
// Bridge between Vue frontend and Rust backend

use crate::core::Package;
use crate::parser::{self, DependencyResolver};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Load a package from a file path
#[tauri::command]
pub async fn load_package(path: String) -> Result<Package, String> {
    parser::load_package(&path).map_err(|e| format!("Failed to load package: {}", e))
}

/// M9 Phase 2.5: Load a package with all its dependencies resolved
#[tauri::command]
pub async fn load_package_with_dependencies(
    path: String,
    search_paths: Option<Vec<String>>,
) -> Result<PackageWithDependencies, String> {
    // Convert search paths to PathBuf
    let search_paths = search_paths
        .unwrap_or_else(|| vec!["./packages".to_string(), "./test-packages".to_string()])
        .into_iter()
        .map(PathBuf::from)
        .collect();

    // Create resolver
    let mut resolver = DependencyResolver::new(search_paths);

    // Load package with dependencies
    let (package, deps) = resolver
        .load_package_with_deps(Path::new(&path))
        .map_err(|e| e.to_string())?;

    Ok(PackageWithDependencies {
        package,
        dependencies: deps,
    })
}

#[derive(serde::Serialize)]
pub struct PackageWithDependencies {
    pub package: Package,
    pub dependencies: HashMap<String, Package>,
}

/// Save a package to a file path (M7)
#[tauri::command]
pub async fn save_package(package: Package, path: String) -> Result<(), String> {
    // Serialize package to YAML
    let yaml = serde_yaml::to_string(&package)
        .map_err(|e| format!("Failed to serialize package: {}", e))?;

    // Write to file
    fs::write(&path, yaml).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

/// Create a new empty package (M7)
#[tauri::command]
pub async fn create_package(
    id: String,
    version: String,
    name: String,
    description: Option<String>,
    authors: Vec<String>,
    namespace_id: String,
) -> Result<Package, String> {
    use crate::core::{Namespace, PackageMetadata};
    use std::collections::HashMap;

    let mut namespaces = HashMap::new();
    namespaces.insert(
        namespace_id.clone(),
        Namespace {
            id: namespace_id,
            datatypes: HashMap::new(),
            prompt_sections: HashMap::new(),
            separator_sets: HashMap::new(),
            rules: HashMap::new(),
            decisions: Vec::new(),     // M7: Initialize empty decisions
            rulebooks: HashMap::new(), // M9: Initialize empty rulebooks
        },
    );

    Ok(Package {
        id,
        version,
        metadata: PackageMetadata {
            name,
            description,
            authors,
            bypass_filters: false,
        },
        namespaces,
        dependencies: Vec::new(),
    })
}

/// Get package information (for display)
#[tauri::command]
pub async fn get_package_info(package: Package) -> Result<PackageInfo, String> {
    Ok(PackageInfo {
        id: package.id.clone(),
        version: package.version.clone(),
        name: package.metadata.name.clone(),
        description: package.metadata.description.clone(),
        namespace_count: package.namespaces.len(),
        datatype_count: package
            .namespaces
            .values()
            .map(|ns| ns.datatypes.len())
            .sum(),
        promptsection_count: package
            .namespaces
            .values()
            .map(|ns| ns.prompt_sections.len())
            .sum(),
    })
}

#[derive(serde::Serialize)]
pub struct PackageInfo {
    pub id: String,
    pub version: String,
    pub name: String,
    pub description: Option<String>,
    pub namespace_count: usize,
    pub datatype_count: usize,
    pub promptsection_count: usize,
}

// M2: Package Parser - Load packages from YAML/JSON
// Implements serde-based deserialization with validation
// M8.5 Blocker 2 Phase 2: Load packages with dependencies

use crate::core::Package;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to read file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse YAML: {0}")]
    YamlParse(#[from] serde_yaml::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Invalid package format: {0}")]
    InvalidFormat(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[allow(dead_code)] // Used in error paths, may not be constructed in all cases
    #[error("Dependency not found: {0}")]
    DependencyNotFound(String),
}

pub type Result<T> = std::result::Result<T, ParserError>;

/// Package with loaded dependencies
/// M8.5 Blocker 2 Phase 2: Support cross-package rule execution
#[allow(dead_code)] // Part of API, used when loading packages with dependencies
#[derive(Debug, Clone)]
pub struct LoadedPackage {
    pub package: Package,
    pub dependencies: HashMap<String, Package>,
}

/// Load a package with all its dependencies
/// M9 Phase 2.7: Updated to use DependencyResolver with path-based and search-based resolution
///
/// Dependencies are resolved from:
/// 1. Explicit paths in dependency declarations
/// 2. `dependencies/` subdirectory relative to package file
/// 3. Package directory itself
/// 4. `./test-packages` directory
/// 5. `./packages` directory
///
/// Example:
/// ```yaml
/// dependencies:
///   - package: test.base
///     version: "1.0.0"
///     path: "./test-base.yaml"  # Optional explicit path
/// ```
#[allow(dead_code)] // Part of API, used for loading packages with dependencies
pub fn load_package_with_dependencies<P: AsRef<Path>>(path: P) -> Result<LoadedPackage> {
    use crate::parser::DependencyResolver;

    let path = path.as_ref();

    // Set up search paths
    let package_dir = path.parent().unwrap_or(Path::new("."));
    let search_paths = vec![
        package_dir.join("dependencies"), // dependencies/ subdirectory
        package_dir.to_path_buf(),        // package directory
        PathBuf::from("./test-packages"), // test-packages directory
        PathBuf::from("./packages"),      // packages directory
    ];

    // Use DependencyResolver
    let mut resolver = DependencyResolver::new(search_paths);

    let (package, dependencies) = resolver
        .load_package_with_deps(path)
        .map_err(|e| ParserError::Validation(e.to_string()))?;

    Ok(LoadedPackage {
        package,
        dependencies,
    })
}

/// Load a package from a YAML or JSON file
pub fn load_package<P: AsRef<Path>>(path: P) -> Result<Package> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)?;

    // Determine format from extension
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| ParserError::InvalidFormat("No file extension".to_string()))?;

    match extension.to_lowercase().as_str() {
        "yaml" | "yml" => parse_yaml(&content),
        "json" => parse_json(&content),
        _ => Err(ParserError::InvalidFormat(format!(
            "Unsupported file extension: {}",
            extension
        ))),
    }
}

/// Parse YAML content into a Package
pub fn parse_yaml(content: &str) -> Result<Package> {
    let mut package: Package = serde_yaml::from_str(content)?;
    validate_package(&package)?;
    normalize_references(&mut package);
    Ok(package)
}

/// Parse JSON content into a Package
pub fn parse_json(content: &str) -> Result<Package> {
    let mut package: Package = serde_json::from_str(content)?;
    validate_package(&package)?;
    normalize_references(&mut package);
    Ok(package)
}

/// Normalize all relative references to absolute references
///
/// M9 Phase 3: Cross-package fix
/// Converts relative references (e.g., "colors") to absolute (e.g., "provider:colors")
/// based on the namespace they're defined in. This ensures correct resolution when
/// components are referenced cross-package.
///
/// Example:
/// ```yaml
/// # In namespace "provider", author writes:
/// color:
///   target: colors  # Relative
///
/// # After normalization:
/// color:
///   target: provider:colors  # Absolute
/// ```
fn normalize_references(package: &mut Package) {
    for (namespace_id, namespace) in &mut package.namespaces {
        // Normalize references in all promptsections
        for promptsection in namespace.prompt_sections.values_mut() {
            for reference in promptsection.references.values_mut() {
                // Skip empty, context references, and already-absolute references
                if !reference.target.is_empty()
                    && !reference.target.starts_with("context:")
                    && !reference.target.contains(':')
                {
                    // Make relative reference absolute by prepending namespace
                    reference.target = format!("{}:{}", namespace_id, reference.target);
                }
            }
        }
    }
}

/// Basic package validation
/// More comprehensive validation will be added in M6
fn validate_package(package: &Package) -> Result<()> {
    // Validate package has at least one namespace
    if package.namespaces.is_empty() {
        return Err(ParserError::Validation(
            "Package must contain at least one namespace".to_string(),
        ));
    }

    // Validate package ID is not empty
    if package.id.is_empty() {
        return Err(ParserError::Validation(
            "Package ID cannot be empty".to_string(),
        ));
    }

    // Validate version format (basic semver check)
    if !is_valid_version(&package.version) {
        return Err(ParserError::Validation(format!(
            "Invalid version format: {}",
            package.version
        )));
    }

    // TODO M6: Add comprehensive validation
    // - Reference resolution
    // - Circular dependency detection
    // - Tag filter syntax validation
    // - Rule/Decision validation

    Ok(())
}

/// Basic semver validation
fn is_valid_version(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    parts.iter().all(|part| part.parse::<u32>().is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_validation() {
        assert!(is_valid_version("1.0.0"));
        assert!(is_valid_version("0.1.0"));
        assert!(is_valid_version("10.20.30"));
        assert!(!is_valid_version("1.0"));
        assert!(!is_valid_version("1.0.0.0"));
        assert!(!is_valid_version("invalid"));
    }

    #[test]
    fn test_parse_minimal_yaml() {
        let yaml = r#"
id: test.package
version: 1.0.0
metadata:
  name: Test Package
  authors: []
namespaces:
  test:
    id: test
"#;
        let result = parse_yaml(yaml);
        assert!(result.is_ok());
        let package = result.unwrap();
        assert_eq!(package.id, "test.package");
        assert_eq!(package.version, "1.0.0");
    }
}

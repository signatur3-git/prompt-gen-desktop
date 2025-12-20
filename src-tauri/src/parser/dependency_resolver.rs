// M9 Phase 2.2: Dependency Resolution
//
// Handles loading packages with their dependencies, with cycle detection and caching.

use crate::core::{Dependency, Package, validate_exact_match};
use crate::parser::package_loader;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Resolves and loads package dependencies
pub struct DependencyResolver {
    /// Cache of loaded packages (key: package_id)
    cache: HashMap<String, Package>,

    /// Paths to search for packages
    search_paths: Vec<PathBuf>,

    /// Track packages being loaded to detect circular dependencies
    loading: HashSet<String>,
}

impl DependencyResolver {
    /// Create a new resolver with search paths
    pub fn new(search_paths: Vec<PathBuf>) -> Self {
        Self {
            cache: HashMap::new(),
            search_paths,
            loading: HashSet::new(),
        }
    }

    /// Load a package and all its dependencies
    /// Returns (main_package, dependencies_map)
    pub fn load_package_with_deps(
        &mut self,
        path: &Path,
    ) -> Result<(Package, HashMap<String, Package>), DependencyError> {
        // Load main package
        let package = package_loader::load_package(path)
            .map_err(|e| DependencyError::LoadError {
                package_id: path.display().to_string(),
                path: path.to_path_buf(),
                reason: e.to_string(),
            })?;

        // Resolve all dependencies
        let deps = self.resolve_dependencies(&package, path.parent())?;

        Ok((package, deps))
    }

    /// Resolve dependencies for a package
    fn resolve_dependencies(
        &mut self,
        package: &Package,
        base_path: Option<&Path>,
    ) -> Result<HashMap<String, Package>, DependencyError> {
        let mut resolved = HashMap::new();

        // Check for circular dependency
        if self.loading.contains(&package.id) {
            return Err(DependencyError::CircularDependency {
                cycle: self.format_cycle(&package.id),
            });
        }

        // Mark as loading
        self.loading.insert(package.id.clone());

        // Process each dependency
        for dep in &package.dependencies {
            let dep_package = self.load_dependency(dep, base_path)?;

            // Validate version match (exact only!)
            validate_exact_match(&dep_package.version, &dep.version)
                .map_err(|e| DependencyError::VersionMismatch(Box::new(VersionMismatchData {
                    package_id: dep.package_id.clone(),
                    required: dep.version.clone(),
                    found: dep_package.version.clone(),
                    path: self.resolve_path(dep, base_path),
                    details: e.to_string(),
                })))?;

            // Store in resolved map
            resolved.insert(dep.package_id.clone(), dep_package);
        }

        // Done loading this package
        self.loading.remove(&package.id);

        Ok(resolved)
    }

    /// Load a single dependency
    fn load_dependency(
        &mut self,
        dep: &Dependency,
        base_path: Option<&Path>,
    ) -> Result<Package, DependencyError> {
        // Check cache first
        if let Some(cached) = self.cache.get(&dep.package_id) {
            return Ok(cached.clone());
        }

        // Resolve path
        let dep_path = self.find_package_path(dep, base_path)?;

        // Load package
        let package = package_loader::load_package(&dep_path)
            .map_err(|e| DependencyError::LoadError {
                package_id: dep.package_id.clone(),
                path: dep_path.clone(),
                reason: e.to_string(),
            })?;

        // Validate package ID matches
        if package.id != dep.package_id {
            return Err(DependencyError::PackageIdMismatch {
                expected: dep.package_id.clone(),
                found: package.id.clone(),
                path: dep_path,
            });
        }

        // Cache it
        self.cache.insert(dep.package_id.clone(), package.clone());

        // Recursively resolve its dependencies
        let _sub_deps = self.resolve_dependencies(&package, dep_path.parent())?;

        Ok(package)
    }

    /// Find the path to a dependency package
    fn find_package_path(
        &self,
        dep: &Dependency,
        base_path: Option<&Path>,
    ) -> Result<PathBuf, DependencyError> {
        // If explicit path provided, use it
        if let Some(path_str) = &dep.path {
            let path = PathBuf::from(path_str);

            // Try as absolute path
            if path.is_absolute() && path.exists() {
                return Ok(path);
            }

            // Try relative to base_path
            if let Some(base) = base_path {
                let relative_path = base.join(&path);
                if relative_path.exists() {
                    return Ok(relative_path);
                }
            }

            // Try relative to current dir
            if path.exists() {
                return Ok(path);
            }
        }

        // Search in search_paths
        for search_path in &self.search_paths {
            let candidate = search_path.join(format!("{}.yaml", dep.package_id.replace('.', "-")));
            if candidate.exists() {
                return Ok(candidate);
            }
        }

        // Not found
        Err(DependencyError::NotFound {
            package_id: dep.package_id.clone(),
            searched_paths: self.format_searched_paths(dep, base_path),
        })
    }

    /// Helper to resolve path for error messages
    fn resolve_path(&self, dep: &Dependency, base_path: Option<&Path>) -> Option<PathBuf> {
        self.find_package_path(dep, base_path).ok()
    }

    /// Format the circular dependency cycle
    fn format_cycle(&self, target: &str) -> String {
        let mut cycle = vec![target.to_string()];
        for pkg_id in &self.loading {
            cycle.push(pkg_id.clone());
        }
        cycle.push(target.to_string());
        cycle.join(" -> ")
    }

    /// Format searched paths for error message
    fn format_searched_paths(&self, dep: &Dependency, base_path: Option<&Path>) -> Vec<String> {
        let mut paths = Vec::new();

        // Explicit path
        if let Some(path_str) = &dep.path {
            let path = PathBuf::from(path_str);

            if path.is_absolute() {
                paths.push(path.display().to_string());
            } else {
                if let Some(base) = base_path {
                    paths.push(base.join(&path).display().to_string());
                }
                paths.push(path.display().to_string());
            }
        }

        // Search paths
        for search_path in &self.search_paths {
            let candidate = search_path.join(format!("{}.yaml", dep.package_id.replace('.', "-")));
            paths.push(candidate.display().to_string());
        }

        paths
    }
}

/// Errors that can occur during dependency resolution
#[derive(Debug, Clone)]
pub enum DependencyError {
    /// Package not found
    NotFound {
        package_id: String,
        searched_paths: Vec<String>,
    },

    /// Failed to load package
    LoadError {
        package_id: String,
        path: PathBuf,
        reason: String,
    },

    /// Version mismatch (exact version required)
    /// Boxed to reduce Result size (clippy::result_large_err)
    VersionMismatch(Box<VersionMismatchData>),

    /// Package ID doesn't match expected
    PackageIdMismatch {
        expected: String,
        found: String,
        path: PathBuf,
    },

    /// Circular dependency detected
    CircularDependency {
        cycle: String,
    },
}

/// Data for version mismatch errors (boxed to reduce size)
#[derive(Debug, Clone)]
pub struct VersionMismatchData {
    pub package_id: String,
    pub required: String,
    pub found: String,
    pub path: Option<PathBuf>,
    pub details: String,
}

impl std::fmt::Display for DependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyError::NotFound { package_id, searched_paths } => {
                writeln!(f, "Dependency not found: {}", package_id)?;
                writeln!(f, "\nSearched paths:")?;
                for path in searched_paths {
                    writeln!(f, "  - {}", path)?;
                }
                writeln!(f, "\nSuggestion: Check the 'path' field or install the package")
            }

            DependencyError::LoadError { package_id, path, reason } => {
                writeln!(f, "Failed to load dependency: {}", package_id)?;
                writeln!(f, "  Path: {}", path.display())?;
                writeln!(f, "  Error: {}", reason)
            }

            DependencyError::VersionMismatch(data) => {
                writeln!(f, "Version mismatch for dependency: {}", data.package_id)?;
                writeln!(f, "  Required: {} (exact)", data.required)?;
                writeln!(f, "  Found: {}", data.found)?;
                if let Some(p) = &data.path {
                    writeln!(f, "  Location: {}", p.display())?;
                }
                writeln!(f, "\n{}", data.details)?;
                writeln!(f, "\nSuggestion: Update {} to version {} or change dependency to version: \"{}\"",
                    data.package_id, data.required, data.found)
            }

            DependencyError::PackageIdMismatch { expected, found, path } => {
                writeln!(f, "Package ID mismatch")?;
                writeln!(f, "  Expected: {}", expected)?;
                writeln!(f, "  Found: {}", found)?;
                writeln!(f, "  File: {}", path.display())?;
                writeln!(f, "\nSuggestion: Check the package ID in the file matches the dependency declaration")
            }

            DependencyError::CircularDependency { cycle } => {
                writeln!(f, "Circular dependency detected")?;
                writeln!(f, "  Cycle: {}", cycle)?;
                writeln!(f, "\nSuggestion: Remove one of the dependencies to break the cycle")
            }
        }
    }
}

impl std::error::Error for DependencyError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_resolver() {
        let paths = vec![PathBuf::from("./test-packages")];
        let resolver = DependencyResolver::new(paths.clone());
        assert_eq!(resolver.search_paths, paths);
        assert!(resolver.cache.is_empty());
        assert!(resolver.loading.is_empty());
    }

    #[test]
    fn test_format_cycle() {
        let mut resolver = DependencyResolver::new(vec![]);
        resolver.loading.insert("pkg-a".to_string());
        resolver.loading.insert("pkg-b".to_string());

        let cycle = resolver.format_cycle("pkg-c");
        assert!(cycle.contains("pkg-a"));
        assert!(cycle.contains("pkg-b"));
        assert!(cycle.contains("pkg-c"));
    }

    #[test]
    fn test_format_searched_paths() {
        let resolver = DependencyResolver::new(vec![
            PathBuf::from("./packages"),
            PathBuf::from("~/.rpg/packages"),
        ]);

        let dep = Dependency {
            package_id: "test.package".to_string(),
            version: "1.0.0".to_string(),
            path: Some("./local.yaml".to_string()),
        };

        let paths = resolver.format_searched_paths(&dep, Some(Path::new("./base")));
        assert!(!paths.is_empty());
        assert!(paths.iter().any(|p| p.contains("local.yaml")));
    }
}


use super::library_entry::{LibraryEntry, PackageSource};
use crate::core::Package;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const LIBRARY_VERSION: &str = "1.0.0";
const LIBRARY_FILE_NAME: &str = "library.json";

/// Package library managing all installed and local packages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageLibrary {
    /// Library format version
    pub version: String,

    /// All packages in the library
    pub packages: Vec<LibraryEntry>,

    /// Path to library root (not serialized)
    #[serde(skip)]
    library_path: PathBuf,
}

impl PackageLibrary {
    /// Load or create library from app data directory
    pub fn load(app_data_dir: &Path) -> Result<Self, String> {
        let library_path = app_data_dir.join("packages");
        let library_file = library_path.join(LIBRARY_FILE_NAME);

        // Ensure library directory exists
        fs::create_dir_all(&library_path)
            .map_err(|e| format!("Failed to create library directory: {}", e))?;

        // Create subdirectories
        fs::create_dir_all(library_path.join("installed")).ok();
        fs::create_dir_all(library_path.join("local")).ok();

        // Load existing library or create new one
        let library = if library_file.exists() {
            let content = fs::read_to_string(&library_file)
                .map_err(|e| format!("Failed to read library file: {}", e))?;

            let mut lib: PackageLibrary = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse library file: {}", e))?;

            lib.library_path = library_path;
            lib
        } else {
            // Create new library
            PackageLibrary {
                version: LIBRARY_VERSION.to_string(),
                packages: Vec::new(),
                library_path,
            }
        };

        // Save immediately to ensure file exists
        library.save()?;

        Ok(library)
    }

    /// Save library to disk
    pub fn save(&self) -> Result<(), String> {
        let library_file = self.library_path.join(LIBRARY_FILE_NAME);

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize library: {}", e))?;

        fs::write(&library_file, content)
            .map_err(|e| format!("Failed to write library file: {}", e))?;

        Ok(())
    }

    /// Install a package to the library
    pub fn install_package(
        &mut self,
        package: &Package,
        yaml_content: &str,
        source: PackageSource,
    ) -> Result<LibraryEntry, String> {
        // Determine subdirectory based on source
        let subdir = match source {
            PackageSource::Marketplace => "installed",
            PackageSource::Local => "local",
            PackageSource::Imported => "local",
        };

        // Generate filename
        let filename = format!("{}@{}.yaml", package.id, package.version);
        let relative_path = format!("{}/{}", subdir, filename);
        let full_path = self.library_path.join(&relative_path);

        // Write package file
        fs::write(&full_path, yaml_content)
            .map_err(|e| format!("Failed to write package file: {}", e))?;

        // Create library entry
        let entry = LibraryEntry::new(
            package.id.clone(),
            package.metadata.name.clone(),
            package.version.clone(),
            source,
            relative_path,
            package.metadata.authors.clone(),
            package.metadata.description.clone(),
        );

        // Check if package already exists (update instead of duplicate)
        if let Some(existing_idx) = self
            .packages
            .iter()
            .position(|e| e.id == entry.id && e.version == entry.version)
        {
            // Update existing entry
            self.packages[existing_idx] = entry.clone();
        } else {
            // Add new entry
            self.packages.push(entry.clone());
        }

        // Save library
        self.save()?;

        Ok(entry)
    }

    /// Remove a package from the library
    pub fn remove_package(&mut self, package_id: &str, version: &str) -> Result<(), String> {
        // Find entry
        let entry = self
            .packages
            .iter()
            .find(|e| e.id == package_id && e.version == version)
            .ok_or_else(|| format!("Package {}@{} not found in library", package_id, version))?
            .clone();

        // Delete file
        let full_path = self.library_path.join(&entry.path);
        if full_path.exists() {
            fs::remove_file(&full_path)
                .map_err(|e| format!("Failed to delete package file: {}", e))?;
        }

        // Remove from library
        self.packages.retain(|e| !(e.id == package_id && e.version == version));

        // Save library
        self.save()?;

        Ok(())
    }

    /// Load a package from the library
    pub fn load_package(&mut self, package_id: &str, version: &str) -> Result<Package, String> {
        // Find entry
        let entry_idx = self
            .packages
            .iter()
            .position(|e| e.id == package_id && e.version == version)
            .ok_or_else(|| format!("Package {}@{} not found in library", package_id, version))?;

        // Load package file
        let full_path = self.library_path.join(&self.packages[entry_idx].path);
        let package = crate::parser::load_package(&full_path)
            .map_err(|e| e.to_string())?;

        // Update last used timestamp
        self.packages[entry_idx].touch();
        self.save()?;

        Ok(package)
    }

    /// Load all packages from library
    pub fn load_all_packages(&self) -> Result<HashMap<String, Package>, String> {
        let mut packages = HashMap::new();

        for entry in &self.packages {
            let full_path = self.library_path.join(&entry.path);
            match crate::parser::load_package(&full_path) {
                Ok(package) => {
                    // Use "id@version" as key for unique identification
                    let key = format!("{}@{}", entry.id, entry.version);
                    packages.insert(key, package);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load package {}: {}", entry.id, e);
                    // Continue loading other packages
                }
            }
        }

        Ok(packages)
    }

    /// List all packages in library
    pub fn list_packages(&self) -> &[LibraryEntry] {
        &self.packages
    }

    /// Get library root path
    pub fn library_path(&self) -> &Path {
        &self.library_path
    }

    /// Find entry by package ID (returns latest version)
    #[allow(dead_code)]
    pub fn find_latest(&self, package_id: &str) -> Option<&LibraryEntry> {
        self.packages
            .iter()
            .filter(|e| e.id == package_id)
            .max_by(|a, b| a.version.cmp(&b.version))
    }

    /// Get all versions of a package
    #[allow(dead_code)]
    pub fn get_versions(&self, package_id: &str) -> Vec<&LibraryEntry> {
        self.packages
            .iter()
            .filter(|e| e.id == package_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_library_creation() {
        let temp_dir = TempDir::new().unwrap();
        let library = PackageLibrary::load(temp_dir.path()).unwrap();

        assert_eq!(library.version, LIBRARY_VERSION);
        assert_eq!(library.packages.len(), 0);
        assert!(temp_dir.path().join("packages/library.json").exists());
    }
}

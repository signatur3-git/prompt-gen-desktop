use serde::{Deserialize, Serialize};

/// Source of a package in the library
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PackageSource {
    /// Installed from marketplace
    Marketplace,
    /// Created locally by user
    Local,
    /// Imported from file system
    Imported,
}

/// Entry in the package library
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryEntry {
    /// Package ID (e.g., "namespace.package-name")
    pub id: String,

    /// Display name
    pub name: String,

    /// Package version
    pub version: String,

    /// Source of the package
    pub source: PackageSource,

    /// Relative path from library root (e.g., "installed/pkg@1.0.0.yaml")
    pub path: String,

    /// When package was installed/added
    #[serde(with = "chrono::serde::ts_seconds")]
    pub installed_at: chrono::DateTime<chrono::Utc>,

    /// Last time package was used/loaded
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_used: chrono::DateTime<chrono::Utc>,

    /// Package metadata
    pub metadata: LibraryEntryMetadata,
}

/// Metadata for a library entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryEntryMetadata {
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl LibraryEntry {
    pub fn new(
        id: String,
        name: String,
        version: String,
        source: PackageSource,
        path: String,
        authors: Vec<String>,
        description: Option<String>,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            name,
            version,
            source,
            path,
            installed_at: now,
            last_used: now,
            metadata: LibraryEntryMetadata {
                authors,
                description,
                tags: Vec::new(),
            },
        }
    }

    /// Mark this entry as recently used
    pub fn touch(&mut self) {
        self.last_used = chrono::Utc::now();
    }
}


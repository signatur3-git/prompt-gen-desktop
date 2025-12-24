// Package Storage Module
pub use package_library::PackageLibrary;
pub use library_entry::{LibraryEntry, PackageSource};

pub mod package_library;
pub mod library_entry;

// Provides a library system for installed, local, and imported packages.
// Manages persistent storage of packages in the user's app data directory.
//


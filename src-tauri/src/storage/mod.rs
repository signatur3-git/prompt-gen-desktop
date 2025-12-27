// Package Storage Module
pub use library_entry::{LibraryEntry, PackageSource};
pub use package_library::PackageLibrary;

pub mod library_entry;
pub mod package_library;

// Provides a library system for installed, local, and imported packages.
// Manages persistent storage of packages in the user's app data directory.
//

// M2: Parser module - Package loading and parsing

pub mod package_loader;
pub mod dependency_resolver; // M9: Dependency resolution

pub use package_loader::*;
pub use dependency_resolver::*;


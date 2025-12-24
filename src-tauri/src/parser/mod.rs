// M2: Parser module - Package loading and parsing

pub mod dependency_resolver;
pub mod package_loader; // M9: Dependency resolution

pub use dependency_resolver::*;
pub use package_loader::*;

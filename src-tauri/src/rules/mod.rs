// M4: Rules Module
// Execute coordination rules during enrichment phase

pub mod processor;

pub use processor::{RuleError, RulesProcessor};

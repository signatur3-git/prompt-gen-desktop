// M2: Foundation - Core Data Models
// Based on M1 decisions (DEC-0001, DEC-0002, DEC-0003)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Package - Root container for RPG content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    /// Unique package identifier
    pub id: String,

    /// Semantic version
    pub version: String,

    /// Package metadata
    pub metadata: PackageMetadata,

    /// Namespaces defined in this package
    pub namespaces: HashMap<String, Namespace>,

    /// Package dependencies
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub description: Option<String>,
    pub authors: Vec<String>,

    /// Optional: Bypass all tag filtering in this package (for absurdist packages)
    #[serde(default)]
    pub bypass_filters: bool,
}

/// Dependency - Package dependency with exact version matching
/// 
/// We enforce exact version matching (not semver ranges) to guarantee deterministic rendering.
/// See DESIGN_DECISION_EXACT_VERSION_MATCHING.md for rationale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Package ID to depend on (e.g., "prompt-gen.common")
    #[serde(alias = "package")]
    pub package_id: String,

    /// Exact version required (e.g., "1.0.0")
    /// Note: Semver ranges like "^1.0.0" or ">=1.0.0" are NOT supported.
    /// This ensures deterministic rendering: same package + same seed = same output.
    pub version: String,

    /// Optional path to local package file
    /// If not provided, package will be searched in standard locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Namespace - Organizational unit within a package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Namespace {
    /// Namespace identifier (e.g., "featured.common")
    pub id: String,

    /// Datatypes defined in this namespace
    #[serde(default)]
    pub datatypes: HashMap<String, Datatype>,

    /// Prompt sections (templates)
    #[serde(default)]
    pub prompt_sections: HashMap<String, PromptSection>,

    /// Separator sets for list formatting
    #[serde(default)]
    pub separator_sets: HashMap<String, SeparatorSet>,

    /// Rules for coordination (M1 Pattern 1, 2)
    /// Changed from Vec to HashMap to support rule referencing
    #[serde(default)]
    pub rules: HashMap<String, Rule>,

    /// Decisions for complex logic (M1 Pattern 3)
    #[serde(default)]
    pub decisions: Vec<Decision>,
    
    /// Rulebooks - Entry point wrappers for rendering (M9)
    #[serde(default)]
    pub rulebooks: HashMap<String, crate::core::rulebook::Rulebook>,
}

/// Datatype - Collection of selectable values with tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datatype {
    pub name: String,
    pub values: Vec<DatatypeValue>,

    /// Optional: Extends another datatype
    pub extends: Option<String>,

    /// Optional: Override tags when extending
    #[serde(default)]
    pub override_tags: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatatypeValue {
    /// The actual text value
    pub text: String,

    /// Tags for coordination (M1: article, plural, gender, etc.)
    #[serde(default)]
    pub tags: HashMap<String, serde_json::Value>,

    /// Optional weight for selection probability
    #[serde(default = "default_weight")]
    pub weight: f32,
}

fn default_weight() -> f32 {
    1.0
}

/// PromptSection - Template with references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptSection {
    pub name: String,

    /// Template string with {references}
    pub template: String,

    /// Reference definitions
    #[serde(default)]
    pub references: HashMap<String, Reference>,
}

/// Reference - How to select values in a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// Target datatype or promptsection (namespace:name format)
    pub target: String,

    /// Optional tag filter (M1 Pattern 4 - Tag Filtering)
    /// Examples:
    /// - Static: "mood:peaceful"
    /// - Dynamic: "{tags.requires_can_swim and ref:subject.tags.can_swim}"
    pub filter: Option<String>,

    /// Repetition parameters (M5 Phase 3+4)
    #[serde(default = "default_min")]
    pub min: usize,

    #[serde(default = "default_max")]
    pub max: usize,

    /// Separator set for lists (M5 Phase 3+4)
    pub separator: Option<String>,

    /// Whether to enforce uniqueness in multi-selection (M5 Phase 3+4)
    #[serde(default)]
    pub unique: bool,
}

fn default_min() -> usize {
    1
}

fn default_max() -> usize {
    1
}

/// SeparatorSet - List formatting rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeparatorSet {
    pub name: String,
    pub primary: String,    // ", "
    pub secondary: String,  // " and "
    pub tertiary: Option<String>,
}

/// Rule - Simple coordination logic (M1 Pattern 1, 2)
/// Stored in HashMap<String, Rule> where key is the rule ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Field to check (triggers the rule)
    pub when: String,

    /// Optional logic expression (empty = field exists)
    #[serde(default)]
    pub logic: String,

    /// Context field to write to
    pub set: String,

    /// Value to write (literal or expression)
    pub value: String,
}


/// Decision - Complex reusable logic (M1 Pattern 3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    /// Unique name (qualified with namespace)
    pub name: String,

    /// Input parameters
    pub inputs: HashMap<String, String>,  // name -> type

    /// Output parameters
    pub outputs: HashMap<String, String>, // name -> type

    /// Processor type and implementation
    pub processor: Processor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Processor {
    /// Simple expression evaluation
    #[serde(rename = "expression")]
    Expression { formula: String },

    /// Rule set with conditions
    #[serde(rename = "rule_set")]
    RuleSet { rules: Vec<ConditionalRule> },

    /// Script-based (for complex logic)
    #[serde(rename = "script")]
    Script {
        language: String,
        code: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalRule {
    pub condition: String,
    pub output: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datatype_value_default_weight() {
        let value = DatatypeValue {
            text: "test".to_string(),
            tags: HashMap::new(),
            weight: default_weight(),
        };
        assert_eq!(value.weight, 1.0);
    }

    #[test]
    fn test_reference_defaults() {
        let reference = Reference {
            target: "colors".to_string(),
            filter: None,
            min: default_min(),
            max: default_max(),
            separator: None,
            unique: false,
        };
        assert_eq!(reference.min, 1);
        assert_eq!(reference.max, 1);
    }
}


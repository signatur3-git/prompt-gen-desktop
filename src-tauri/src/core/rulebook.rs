use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an entry point in a rulebook with its weight
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntryPoint {
    /// Reference to a prompt section (namespace:name)
    pub prompt_section: String,

    /// Weight for weighted random selection (default: 1.0)
    #[serde(default = "default_weight")]
    pub weight: f64,
}

fn default_weight() -> f64 {
    1.0
}

/// Represents a rulebook that defines multiple entry points and optional context defaults
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rulebook {
    /// Human-readable name
    pub name: String,

    /// Description of what this rulebook does
    #[serde(default)]
    pub description: String,

    /// List of possible entry points with weights
    pub entry_points: Vec<EntryPoint>,

    /// Whether to avoid repeating the same entry point in batch rendering
    #[serde(default)]
    pub batch_variety: bool,

    /// Optional context defaults to set before rendering
    #[serde(default)]
    pub context_defaults: HashMap<String, String>,
}

impl Rulebook {
    /// Validate the rulebook
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Rulebook name cannot be empty".to_string());
        }

        if self.entry_points.is_empty() {
            return Err(format!("Rulebook '{}' must have at least one entry point", self.name));
        }

        // Validate all weights are positive
        for (idx, ep) in self.entry_points.iter().enumerate() {
            if ep.weight <= 0.0 {
                return Err(format!(
                    "Rulebook '{}' entry point {} has invalid weight: {}. Weights must be positive.",
                    self.name, idx, ep.weight
                ));
            }

            if ep.prompt_section.is_empty() {
                return Err(format!(
                    "Rulebook '{}' entry point {} has empty prompt_section reference",
                    self.name, idx
                ));
            }
        }

        // Check for duplicate entry points
        let mut seen = std::collections::HashSet::new();
        for ep in &self.entry_points {
            if !seen.insert(&ep.prompt_section) {
                return Err(format!(
                    "Rulebook '{}' has duplicate entry point: {}",
                    self.name, ep.prompt_section
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rulebook_validation_success() {
        let rulebook = Rulebook {
            name: "test_rulebook".to_string(),
            description: "Test rulebook".to_string(),
            entry_points: vec![
                EntryPoint {
                    prompt_section: "test:scene1".to_string(),
                    weight: 1.0,
                },
                EntryPoint {
                    prompt_section: "test:scene2".to_string(),
                    weight: 2.0,
                },
            ],
            batch_variety: false,
            context_defaults: HashMap::new(),
        };

        assert!(rulebook.validate().is_ok());
    }

    #[test]
    fn test_rulebook_validation_empty_name() {
        let rulebook = Rulebook {
            name: "".to_string(),
            description: "".to_string(),
            entry_points: vec![
                EntryPoint {
                    prompt_section: "test:scene1".to_string(),
                    weight: 1.0,
                },
            ],
            batch_variety: false,
            context_defaults: HashMap::new(),
        };

        assert!(rulebook.validate().is_err());
        assert!(rulebook.validate().unwrap_err().contains("name cannot be empty"));
    }

    #[test]
    fn test_rulebook_validation_no_entry_points() {
        let rulebook = Rulebook {
            name: "test".to_string(),
            description: "".to_string(),
            entry_points: vec![],
            batch_variety: false,
            context_defaults: HashMap::new(),
        };

        assert!(rulebook.validate().is_err());
        assert!(rulebook.validate().unwrap_err().contains("at least one entry point"));
    }

    #[test]
    fn test_rulebook_validation_invalid_weight() {
        let rulebook = Rulebook {
            name: "test".to_string(),
            description: "".to_string(),
            entry_points: vec![
                EntryPoint {
                    prompt_section: "test:scene1".to_string(),
                    weight: -1.0,
                },
            ],
            batch_variety: false,
            context_defaults: HashMap::new(),
        };

        assert!(rulebook.validate().is_err());
        assert!(rulebook.validate().unwrap_err().contains("invalid weight"));
    }

    #[test]
    fn test_rulebook_validation_duplicate_entry_points() {
        let rulebook = Rulebook {
            name: "test".to_string(),
            description: "".to_string(),
            entry_points: vec![
                EntryPoint {
                    prompt_section: "test:scene1".to_string(),
                    weight: 1.0,
                },
                EntryPoint {
                    prompt_section: "test:scene1".to_string(),
                    weight: 2.0,
                },
            ],
            batch_variety: false,
            context_defaults: HashMap::new(),
        };

        assert!(rulebook.validate().is_err());
        assert!(rulebook.validate().unwrap_err().contains("duplicate entry point"));
    }
}


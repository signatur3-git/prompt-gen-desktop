// M3: Value Selector
// Selects values from datatypes using seeded randomness
// M5 Phase 2: Complex tag expressions (AND/OR/NOT)
// M8.5 Blocker 1: Cross-reference filtering support

use crate::core::{Datatype, DatatypeValue, Package};
use crate::renderer::seeded_random::SeededRandom;
use crate::renderer::tag_expression::{evaluate_with_context, ExpressionParser};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SelectionError {
    #[error("Datatype not found: {0}")]
    DatatypeNotFound(String),

    #[error("Datatype has no values: {0}")]
    EmptyDatatype(String),

    #[error("Reference format invalid: {0}")]
    InvalidReference(String),

    #[error("Tag expression parse error: {0}")]
    TagExpressionParse(String),

    #[error("Filter matched no values: {0}")]
    NoMatchingValues(String),

    #[error("Cannot select {requested} unique values from datatype with only {available} values")]
    NotEnoughUniqueValues { requested: usize, available: usize },
}

pub type Result<T> = std::result::Result<T, SelectionError>;

/// A selected value with its text and tags
#[derive(Debug, Clone)]
pub struct SelectedValue {
    pub text: String,
    pub tags: HashMap<String, serde_json::Value>,
}

/// Selects values from datatypes
/// M9 Phase 2.7: Supports cross-package references via dependencies
pub struct Selector<'a> {
    package: &'a Package,
    dependencies: Option<&'a HashMap<String, Package>>,
    rng: SeededRandom,
}

impl<'a> Selector<'a> {
    /// Create a new selector with a package and seed (no dependencies)
    pub fn new(package: &'a Package, seed: u64) -> Self {
        Selector {
            package,
            dependencies: None,
            rng: SeededRandom::new(seed),
        }
    }

    /// M9 Phase 2.7: Create a new selector with dependencies
    pub fn new_with_dependencies(
        package: &'a Package,
        dependencies: &'a HashMap<String, Package>,
        seed: u64,
    ) -> Self {
        Selector {
            package,
            dependencies: Some(dependencies),
            rng: SeededRandom::new(seed),
        }
    }

    /// Select a value from a datatype reference
    ///
    /// Reference format: "datatype" or "namespace:datatype"
    /// M3: Simple selection only
    /// M4: Tag filtering
    /// M8.5 Blocker 1: Cross-reference filtering (pass empty HashMap for backward compatibility)
    #[allow(dead_code)] // Use select_with_filter instead
    pub fn select(&mut self, reference: &str) -> Result<SelectedValue> {
        self.select_with_filter(reference, None, &HashMap::new())
    }

    /// Select a value from a datatype reference with optional filter
    ///
    /// Filter format: "tags.can_fly" or "tags.mood == peaceful" or "ref:other.text in tags.applies_to"
    /// M4: Tag filtering implementation
    /// M8.5 Blocker 1: Cross-reference filtering support
    pub fn select_with_filter(
        &mut self,
        reference: &str,
        filter: Option<&str>,
        selected: &HashMap<String, (String, HashMap<String, serde_json::Value>)>,
    ) -> Result<SelectedValue> {
        // Parse reference (simple version for M3)
        let (namespace, datatype_name) = self.parse_reference(reference)?;

        // Find datatype and clone its values to avoid borrowing issues
        let datatype = self.find_datatype(&namespace, &datatype_name)?;
        let mut values = datatype.values.clone();

        // M4: Apply filter if provided
        if let Some(filter_expr) = filter {
            values = self.apply_filter(values, filter_expr, selected)?;
        }

        // Select value
        self.select_from_values(&values, &datatype_name)
    }

    /// M5 Phase 3+4: Select multiple values with optional uniqueness constraint
    ///
    /// Returns a vector of selected values
    /// If unique=true, ensures no duplicates (may return fewer than count if not enough unique values)
    pub fn select_multiple(
        &mut self,
        reference: &str,
        count: usize,
        filter: Option<&str>,
        unique: bool,
        selected: &HashMap<String, (String, HashMap<String, serde_json::Value>)>,
    ) -> Result<Vec<SelectedValue>> {
        if count == 0 {
            return Ok(Vec::new());
        }

        // Parse reference
        let (namespace, datatype_name) = self.parse_reference(reference)?;

        // Find datatype and clone its values
        let datatype = self.find_datatype(&namespace, &datatype_name)?;
        let mut values = datatype.values.clone();

        // Apply filter if provided
        if let Some(filter_expr) = filter {
            values = self.apply_filter(values, filter_expr, selected)?;
        }

        if unique {
            // For unique selections, we need at least `count` values available
            if values.len() < count {
                return Err(SelectionError::NotEnoughUniqueValues {
                    requested: count,
                    available: values.len(),
                });
            }

            // Select without replacement
            self.select_unique_values(&values, count, &datatype_name)
        } else {
            // Select with replacement (can repeat)
            let mut results = Vec::new();
            for _ in 0..count {
                let value = self.select_from_values(&values, &datatype_name)?;
                results.push(value);
            }
            Ok(results)
        }
    }

    /// Select N unique values without replacement
    fn select_unique_values(
        &mut self,
        values: &[DatatypeValue],
        count: usize,
        _name: &str,
    ) -> Result<Vec<SelectedValue>> {
        let mut remaining = values.to_vec();
        let mut results = Vec::new();

        for _ in 0..count {
            if remaining.is_empty() {
                break;
            }

            // Select from remaining values
            let weights: Vec<f32> = remaining.iter().map(|v| v.weight).collect();
            let index = self.rng.weighted_choice(&weights);

            // Take the selected value and remove it from remaining
            let value = remaining.remove(index);
            results.push(SelectedValue {
                text: value.text,
                tags: value.tags,
            });
        }

        Ok(results)
    }

    /// Parse reference into namespace and datatype name
    /// Format: "datatype" uses first namespace, "namespace:datatype" is explicit
    fn parse_reference(&self, reference: &str) -> Result<(String, String)> {
        if reference.contains(':') {
            let parts: Vec<&str> = reference.split(':').collect();
            if parts.len() != 2 {
                return Err(SelectionError::InvalidReference(reference.to_string()));
            }
            Ok((parts[0].to_string(), parts[1].to_string()))
        } else {
            // Use first namespace as default (or could use package default)
            let first_namespace = self.package.namespaces.keys().next().ok_or_else(|| {
                SelectionError::InvalidReference("No namespaces in package".to_string())
            })?;

            Ok((first_namespace.clone(), reference.to_string()))
        }
    }

    /// Find a datatype by namespace and name
    /// M9 Phase 2.7: Searches dependencies if not found in main package
    fn find_datatype(&self, namespace: &str, name: &str) -> Result<&Datatype> {
        // First try main package
        if let Some(ns) = self.package.namespaces.get(namespace) {
            if let Some(dt) = ns.datatypes.get(name) {
                return Ok(dt);
            }
        }

        // M9: If not found and we have dependencies, search them
        if let Some(deps) = self.dependencies {
            for dep_package in deps.values() {
                if let Some(ns) = dep_package.namespaces.get(namespace) {
                    if let Some(dt) = ns.datatypes.get(name) {
                        return Ok(dt);
                    }
                }
            }
        }

        // Not found anywhere
        Err(SelectionError::DatatypeNotFound(format!(
            "{}:{}",
            namespace, name
        )))
    }

    /// Select a value from a list of datatype values, respecting weights
    fn select_from_values(
        &mut self,
        values: &[DatatypeValue],
        name: &str,
    ) -> Result<SelectedValue> {
        if values.is_empty() {
            return Err(SelectionError::EmptyDatatype(name.to_string()));
        }

        // Extract weights
        let weights: Vec<f32> = values.iter().map(|v| v.weight).collect();

        // Select index using weighted choice
        let index = self.rng.weighted_choice(&weights);

        // Get selected value
        let value = &values[index];

        Ok(SelectedValue {
            text: value.text.clone(),
            tags: value.tags.clone(),
        })
    }

    /// Apply filter expression to values
    ///
    /// M5 Phase 2: Complex tag expressions with AND/OR/NOT
    /// M8.5 Blocker 1: Cross-reference filtering support
    /// Supports: tags.can_fly, tags.type == "melee", ref:other.text in tags.applies_to
    fn apply_filter(
        &self,
        values: Vec<DatatypeValue>,
        filter_expr: &str,
        selected: &HashMap<String, (String, HashMap<String, serde_json::Value>)>,
    ) -> Result<Vec<DatatypeValue>> {
        // Parse the filter expression
        let expression = ExpressionParser::parse(filter_expr)
            .map_err(|e| SelectionError::TagExpressionParse(e.to_string()))?;

        // Filter values using the parsed expression with selected values context
        let filtered: Vec<DatatypeValue> = values
            .into_iter()
            .filter(|value| evaluate_with_context(&expression, &value.tags, selected))
            .collect();

        // Check if any values matched
        if filtered.is_empty() {
            return Err(SelectionError::NoMatchingValues(filter_expr.to_string()));
        }

        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Namespace, PackageMetadata};

    fn create_test_package() -> Package {
        let mut datatypes = HashMap::new();

        // Colors datatype
        datatypes.insert(
            "colors".to_string(),
            Datatype {
                name: "colors".to_string(),
                values: vec![
                    DatatypeValue {
                        text: "red".to_string(),
                        tags: HashMap::new(),
                        weight: 1.0,
                    },
                    DatatypeValue {
                        text: "blue".to_string(),
                        tags: HashMap::new(),
                        weight: 1.0,
                    },
                ],
                extends: None,
                override_tags: HashMap::new(),
            },
        );

        let mut namespaces = HashMap::new();
        namespaces.insert(
            "test".to_string(),
            Namespace {
                id: "test".to_string(),
                datatypes,
                prompt_sections: HashMap::new(),
                separator_sets: HashMap::new(),
                rules: HashMap::new(),
                decisions: Vec::new(),
                rulebooks: HashMap::new(), // M9
            },
        );

        Package {
            id: "test.package".to_string(),
            version: "1.0.0".to_string(),
            metadata: PackageMetadata {
                name: "Test Package".to_string(),
                description: None,
                authors: Vec::new(),
                bypass_filters: false,
            },
            namespaces,
            dependencies: Vec::new(),
        }
    }

    #[test]
    fn test_select_simple() {
        let package = create_test_package();
        let mut selector = Selector::new(&package, 42);

        let result = selector.select("colors").unwrap();
        assert!(result.text == "red" || result.text == "blue");
    }

    #[test]
    fn test_select_with_namespace() {
        let package = create_test_package();
        let mut selector = Selector::new(&package, 42);

        let result = selector.select("test:colors").unwrap();
        assert!(result.text == "red" || result.text == "blue");
    }

    #[test]
    fn test_select_deterministic() {
        let package = create_test_package();

        let mut selector1 = Selector::new(&package, 12345);
        let mut selector2 = Selector::new(&package, 12345);

        let result1 = selector1.select("colors").unwrap();
        let result2 = selector2.select("colors").unwrap();

        assert_eq!(result1.text, result2.text);
    }

    #[test]
    fn test_select_nonexistent_datatype() {
        let package = create_test_package();
        let mut selector = Selector::new(&package, 42);

        let result = selector.select("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_select_with_filter() {
        let mut datatypes = HashMap::new();

        // Animals datatype with can_fly tag
        datatypes.insert(
            "animals".to_string(),
            Datatype {
                name: "animals".to_string(),
                values: vec![
                    DatatypeValue {
                        text: "eagle".to_string(),
                        tags: {
                            let mut tags = HashMap::new();
                            tags.insert("can_fly".to_string(), serde_json::Value::Bool(true));
                            tags
                        },
                        weight: 1.0,
                    },
                    DatatypeValue {
                        text: "deer".to_string(),
                        tags: {
                            let mut tags = HashMap::new();
                            tags.insert("can_fly".to_string(), serde_json::Value::Bool(false));
                            tags
                        },
                        weight: 1.0,
                    },
                    DatatypeValue {
                        text: "swan".to_string(),
                        tags: {
                            let mut tags = HashMap::new();
                            tags.insert("can_fly".to_string(), serde_json::Value::Bool(true));
                            tags
                        },
                        weight: 1.0,
                    },
                ],
                extends: None,
                override_tags: HashMap::new(),
            },
        );

        let mut namespaces = HashMap::new();
        namespaces.insert(
            "test".to_string(),
            Namespace {
                id: "test".to_string(),
                datatypes,
                prompt_sections: HashMap::new(),
                separator_sets: HashMap::new(),
                rules: HashMap::new(),
                decisions: Vec::new(),
                rulebooks: HashMap::new(), // M9
            },
        );

        let package = Package {
            id: "test.package".to_string(),
            version: "1.0.0".to_string(),
            metadata: PackageMetadata {
                name: "Test Package".to_string(),
                description: None,
                authors: Vec::new(),
                bypass_filters: false,
            },
            namespaces,
            dependencies: Vec::new(),
        };

        let mut selector = Selector::new(&package, 42);

        // Select with filter - should only get flying animals
        let result = selector
            .select_with_filter("animals", Some("tags.can_fly"), &HashMap::new())
            .unwrap();
        assert!(result.text == "eagle" || result.text == "swan");
        assert_ne!(result.text, "deer");
    }

    #[test]
    fn test_filter_no_matches() {
        let mut datatypes = HashMap::new();

        // Animals without can_swim tag
        datatypes.insert(
            "animals".to_string(),
            Datatype {
                name: "animals".to_string(),
                values: vec![DatatypeValue {
                    text: "eagle".to_string(),
                    tags: HashMap::new(),
                    weight: 1.0,
                }],
                extends: None,
                override_tags: HashMap::new(),
            },
        );

        let mut namespaces = HashMap::new();
        namespaces.insert(
            "test".to_string(),
            Namespace {
                id: "test".to_string(),
                datatypes,
                prompt_sections: HashMap::new(),
                separator_sets: HashMap::new(),
                rules: HashMap::new(),
                decisions: Vec::new(),
                rulebooks: HashMap::new(), // M9
            },
        );

        let package = Package {
            id: "test.package".to_string(),
            version: "1.0.0".to_string(),
            metadata: PackageMetadata {
                name: "Test Package".to_string(),
                description: None,
                authors: Vec::new(),
                bypass_filters: false,
            },
            namespaces,
            dependencies: Vec::new(),
        };

        let mut selector = Selector::new(&package, 42);

        // This filter should match nothing - expecting error
        let result = selector.select_with_filter("animals", Some("tags.can_swim"), &HashMap::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_cross_ref_filter() {
        // Test selecting with cross-reference filter
        let mut datatypes = HashMap::new();

        datatypes.insert(
            "features".to_string(),
            Datatype {
                name: "features".to_string(),
                values: vec![DatatypeValue {
                    text: "eyes".to_string(),
                    tags: HashMap::new(),
                    weight: 1.0,
                }],
                extends: None,
                override_tags: HashMap::new(),
            },
        );

        datatypes.insert(
            "adjectives".to_string(),
            Datatype {
                name: "adjectives".to_string(),
                values: vec![DatatypeValue {
                    text: "blue".to_string(),
                    tags: {
                        let mut tags = HashMap::new();
                        tags.insert(
                            "applies_to".to_string(),
                            serde_json::json!(["eyes", "claws"]),
                        );
                        tags
                    },
                    weight: 1.0,
                }],
                extends: None,
                override_tags: HashMap::new(),
            },
        );

        let mut namespaces = HashMap::new();
        namespaces.insert(
            "test".to_string(),
            Namespace {
                id: "test".to_string(),
                datatypes,
                prompt_sections: HashMap::new(),
                separator_sets: HashMap::new(),
                rules: HashMap::new(),
                decisions: Vec::new(),
                rulebooks: HashMap::new(), // M9
            },
        );

        let package = Package {
            id: "test.package".to_string(),
            version: "1.0.0".to_string(),
            metadata: PackageMetadata {
                name: "Test".to_string(),
                description: None,
                authors: Vec::new(),
                bypass_filters: false,
            },
            namespaces,
            dependencies: Vec::new(),
        };

        let mut selector = Selector::new(&package, 42);

        // Test selecting with cross-reference filter
        let mut selected = HashMap::new();
        selected.insert(
            "feature".to_string(),
            ("eyes".to_string(), {
                let mut tags = HashMap::new();
                tags.insert(
                    "applies_to".to_string(),
                    serde_json::json!(["eyes", "claws"]),
                );
                tags
            }),
        );

        let result = selector.select_with_filter(
            "adjectives",
            Some("ref:feature.text in tags.applies_to"),
            &selected,
        );
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value.text, "blue"); // Value with "eyes" in applies_to, matching ref:feature.text
    }

    #[test]
    fn test_cross_ref_filter_no_match() {
        // Test when the cross-reference doesn't match any values
        let mut datatypes = HashMap::new();

        datatypes.insert(
            "features".to_string(),
            Datatype {
                name: "features".to_string(),
                values: vec![DatatypeValue {
                    text: "sharp".to_string(),
                    tags: HashMap::new(),
                    weight: 1.0,
                }],
                extends: None,
                override_tags: HashMap::new(),
            },
        );

        datatypes.insert(
            "adjectives".to_string(),
            Datatype {
                name: "adjectives".to_string(),
                values: vec![DatatypeValue {
                    text: "blue".to_string(),
                    tags: {
                        let mut tags = HashMap::new();
                        tags.insert("applies_to".to_string(), serde_json::json!(["wings"]));
                        tags
                    },
                    weight: 1.0,
                }],
                extends: None,
                override_tags: HashMap::new(),
            },
        );

        let mut namespaces = HashMap::new();
        namespaces.insert(
            "test".to_string(),
            Namespace {
                id: "test".to_string(),
                datatypes,
                prompt_sections: HashMap::new(),
                separator_sets: HashMap::new(),
                rules: HashMap::new(),
                decisions: Vec::new(),
                rulebooks: HashMap::new(), // M9
            },
        );

        let package = Package {
            id: "test.package".to_string(),
            version: "1.0.0".to_string(),
            metadata: PackageMetadata {
                name: "Test".to_string(),
                description: None,
                authors: Vec::new(),
                bypass_filters: false,
            },
            namespaces,
            dependencies: Vec::new(),
        };
        let mut selector = Selector::new(&package, 42);
        // Test selecting with cross-reference that doesn't match
        let mut selected = HashMap::new();
        selected.insert(
            "feature".to_string(),
            ("sharp".to_string(), {
                let mut tags = HashMap::new();
                tags.insert(
                    "applies_to".to_string(),
                    serde_json::json!(["eyes", "claws"]),
                );
                tags
            }),
        );
        // This should fail because "blue" only applies to "wings", not "eyes" or "claws"
        let result = selector.select_with_filter(
            "adjectives",
            Some("ref:feature.text in tags.applies_to"),
            &selected,
        );
        assert!(result.is_err());
    }
}

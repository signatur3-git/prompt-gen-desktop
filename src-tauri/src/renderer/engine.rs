// M3: Three-Phase Renderer
// Implements the three-phase rendering pipeline
// M4: Added Rules execution in Phase 2

use crate::context::Context;
use crate::core::{Package, PromptSection};
use crate::renderer::template_parser::{Template, TemplateToken};
use crate::renderer::selector::{Selector, SelectedValue};
use crate::rules::RulesProcessor;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Template parse error: {0}")]
    TemplateParse(#[from] crate::renderer::template_parser::ParseError),

    #[error("Selection error: {0}")]
    Selection(#[from] crate::renderer::selector::SelectionError),

    #[error("PromptSection not found: {0}")]
    PromptSectionNotFound(String),

    #[error("Reference not selected: {0}")]
    ReferenceNotSelected(String),

    #[error("Rule execution error: {0}")]
    RuleExecution(#[from] crate::rules::RuleError),

    #[error("Context error: {0}")]
    Context(#[from] crate::context::ContextError),

    #[error("Maximum recursion depth ({0}) exceeded for promptsection: {1}")]
    MaxRecursionDepth(usize, String),
}

pub type Result<T> = std::result::Result<T, RenderError>;

/// Maximum nesting depth for promptsections (prevent infinite recursion)
const MAX_RECURSION_DEPTH: usize = 10;

/// Result of rendering
#[derive(Debug, Clone, serde::Serialize)]
pub struct RenderResult {
    /// The final rendered output
    pub output: String,

    /// The seed used for rendering
    pub seed: u64,

    /// Selected values (for debugging)
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub selected_values: HashMap<String, String>,
}

/// Three-phase renderer
/// M8.5 Blocker 2 Phase 2: Supports cross-package rule execution
pub struct Renderer<'a> {
    package: &'a Package,
    dependencies: Option<&'a HashMap<String, Package>>,
    seed: u64,
}

impl<'a> Renderer<'a> {
    /// Create a new renderer without dependencies
    pub fn new(package: &'a Package, seed: u64) -> Self {
        Renderer {
            package,
            dependencies: None, // No dependencies
            seed
        }
    }

    /// Create a new renderer with dependencies
    /// M8.5 Blocker 2 Phase 2: Cross-package rule execution
    pub fn new_with_dependencies(
        package: &'a Package,
        dependencies: &'a HashMap<String, Package>,
        seed: u64
    ) -> Self {
        Renderer { package, dependencies: Some(dependencies), seed }
    }

    /// Render a promptsection by name
    ///
    /// Reference format: "section" or "namespace:section"
    pub fn render(&self, promptsection_ref: &str) -> Result<RenderResult> {
        self.render_with_context(promptsection_ref, None)
    }

    /// Render a promptsection with initial context values
    ///
    /// M9: Used by rulebooks to apply context defaults
    fn render_with_context(
        &self,
        promptsection_ref: &str,
        initial_context: Option<HashMap<String, String>>,
    ) -> Result<RenderResult> {
        self.render_with_depth_and_context(promptsection_ref, 0, initial_context)
    }

    /// Render a promptsection with recursion depth tracking (M5 Phase 1)
    ///
    /// This allows nested promptsections while preventing infinite recursion
    fn render_with_depth(&self, promptsection_ref: &str, depth: usize) -> Result<RenderResult> {
        self.render_with_depth_and_context(promptsection_ref, depth, None)
    }

    /// Render a promptsection with recursion depth tracking and initial context
    ///
    /// M9: Extended to support initial context values from rulebooks
    fn render_with_depth_and_context(
        &self,
        promptsection_ref: &str,
        depth: usize,
        initial_context: Option<HashMap<String, String>>,
    ) -> Result<RenderResult> {
        // Check recursion depth
        if depth > MAX_RECURSION_DEPTH {
            return Err(RenderError::MaxRecursionDepth(
                MAX_RECURSION_DEPTH,
                promptsection_ref.to_string()
            ));
        }

        // Find promptsection
        let promptsection = self.find_promptsection(promptsection_ref)?;

        // Find namespace for this promptsection
        let namespace = self.find_namespace_for_promptsection(promptsection_ref)?;

        // Create context
        let mut context = Context::new();

        // M9: Apply initial context values (e.g., from rulebook context_defaults)
        if let Some(defaults) = initial_context {
            for (key, value) in defaults {
                context.set(&key, value)?;
            }
        }

        // Three phases
        let selected = self.phase_1_selection(promptsection, depth)?;
        self.phase_2_enrichment(&mut context, &selected, namespace)?;
        let output = self.phase_3_rendering(&promptsection.template, promptsection, &selected, &context, namespace)?;

        // Build result - include selected values (flatten Vec to first item for display)
        let mut selected_values: HashMap<String, String> = selected
            .iter()
            .filter_map(|(k, values)| {
                values.first().map(|v| (k.clone(), v.text.clone()))
            })
            .collect();

        // Add context values from all scopes (with "context:" prefix to distinguish them)
        // Check common scopes: prompt, global, and any custom ones
        for scope_name in &["prompt", "global"] {
            if let Some(scope) = context.get_scope(scope_name) {
                for (key, value) in scope {
                    selected_values.insert(
                        format!("context:{}", key),
                        value.to_string()
                    );
                }
            }
        }

        Ok(RenderResult {
            output,
            seed: self.seed,
            selected_values,
        })
    }

    /// PHASE 1: Selection - Parse template and select values
    ///
    /// M5 Phase 1: Can recursively render nested promptsections
    /// M5 Phase 3+4: Can select multiple values per reference
    /// M8.5 Blocker 1: Cross-reference filtering with dependency ordering
    fn phase_1_selection(&self, promptsection: &PromptSection, depth: usize) -> Result<HashMap<String, Vec<SelectedValue>>> {
        // Parse template
        let parsed = Template::parse(&promptsection.template)?;

        // M8.5 Blocker 1 Part 2: Determine selection order based on dependencies
        let selection_order = self.compute_selection_order(promptsection, &parsed)?;

        // M9 Phase 2.7: Create selector with dependencies if available
        let mut selector = if let Some(deps) = self.dependencies {
            Selector::new_with_dependencies(self.package, deps, self.seed)
        } else {
            Selector::new(self.package, self.seed)
        };

        // M8.5: Build selection context for cross-reference filtering
        // Maps ref_name -> (text, tags) for already-selected values
        let mut selection_context: HashMap<String, (String, HashMap<String, serde_json::Value>)> = HashMap::new();

        // Select value(s) for each reference in dependency order
        let mut selected = HashMap::new();

        for ref_name in selection_order {
            // Look up the reference definition to get the target and parameters
            let reference = promptsection.references.get(&ref_name)
                .ok_or_else(|| RenderError::ReferenceNotSelected(
                    format!("Reference '{}' not defined in promptsection", ref_name)
                ))?;

            // Skip context references - they'll be populated by Rules during Phase 2
            if reference.target.starts_with("context:") {
                continue;
            }

            // Get filter from reference definition
            let filter = reference.filter.as_deref();

            // M5 Phase 3+4: Use parameters from YAML Reference
            let min = reference.min;
            let max = reference.max;
            let unique = reference.unique;

            // M5 Phase 1: Check if this is a nested promptsection reference
            if self.is_promptsection_reference(&reference.target) {
                // Render the nested promptsection recursively
                let nested_result = self.render_with_depth(&reference.target, depth + 1)?;

                // Store the rendered output as a selected value
                let selected_val = SelectedValue {
                    text: nested_result.output.clone(),
                    tags: HashMap::new(), // Nested sections don't have tags
                };

                // Add to selection context for cross-reference filtering
                selection_context.insert(ref_name.clone(), (nested_result.output, HashMap::new()));

                selected.insert(ref_name.clone(), vec![selected_val]);
            } else {
                // M5 Phase 3+4: Determine how many values to select
                let count = if min == max {
                    min
                } else {
                    // Use selector's RNG to pick between min and max
                    use crate::renderer::seeded_random::SeededRandom;
                    let mut temp_rng = SeededRandom::new(self.seed + ref_name.len() as u64);
                    temp_rng.gen_range(min..=max)
                };

                // M8.5: Select values with cross-reference filtering support
                let values = if count > 1 {
                    selector.select_multiple(&reference.target, count, filter, unique, &selection_context)?
                } else if count == 1 {
                    vec![selector.select_with_filter(&reference.target, filter, &selection_context)?]
                } else {
                    // count == 0, return empty vec
                    Vec::new()
                };

                // M8.5: Add first selected value to context for subsequent cross-reference filtering
                if let Some(first_value) = values.first() {
                    selection_context.insert(
                        ref_name.clone(),
                        (first_value.text.clone(), first_value.tags.clone())
                    );
                }

                selected.insert(ref_name.clone(), values);
            }
        }

        Ok(selected)
    }

    /// Compute selection order based on filter dependencies
    ///
    /// M8.5 Blocker 1 Part 2: Dependency ordering
    /// References with filters that depend on other references must be selected after those references
    fn compute_selection_order(&self, promptsection: &PromptSection, parsed: &Template) -> Result<Vec<String>> {
        use crate::renderer::tag_expression::ExpressionParser;

        // Get all reference names from template (in template order)
        let mut ref_names: Vec<String> = Vec::new();
        for token in &parsed.tokens {
            if let TemplateToken::Reference { name, .. } = token {
                if !ref_names.contains(name) {
                    ref_names.push(name.clone());
                }
            }
        }

        // Build dependency graph
        let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();

        for ref_name in &ref_names {
            if let Some(reference) = promptsection.references.get(ref_name) {
                // Skip context references
                if reference.target.starts_with("context:") {
                    continue;
                }

                // Extract dependencies from filter
                if let Some(filter_expr) = &reference.filter {
                    if let Ok(parsed_filter) = ExpressionParser::parse(filter_expr) {
                        let deps = crate::renderer::tag_expression::extract_ref_dependencies(&parsed_filter);
                        if !deps.is_empty() {
                            dependencies.insert(ref_name.clone(), deps);
                        }
                    }
                }
            }
        }

        // Topological sort using Kahn's algorithm
        let sorted = self.topological_sort(&ref_names, &dependencies)?;

        Ok(sorted)
    }

    /// Topological sort using Kahn's algorithm
    /// Detects circular dependencies
    fn topological_sort(&self, nodes: &[String], dependencies: &HashMap<String, Vec<String>>) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();

        // Initialize in-degree for all nodes
        for node in nodes {
            in_degree.insert(node.clone(), 0);
        }

        // Calculate in-degrees
        for (node, deps) in dependencies {
            for dep in deps {
                // Only count dependencies that exist in our node set
                if in_degree.contains_key(dep) {
                    *in_degree.get_mut(node).unwrap() += 1;
                }
            }
        }

        // Find all nodes with in-degree 0
        let mut queue: Vec<String> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(node, _)| node.clone())
            .collect();

        // Process nodes in template order for determinism
        queue.sort_by_key(|name| nodes.iter().position(|n| n == name).unwrap_or(usize::MAX));

        while let Some(node) = queue.pop() {
            result.push(node.clone());

            // Find all nodes that depend on this node
            for (dependent, deps) in dependencies {
                if deps.contains(&node) {
                    let degree = in_degree.get_mut(dependent).unwrap();
                    *degree -= 1;

                    if *degree == 0 {
                        queue.push(dependent.clone());
                        // Re-sort to maintain determinism
                        queue.sort_by_key(|name| nodes.iter().position(|n| n == name).unwrap_or(usize::MAX));
                    }
                }
            }
        }

        // Check for circular dependencies
        if result.len() != in_degree.len() {
            // Find the nodes involved in the cycle
            let unprocessed: Vec<String> = in_degree
                .iter()
                .filter(|(node, _)| !result.contains(node))
                .map(|(node, _)| node.clone())
                .collect();

            return Err(RenderError::Selection(
                crate::renderer::selector::SelectionError::InvalidReference(
                    format!("Circular dependency detected in references: {}", unprocessed.join(", "))
                )
            ));
        }

        Ok(result)
    }

    /// PHASE 2: Enrichment - Apply rules and decisions
    ///
    /// M4: Execute Rules to compute derived values
    /// M4: Rules can read selected values' tags and write to context
    /// M5: Converts Vec<SelectedValue> to single value for rules (uses first)
    /// M8.5 Blocker 2 Phase 1: Execute rules from ALL namespaces (package-wide)
    /// M8.5 Blocker 2 Phase 2: Execute rules from dependencies too (cross-package)
    fn phase_2_enrichment(
        &self,
        context: &mut Context,
        selected: &HashMap<String, Vec<SelectedValue>>,
        _namespace: &crate::core::Namespace  // Kept for compatibility but unused now
    ) -> Result<()> {
        // M5: Convert Vec<SelectedValue> to HashMap<String, SelectedValue> for rules
        // Rules only see the first selected value
        let single_selected: HashMap<String, SelectedValue> = selected
            .iter()
            .filter_map(|(key, values)| {
                values.first().map(|v| (key.clone(), v.clone()))
            })
            .collect();

        println!("\n=== Phase 2: Enrichment ===");

        // M8.5 Blocker 2 Phase 2: Execute rules from dependencies FIRST (if any)
        // This allows main package to override dependency rules if needed
        if let Some(dependencies) = self.dependencies {
            for (dep_id, dep_package) in dependencies {
                for (namespace_id, namespace) in &dep_package.namespaces {
                    if !namespace.rules.is_empty() {
                        println!("Executing {} rule(s) from dependency {}.{}",
                                 namespace.rules.len(), dep_id, namespace_id);

                        let mut rules_processor = RulesProcessor::new(context, &single_selected);
                        rules_processor.execute_rules(&namespace.rules)?;
                    }
                }
            }
        }

        // M8.5 Blocker 2 Phase 1: Execute rules from ALL namespaces in main package
        // This makes rules package-wide instead of namespace-scoped
        for (namespace_id, namespace) in &self.package.namespaces {
            if !namespace.rules.is_empty() {
                println!("Executing {} rule(s) from namespace: {}", namespace.rules.len(), namespace_id);

                // M4: Execute Rules from this namespace
                let mut rules_processor = RulesProcessor::new(context, &single_selected);
                rules_processor.execute_rules(&namespace.rules)?;
            }
        }

        // M4: Later will add Decisions execution here

        Ok(())
    }

    /// PHASE 3: Rendering - Substitute selected values into template
    ///
    /// M4: Can read from context for computed values
    /// M5: Handles Vec<SelectedValue> and formats with separator sets
    fn phase_3_rendering(
        &self,
        template: &str,
        promptsection: &PromptSection,
        selected: &HashMap<String, Vec<SelectedValue>>,
        context: &Context,
        namespace: &crate::core::Namespace,
    ) -> Result<String> {
        let parsed = Template::parse(template)?;

        let mut output = String::new();

        for token in parsed.tokens {
            match token {
                TemplateToken::Text(text) => {
                    output.push_str(&text);
                }
                TemplateToken::Reference { name: ref_name, .. } => {
                    // Try to get from selected values first
                    if let Some(values) = selected.get(&ref_name) {
                        // M5: Get separator from YAML Reference definition
                        let separator_ref = promptsection.references.get(&ref_name)
                            .and_then(|r| r.separator.as_ref());

                        // M5: Format multiple values with separator
                        let text = if values.len() > 1 {
                            // Get separator set if specified
                            if let Some(sep_ref) = separator_ref {
                                // Find separator set in namespace
                                if let Some(sep_set) = namespace.separator_sets.get(sep_ref) {
                                    // Use separator set to format
                                    let texts: Vec<String> = values.iter().map(|v| v.text.clone()).collect();
                                    sep_set.format(&texts)
                                } else {
                                    // Separator not found, fall back to space
                                    values.iter().map(|v| v.text.as_str()).collect::<Vec<_>>().join(" ")
                                }
                            } else {
                                // No separator specified, use space
                                values.iter().map(|v| v.text.as_str()).collect::<Vec<_>>().join(" ")
                            }
                        } else if values.len() == 1 {
                            // Single value, just use it
                            values[0].text.clone()
                        } else {
                            // Empty list, render nothing
                            String::new()
                        };

                        output.push_str(&text);
                    } else {
                        // Try to get from context (for computed values)
                        // Check if it exists in prompt scope
                        if context.has(&ref_name) {
                            let text = context.get_text(&ref_name)?;
                            output.push_str(&text);
                        } else {
                            return Err(RenderError::ReferenceNotSelected(ref_name.clone()));
                        }
                    }
                }
            }
        }

        Ok(output)
    }

    /// Find a promptsection by reference
    /// M9 Phase 3: Searches dependencies if not found in main package
    fn find_promptsection(&self, reference: &str) -> Result<&PromptSection> {
        // Parse reference
        let (namespace, section_name) = if reference.contains(':') {
            let parts: Vec<&str> = reference.split(':').collect();
            (parts[0].to_string(), parts[1].to_string())
        } else {
            // Use first namespace
            let first_ns = self.package.namespaces
                .keys()
                .next()
                .ok_or_else(|| RenderError::PromptSectionNotFound(reference.to_string()))?;
            (first_ns.clone(), reference.to_string())
        };

        // First try main package
        if let Some(ns) = self.package.namespaces.get(&namespace) {
            if let Some(ps) = ns.prompt_sections.get(&section_name) {
                return Ok(ps);
            }
        }

        // M9: If not found and we have dependencies, search them
        if let Some(deps) = self.dependencies {
            for dep_package in deps.values() {
                if let Some(ns) = dep_package.namespaces.get(&namespace) {
                    if let Some(ps) = ns.prompt_sections.get(&section_name) {
                        return Ok(ps);
                    }
                }
            }
        }

        // Not found anywhere
        Err(RenderError::PromptSectionNotFound(reference.to_string()))
    }

    /// Find the namespace that contains a promptsection
    /// M9 Phase 3: Searches dependencies if not found in main package
    fn find_namespace_for_promptsection(&self, reference: &str) -> Result<&crate::core::Namespace> {
        // Parse reference
        let (namespace_id, section_name) = if reference.contains(':') {
            let parts: Vec<&str> = reference.split(':').collect();
            (parts[0].to_string(), parts[1].to_string())
        } else {
            // Use first namespace
            let first_ns = self.package.namespaces
                .keys()
                .next()
                .ok_or_else(|| RenderError::PromptSectionNotFound(reference.to_string()))?;
            (first_ns.clone(), reference.to_string())
        };

        // First try main package
        if let Some(ns) = self.package.namespaces.get(&namespace_id) {
            // Verify the promptsection actually exists in this namespace
            if ns.prompt_sections.contains_key(&section_name) {
                return Ok(ns);
            }
        }

        // M9: If not found and we have dependencies, search them
        if let Some(deps) = self.dependencies {
            for dep_package in deps.values() {
                if let Some(ns) = dep_package.namespaces.get(&namespace_id) {
                    if ns.prompt_sections.contains_key(&section_name) {
                        return Ok(ns);
                    }
                }
            }
        }

        // Not found anywhere
        Err(RenderError::PromptSectionNotFound(reference.to_string()))
    }

    /// M5 Phase 1: Check if a reference points to a promptsection (vs a datatype)
    fn is_promptsection_reference(&self, reference: &str) -> bool {
        // Try to find as promptsection
        self.find_promptsection(reference).is_ok()
    }

    /// M9: Render from a rulebook
    ///
    /// Selects an entry point from the rulebook based on weights,
    /// optionally applies context defaults, then renders the selected promptsection.
    ///
    /// Reference format: "rulebook" or "namespace:rulebook"
    pub fn render_from_rulebook(&self, rulebook_ref: &str) -> Result<RenderResult> {
        self.render_from_rulebook_with_options(rulebook_ref, None)
    }

    /// M9: Render from a rulebook with batch variety tracking
    ///
    /// If used_entry_points is provided and batch_variety is enabled,
    /// will try to avoid using the same entry point twice.
    ///
    /// Context defaults from the rulebook are applied before rendering.
    pub fn render_from_rulebook_with_options(
        &self,
        rulebook_ref: &str,
        used_entry_points: Option<&mut Vec<String>>,
    ) -> Result<RenderResult> {
        // Parse rulebook reference
        let (namespace_name, rulebook_name) = self.parse_rulebook_reference(rulebook_ref)?;

        // Find rulebook
        let rulebook = self.find_rulebook(&namespace_name, &rulebook_name)?;

        // Validate rulebook
        rulebook.validate()
            .map_err(|e| RenderError::Selection(crate::renderer::selector::SelectionError::InvalidReference(e)))?;

        // Select entry point
        let entry_point = self.select_entry_point(rulebook, used_entry_points)?;

        // Prepare context defaults (clone to avoid lifetime issues)
        let context_defaults = if rulebook.context_defaults.is_empty() {
            None
        } else {
            Some(rulebook.context_defaults.clone())
        };

        // Render the selected promptsection with context defaults
        self.render_with_context(&entry_point, context_defaults)
    }

    /// Select an entry point from a rulebook based on weights
    ///
    /// If batch_variety is enabled and used_entry_points is provided,
    /// will try to select an unused entry point.
    fn select_entry_point(
        &self,
        rulebook: &crate::core::rulebook::Rulebook,
        used_entry_points: Option<&mut Vec<String>>,
    ) -> Result<String> {
        use crate::renderer::seeded_random::SeededRandom;

        // If batch variety is enabled and we have tracking, filter out used entry points
        let available_points: Vec<&crate::core::rulebook::EntryPoint> = if let Some(used) = used_entry_points.as_ref() {
            if rulebook.batch_variety {
                rulebook.entry_points
                    .iter()
                    .filter(|ep| !used.contains(&ep.prompt_section))
                    .collect()
            } else {
                rulebook.entry_points.iter().collect()
            }
        } else {
            rulebook.entry_points.iter().collect()
        };

        // If we've used all entry points, reset and use all of them
        let points_to_use: Vec<&crate::core::rulebook::EntryPoint> = if available_points.is_empty() {
            rulebook.entry_points.iter().collect()
        } else {
            available_points
        };

        // Create RNG with the same seed for determinism
        let mut rng = SeededRandom::new(self.seed);

        // Extract weights
        let weights: Vec<f64> = points_to_use.iter().map(|ep| ep.weight).collect();

        // Select entry point using weighted choice
        let index = rng.weighted_choice_f64(&weights);
        let selected = points_to_use[index];

        // Track this entry point if batch variety is enabled
        if let Some(used) = used_entry_points {
            used.push(selected.prompt_section.clone());
        }

        Ok(selected.prompt_section.clone())
    }

    /// Find a rulebook by reference
    fn find_rulebook(&self, namespace: &str, name: &str) -> Result<&crate::core::rulebook::Rulebook> {
        let ns = self.package.namespaces
            .get(namespace)
            .ok_or_else(|| RenderError::PromptSectionNotFound(format!("Namespace not found: {}", namespace)))?;

        ns.rulebooks
            .get(name)
            .ok_or_else(|| RenderError::PromptSectionNotFound(format!("Rulebook not found: {}:{}", namespace, name)))
    }

    /// Parse a rulebook reference into namespace and name
    /// Format: "name" uses first namespace, "namespace:name" is explicit
    fn parse_rulebook_reference(&self, reference: &str) -> Result<(String, String)> {
        if reference.contains(':') {
            let parts: Vec<&str> = reference.split(':').collect();
            if parts.len() != 2 {
                return Err(RenderError::Selection(
                    crate::renderer::selector::SelectionError::InvalidReference(reference.to_string())
                ));
            }
            Ok((parts[0].to_string(), parts[1].to_string()))
        } else {
            // Use first namespace as default
            let first_namespace = self.package.namespaces
                .keys()
                .next()
                .ok_or_else(|| RenderError::Selection(
                    crate::renderer::selector::SelectionError::InvalidReference("No namespaces in package".to_string())
                ))?;

            Ok((first_namespace.clone(), reference.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Namespace, Datatype, DatatypeValue, PackageMetadata, Reference};

    fn create_test_package() -> Package {
        let mut datatypes = HashMap::new();

        // Colors
        datatypes.insert("colors".to_string(), Datatype {
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
        });

        // Objects
        datatypes.insert("objects".to_string(), Datatype {
            name: "objects".to_string(),
            values: vec![
                DatatypeValue {
                    text: "ball".to_string(),
                    tags: HashMap::new(),
                    weight: 1.0,
                },
                DatatypeValue {
                    text: "apple".to_string(),
                    tags: HashMap::new(),
                    weight: 1.0,
                },
            ],
            extends: None,
            override_tags: HashMap::new(),
        });

        let mut prompt_sections = HashMap::new();
        let mut references = HashMap::new();
        references.insert("color".to_string(), Reference {
            target: "test:colors".to_string(),
            filter: None,
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });
        references.insert("object".to_string(), Reference {
            target: "test:objects".to_string(),
            filter: None,
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });

        prompt_sections.insert("simple".to_string(), PromptSection {
            name: "simple".to_string(),
            template: "A {color} {object}".to_string(),
            references,
        });

        let mut namespaces = HashMap::new();
        namespaces.insert("test".to_string(), Namespace {
            id: "test".to_string(),
            datatypes,
            prompt_sections,
            separator_sets: HashMap::new(),
            rules: HashMap::new(),
            decisions: Vec::new(),
            rulebooks: HashMap::new(), // M9: Added rulebooks
        });

        Package {
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
        }
    }

    #[test]
    fn test_render_simple() {
        let package = create_test_package();
        let renderer = Renderer::new(&package, 42);

        let result = renderer.render("simple").unwrap();

        // Should render something like "A red ball" or "A blue apple"
        assert!(result.output.starts_with("A "));
        assert!(result.output.contains("red") || result.output.contains("blue"));
        assert!(result.output.contains("ball") || result.output.contains("apple"));
    }

    #[test]
    fn test_render_deterministic() {
        let package = create_test_package();

        let renderer1 = Renderer::new(&package, 12345);
        let renderer2 = Renderer::new(&package, 12345);

        let result1 = renderer1.render("simple").unwrap();
        let result2 = renderer2.render("simple").unwrap();

        assert_eq!(result1.output, result2.output);
    }

    #[test]
    fn test_render_different_seeds() {
        let package = create_test_package();

        let renderer1 = Renderer::new(&package, 111);
        let renderer2 = Renderer::new(&package, 222);

        let result1 = renderer1.render("simple").unwrap();
        let result2 = renderer2.render("simple").unwrap();

        // Different seeds might produce same output by chance,
        // but check that both are valid
        assert!(result1.output.starts_with("A "));
        assert!(result2.output.starts_with("A "));
    }

    #[test]
    fn test_rulebook_weighted_selection() {
        use crate::core::rulebook::{Rulebook, EntryPoint};

        let mut package = create_test_package();

        // Add a rulebook to the test namespace
        let rulebook = Rulebook {
            name: "test_rulebook".to_string(),
            description: "Test Rulebook".to_string(),
            entry_points: vec![
                EntryPoint {
                    prompt_section: "test:simple".to_string(),
                    weight: 1.0,
                },
            ],
            batch_variety: false,
            context_defaults: HashMap::new(),
        };

        package.namespaces
            .get_mut("test")
            .unwrap()
            .rulebooks
            .insert("test_rulebook".to_string(), rulebook);

        let renderer = Renderer::new(&package, 42);
        let result = renderer.render_from_rulebook("test_rulebook").unwrap();

        // Should successfully render from the entry point
        assert!(result.output.starts_with("A "));
    }

    #[test]
    fn test_rulebook_batch_variety() {
        use crate::core::rulebook::{Rulebook, EntryPoint};

        let mut package = create_test_package();

        // Add another promptsection
        let mut refs2 = HashMap::new();
        refs2.insert("color".to_string(), Reference {
            target: "test:colors".to_string(),
            filter: None,
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });

        package.namespaces
            .get_mut("test")
            .unwrap()
            .prompt_sections
            .insert("simple2".to_string(), PromptSection {
                name: "simple2".to_string(),
                template: "{color} thing".to_string(),
                references: refs2,
            });

        // Add a rulebook with multiple entry points
        let rulebook = Rulebook {
            name: "variety_test".to_string(),
            description: String::new(),
            entry_points: vec![
                EntryPoint {
                    prompt_section: "test:simple".to_string(),
                    weight: 1.0,
                },
                EntryPoint {
                    prompt_section: "test:simple2".to_string(),
                    weight: 1.0,
                },
            ],
            batch_variety: true,
            context_defaults: HashMap::new(),
        };

        package.namespaces
            .get_mut("test")
            .unwrap()
            .rulebooks
            .insert("variety_test".to_string(), rulebook);

        let renderer = Renderer::new(&package, 42);

        // Test batch variety - should select different entry points
        let mut used = Vec::new();
        let _result1 = renderer.render_from_rulebook_with_options("variety_test", Some(&mut used)).unwrap();
        let _result2 = renderer.render_from_rulebook_with_options("variety_test", Some(&mut used)).unwrap();

        // Should have tracked two entries
        assert_eq!(used.len(), 2);
    }

    #[test]
    fn test_rulebook_context_defaults() {
        use crate::core::rulebook::{Rulebook, EntryPoint};

        let mut package = create_test_package();

        // Add a promptsection that uses context values
        // Template references context via rules
        let mut refs = HashMap::new();
        refs.insert("item".to_string(), Reference {
            target: "test:objects".to_string(),
            filter: None,
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });

        package.namespaces
            .get_mut("test")
            .unwrap()
            .prompt_sections
            .insert("with_context".to_string(), PromptSection {
                name: "with_context".to_string(),
                template: "{item}".to_string(),
                references: refs,
            });

        // Add a rulebook with context defaults
        let mut context_defaults = HashMap::new();
        context_defaults.insert("test_key".to_string(), "test_value".to_string());
        context_defaults.insert("number".to_string(), "42".to_string());

        let rulebook = Rulebook {
            name: "context_test".to_string(),
            description: "Test context defaults".to_string(),
            entry_points: vec![
                EntryPoint {
                    prompt_section: "test:with_context".to_string(),
                    weight: 1.0,
                },
            ],
            batch_variety: false,
            context_defaults,
        };

        package.namespaces
            .get_mut("test")
            .unwrap()
            .rulebooks
            .insert("context_test".to_string(), rulebook);

        let renderer = Renderer::new(&package, 42);
        let result = renderer.render_from_rulebook("context_test").unwrap();

        // Should successfully render
        assert!(!result.output.is_empty());

        // Context values should be in the selected_values
        assert!(result.selected_values.contains_key("context:test_key"));
        assert_eq!(result.selected_values.get("context:test_key").unwrap(), "test_value");
        assert!(result.selected_values.contains_key("context:number"));
        assert_eq!(result.selected_values.get("context:number").unwrap(), "42");
    }

    #[test]
    fn test_rulebook_context_with_scope() {
        use crate::core::rulebook::{Rulebook, EntryPoint};

        let mut package = create_test_package();

        // Add a promptsection
        let mut refs = HashMap::new();
        refs.insert("color".to_string(), Reference {
            target: "test:colors".to_string(),
            filter: None,
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });

        package.namespaces
            .get_mut("test")
            .unwrap()
            .prompt_sections
            .insert("scoped".to_string(), PromptSection {
                name: "scoped".to_string(),
                template: "{color} item".to_string(),
                references: refs,
            });

        // Add a rulebook with scoped context defaults
        let mut context_defaults = HashMap::new();
        context_defaults.insert("prompt:style".to_string(), "fantasy".to_string());
        context_defaults.insert("global:world".to_string(), "middle_earth".to_string());

        let rulebook = Rulebook {
            name: "scoped_test".to_string(),
            description: "Test scoped context".to_string(),
            entry_points: vec![
                EntryPoint {
                    prompt_section: "test:scoped".to_string(),
                    weight: 1.0,
                },
            ],
            batch_variety: false,
            context_defaults,
        };

        package.namespaces
            .get_mut("test")
            .unwrap()
            .rulebooks
            .insert("scoped_test".to_string(), rulebook);

        let renderer = Renderer::new(&package, 42);
        let result = renderer.render_from_rulebook("scoped_test").unwrap();

        // Should successfully render
        assert!(!result.output.is_empty());

        // Both prompt and global scope values should be present
        assert!(result.selected_values.contains_key("context:style"));
        assert_eq!(result.selected_values.get("context:style").unwrap(), "fantasy");
        assert!(result.selected_values.contains_key("context:world"));
        assert_eq!(result.selected_values.get("context:world").unwrap(), "middle_earth");
    }
}


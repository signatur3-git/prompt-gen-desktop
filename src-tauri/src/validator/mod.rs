// M6 Phase 1: Package Validator
// Comprehensive validation with helpful error messages

use crate::core::models::Package;
use std::collections::HashMap;
use thiserror::Error;

#[cfg(test)]
mod integration_tests;

/// Validation error types
#[derive(Error, Debug, Clone)]
pub enum ValidationError {
    #[error("Reference not found: '{reference}' in {defined_in}")]
    ReferenceNotFound {
        reference: String,
        defined_in: String,
        suggestion: Option<String>,
    },

    #[error("Circular reference detected: {chain}")]
    CircularReference { chain: String },

    #[error("Invalid tag filter: {expression} - {reason}")]
    InvalidTagFilter { expression: String, reason: String },

    #[error("Separator set not found: '{separator}' referenced in {defined_in}")]
    SeparatorNotFound {
        separator: String,
        defined_in: String,
    },

    #[error("Min must be <= Max: min={min}, max={max} in {defined_in}")]
    MinMaxInvalid {
        min: usize,
        max: usize,
        defined_in: String,
    },

    #[error("Unique constraint infeasible: requested {requested} unique values but only {available} values available in {datatype}")]
    UniqueConstraintInfeasible {
        requested: usize,
        available: usize,
        datatype: String,
    },

    #[allow(dead_code)] // Reserved for future validation
    #[error("Invalid rule: {reason} in rule '{rule_name}'")]
    InvalidRule { rule_name: String, reason: String },

    #[allow(dead_code)] // Reserved for future validation
    #[error("Duplicate ID: '{id}' defined multiple times in namespace '{namespace}'")]
    DuplicateId { id: String, namespace: String },

    #[error("Invalid naming: '{name}' - {reason}")]
    InvalidNaming { name: String, reason: String },

    // M9 Phase 2.4: Dependency validation errors
    #[error("Invalid dependency: package '{package}' - {reason}")]
    InvalidDependency { package: String, reason: String },

    #[error("Invalid version format: '{version}' in dependency '{package}' - {reason}")]
    InvalidDependencyVersion {
        package: String,
        version: String,
        reason: String,
    },
}

/// Validation warning types
#[derive(Debug, Clone)]
pub enum ValidationWarning {
    UnusedDatatype {
        datatype: String,
        namespace: String,
    },
    #[allow(dead_code)] // Reserved for future validation
    UnusedPromptSection {
        promptsection: String,
        namespace: String,
    },
    UnusedSeparatorSet {
        separator: String,
        namespace: String,
    },
    UnusedReference {
        reference: String,
        promptsection: String,
    },
    LargeWeightSum {
        datatype: String,
        sum: f32,
    },
    #[allow(dead_code)] // Reserved for future validation
    MissingDescription {
        component: String,
    },
    // M9 Phase 3: Dependency warnings
    MajorVersionRange {
        package: String,
        version: String,
        suggestion: String,
    },
    FlexibleDependency {
        package: String,
        version: String,
        info: String,
    },
}

impl std::fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationWarning::UnusedDatatype {
                datatype,
                namespace,
            } => write!(
                f,
                "Unused datatype: '{}:{}' is defined but never referenced",
                namespace, datatype
            ),
            ValidationWarning::UnusedPromptSection {
                promptsection,
                namespace,
            } => write!(
                f,
                "Unused promptsection: '{}:{}' is defined but never referenced",
                namespace, promptsection
            ),
            ValidationWarning::UnusedSeparatorSet {
                separator,
                namespace,
            } => write!(
                f,
                "Unused separator set: '{}:{}' is defined but never referenced",
                namespace, separator
            ),
            ValidationWarning::UnusedReference {
                reference,
                promptsection,
            } => write!(
                f,
                "Unused reference '{}' in {}: defined but not used in template (consider adding {{{}}} or removing the definition)",
                reference, promptsection, reference
            ),
            ValidationWarning::LargeWeightSum { datatype, sum } => write!(
                f,
                "Large weight sum in datatype '{}': {:.2} (consider normalizing)",
                datatype, sum
            ),
            ValidationWarning::MissingDescription { component } => {
                write!(f, "Missing description for '{}'", component)
            }
            ValidationWarning::MajorVersionRange {
                package,
                version,
                suggestion,
            } => write!(
                f,
                "Dependency '{}' uses major version range ({}). {}",
                package, version, suggestion
            ),
            ValidationWarning::FlexibleDependency {
                package,
                version,
                info,
            } => write!(
                f,
                "Dependency '{}' uses flexible version ({}). {}",
                package, version, info
            ),
        }
    }
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationResult {
    pub fn new() -> Self {
        ValidationResult {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    #[allow(dead_code)] // Public API method
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }

    #[allow(dead_code)] // Public API method for combining validation results
    pub fn merge(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Package validator
pub struct PackageValidator;

impl PackageValidator {
    /// Validate a package (without dependencies)
    #[allow(dead_code)] // Part of public API, used by external callers
    pub fn validate(package: &Package) -> ValidationResult {
        Self::validate_with_dependencies(package, &HashMap::new())
    }

    /// M9 Phase 2.7: Validate a package with its dependencies
    /// This allows validation of cross-package references
    pub fn validate_with_dependencies(
        package: &Package,
        dependencies: &HashMap<String, Package>,
    ) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Schema validation (already done by serde, but we can add more)
        Self::validate_schema(package, &mut result);

        // Semantic validation (with dependencies context)
        Self::validate_semantics_with_deps(package, dependencies, &mut result);

        // Best practices
        Self::validate_best_practices(package, &mut result);

        result
    }

    /// Schema validation - structure and types
    fn validate_schema(_package: &Package, _result: &mut ValidationResult) {
        // Most schema validation is already done by serde
        // We can add additional checks here if needed
    }

    /// M9 Phase 2.7: Semantic validation with dependencies
    fn validate_semantics_with_deps(
        package: &Package,
        dependencies: &HashMap<String, Package>,
        result: &mut ValidationResult,
    ) {
        // M9 Phase 2.4: Validate dependencies first
        Self::validate_dependencies(package, result);

        // Validate all references resolve (with dependencies)
        Self::validate_references_with_deps(package, dependencies, result);

        // Validate that template references are defined
        Self::validate_template_references(package, result);

        // Check for circular references in nested promptsections
        Self::validate_no_circular_references(package, result);

        // Validate tag filters
        Self::validate_tag_filters(package, result);

        // Validate separator sets exist
        Self::validate_separator_sets(package, result);

        // Validate min/max constraints
        Self::validate_min_max(package, result);

        // Validate unique constraints are feasible
        Self::validate_unique_constraints(package, result);

        // Validate rules
        Self::validate_rules(package, result);

        // M9 Task 1.4: Validate rulebooks
        Self::validate_rulebooks(package, result);
    }

    /// Best practices validation - warnings for improvement
    fn validate_best_practices(package: &Package, result: &mut ValidationResult) {
        // Check for unused components
        Self::check_unused_components(package, result);

        // Check naming conventions
        Self::check_naming_conventions(package, result);

        // Check weight sums
        Self::check_weight_sums(package, result);
    }

    // M9 Phase 2.7: Validate all references resolve (with dependencies support)
    fn validate_references_with_deps(
        package: &Package,
        dependencies: &HashMap<String, Package>,
        result: &mut ValidationResult,
    ) {
        for (ns_id, namespace) in &package.namespaces {
            // Check all promptsection references
            for (ps_name, promptsection) in &namespace.prompt_sections {
                for (ref_name, reference) in &promptsection.references {
                    // Skip empty targets (user is still editing)
                    if reference.target.is_empty() {
                        continue;
                    }

                    // Skip context references (they're special)
                    if reference.target.starts_with("context:") {
                        continue;
                    }

                    // Check for self-reference (reference name same as target without namespace)
                    // This is usually a mistake: reference "colors" with target "colors"
                    if !reference.target.contains(':') && reference.target == *ref_name {
                        // Check if there's actually a datatype/promptsection with this name
                        let has_matching_component =
                            namespace.datatypes.contains_key(&reference.target)
                                || namespace.prompt_sections.contains_key(&reference.target);

                        if !has_matching_component {
                            result.add_error(ValidationError::ReferenceNotFound {
                                reference: reference.target.clone(),
                                defined_in: format!("{}:{}", ns_id, ps_name),
                                suggestion: Some(format!(
                                    "Did you mean '{}:{}' (with namespace prefix)?",
                                    ns_id, reference.target
                                )),
                            });
                            continue;
                        }
                    }

                    // Parse the target (format: "namespace:name" or just "name")
                    let (target_ns, target_name) = if reference.target.contains(':') {
                        let parts: Vec<&str> = reference.target.split(':').collect();
                        (parts[0].to_string(), parts[1].to_string())
                    } else {
                        // Same namespace
                        (ns_id.clone(), reference.target.clone())
                    };

                    // Check if it exists in the main package
                    let found_in_package =
                        if let Some(target_namespace) = package.namespaces.get(&target_ns) {
                            target_namespace.datatypes.contains_key(&target_name)
                                || target_namespace.prompt_sections.contains_key(&target_name)
                        } else {
                            false
                        };

                    // M9: If not found in main package, check dependencies
                    let found = if found_in_package {
                        true
                    } else {
                        Self::find_in_dependencies(dependencies, &target_ns, &target_name)
                    };

                    if !found {
                        // Try to suggest alternatives
                        let suggestion = Self::find_similar_name_with_deps(
                            package,
                            dependencies,
                            &target_ns,
                            &target_name,
                        );

                        result.add_error(ValidationError::ReferenceNotFound {
                            reference: reference.target.clone(),
                            defined_in: format!("{}:{}", ns_id, ps_name),
                            suggestion,
                        });
                    }
                }
            }
        }
    }

    /// M9 Phase 2.7: Check if a component exists in any dependency
    fn find_in_dependencies(
        dependencies: &HashMap<String, Package>,
        target_ns: &str,
        target_name: &str,
    ) -> bool {
        for dep_package in dependencies.values() {
            if let Some(target_namespace) = dep_package.namespaces.get(target_ns) {
                if target_namespace.datatypes.contains_key(target_name)
                    || target_namespace.prompt_sections.contains_key(target_name)
                {
                    return true;
                }
            }
        }
        false
    }

    /// M9 Phase 2.7: Find similar names including dependencies
    fn find_similar_name_with_deps(
        package: &Package,
        dependencies: &HashMap<String, Package>,
        target_ns: &str,
        target_name: &str,
    ) -> Option<String> {
        // First try in main package
        if let Some(suggestion) = Self::find_similar_name(package, target_ns, target_name) {
            return Some(suggestion);
        }

        // Then try in dependencies
        for dep_package in dependencies.values() {
            if let Some(suggestion) = Self::find_similar_name(dep_package, target_ns, target_name) {
                return Some(format!(
                    "{} (from dependency {})",
                    suggestion, dep_package.id
                ));
            }
        }

        None
    }

    // Validate that references used in templates are defined in the promptsection
    fn validate_template_references(package: &Package, result: &mut ValidationResult) {
        for (ns_id, namespace) in &package.namespaces {
            for (ps_name, promptsection) in &namespace.prompt_sections {
                // Extract references from template
                let template_refs = Self::extract_template_references(&promptsection.template);

                // Check each reference in template is defined (ERROR if missing)
                for ref_name in &template_refs {
                    if !promptsection.references.contains_key(ref_name) {
                        result.add_error(ValidationError::ReferenceNotFound {
                            reference: ref_name.clone(),
                            defined_in: format!("{}:{}", ns_id, ps_name),
                            suggestion: Some(format!(
                                "Add reference definition for '{}' in the references section",
                                ref_name
                            )),
                        });
                    }
                }

                // Check for unused references (WARNING if defined but not in template)
                for ref_name in promptsection.references.keys() {
                    if !template_refs.contains(ref_name) {
                        result.add_warning(ValidationWarning::UnusedReference {
                            reference: ref_name.clone(),
                            promptsection: format!("{}:{}", ns_id, ps_name),
                        });
                    }
                }
            }
        }
    }

    // Extract reference names from template (finds {ref_name} patterns)
    fn extract_template_references(template: &str) -> Vec<String> {
        let mut refs = Vec::new();
        let mut chars = template.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                let mut ref_name = String::new();

                // Collect characters until }
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '}' {
                        chars.next(); // consume }
                        break;
                    }
                    ref_name.push(chars.next().unwrap());
                }

                if !ref_name.is_empty() && !ref_name.starts_with("context.") {
                    refs.push(ref_name);
                }
            }
        }

        refs
    }

    // Find similar names for helpful suggestions
    fn find_similar_name(package: &Package, namespace: &str, target: &str) -> Option<String> {
        if let Some(ns) = package.namespaces.get(namespace) {
            // Check datatypes
            for dt_name in ns.datatypes.keys() {
                if Self::is_similar(dt_name, target) {
                    return Some(format!("{}:{} (datatype)", namespace, dt_name));
                }
            }
            // Check promptsections
            for ps_name in ns.prompt_sections.keys() {
                if Self::is_similar(ps_name, target) {
                    return Some(format!("{}:{} (promptsection)", namespace, ps_name));
                }
            }
        }
        None
    }

    // Simple similarity check (could use Levenshtein distance for better results)
    fn is_similar(a: &str, b: &str) -> bool {
        let a_lower = a.to_lowercase();
        let b_lower = b.to_lowercase();

        // Check if one starts with the other
        if a_lower.starts_with(&b_lower) || b_lower.starts_with(&a_lower) {
            return true;
        }

        // Check if one contains the other
        if a_lower.contains(&b_lower) || b_lower.contains(&a_lower) {
            return true;
        }

        // Check if they start the same (at least 3 chars)
        if a_lower.len() >= 3 && b_lower.len() >= 3 && a_lower[..3] == b_lower[..3] {
            return true;
        }

        false
    }

    // M9: Find similar promptsection names for rulebook validation
    fn find_similar_promptsection(
        package: &Package,
        namespace: &str,
        target: &str,
    ) -> Option<String> {
        if let Some(ns) = package.namespaces.get(namespace) {
            for ps_name in ns.prompt_sections.keys() {
                if Self::is_similar(ps_name, target) {
                    return Some(format!("{}:{}", namespace, ps_name));
                }
            }
        }
        None
    }

    // Check for circular references in nested promptsections
    fn validate_no_circular_references(package: &Package, result: &mut ValidationResult) {
        for (ns_id, namespace) in &package.namespaces {
            for ps_name in namespace.prompt_sections.keys() {
                let full_name = format!("{}:{}", ns_id, ps_name);
                let mut visited = Vec::new();

                if let Some(chain) = Self::find_circular_ref(package, &full_name, &mut visited) {
                    result.add_error(ValidationError::CircularReference {
                        chain: chain.join(" â†’ "),
                    });
                }
            }
        }
    }

    // Recursively check for circular references
    fn find_circular_ref(
        package: &Package,
        current: &str,
        visited: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        // If we've seen this before, we have a cycle
        if visited.contains(&current.to_string()) {
            let mut chain = visited.clone();
            chain.push(current.to_string());
            return Some(chain);
        }

        // Add to visited
        visited.push(current.to_string());

        // Parse current (format: "namespace:name")
        let parts: Vec<&str> = current.split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        let (ns_id, ps_name) = (parts[0], parts[1]);

        // Get the promptsection
        let namespace = package.namespaces.get(ns_id)?;
        let promptsection = namespace.prompt_sections.get(ps_name)?;

        // Check all references
        for reference in promptsection.references.values() {
            // Skip context references
            if reference.target.starts_with("context:") {
                continue;
            }

            // Parse the target
            let target = if reference.target.contains(':') {
                reference.target.clone()
            } else {
                format!("{}:{}", ns_id, reference.target)
            };

            // Check if this target is a promptsection
            let target_parts: Vec<&str> = target.split(':').collect();
            if target_parts.len() == 2 {
                let (target_ns, target_name) = (target_parts[0], target_parts[1]);
                if let Some(target_namespace) = package.namespaces.get(target_ns) {
                    if target_namespace.prompt_sections.contains_key(target_name) {
                        // Recursively check
                        if let Some(chain) = Self::find_circular_ref(package, &target, visited) {
                            return Some(chain);
                        }
                    }
                }
            }
        }

        // Remove from visited (backtrack)
        visited.pop();
        None
    }

    // Validate tag filters are parseable
    fn validate_tag_filters(package: &Package, result: &mut ValidationResult) {
        use crate::renderer::tag_expression::ExpressionParser;

        for namespace in package.namespaces.values() {
            for promptsection in namespace.prompt_sections.values() {
                for reference in promptsection.references.values() {
                    if let Some(filter) = &reference.filter {
                        // Try to parse the expression
                        match ExpressionParser::parse(filter) {
                            Ok(_) => {
                                // Valid expression
                            }
                            Err(e) => {
                                result.add_error(ValidationError::InvalidTagFilter {
                                    expression: filter.clone(),
                                    reason: format!("{}", e),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // Validate separator sets exist
    fn validate_separator_sets(package: &Package, result: &mut ValidationResult) {
        for (ns_id, namespace) in &package.namespaces {
            for (ps_name, promptsection) in &namespace.prompt_sections {
                for (ref_name, reference) in &promptsection.references {
                    if let Some(sep_ref) = &reference.separator {
                        // Check if separator set exists
                        let found = namespace.separator_sets.contains_key(sep_ref);

                        if !found {
                            result.add_error(ValidationError::SeparatorNotFound {
                                separator: sep_ref.clone(),
                                defined_in: format!("{}:{}.{}", ns_id, ps_name, ref_name),
                            });
                        }
                    }
                }
            }
        }
    }

    // Validate min <= max
    fn validate_min_max(package: &Package, result: &mut ValidationResult) {
        for (ns_id, namespace) in &package.namespaces {
            for (ps_name, promptsection) in &namespace.prompt_sections {
                for (ref_name, reference) in &promptsection.references {
                    if reference.min > reference.max {
                        result.add_error(ValidationError::MinMaxInvalid {
                            min: reference.min,
                            max: reference.max,
                            defined_in: format!("{}:{}.{}", ns_id, ps_name, ref_name),
                        });
                    }
                }
            }
        }
    }

    // Validate unique constraints are feasible
    fn validate_unique_constraints(package: &Package, result: &mut ValidationResult) {
        for (ns_id, namespace) in &package.namespaces {
            for promptsection in namespace.prompt_sections.values() {
                for reference in promptsection.references.values() {
                    // Only check if unique is true and max > 1
                    if reference.unique && reference.max > 1 {
                        // Find the target datatype
                        let (target_ns, target_name) = if reference.target.contains(':') {
                            let parts: Vec<&str> = reference.target.split(':').collect();
                            (parts[0].to_string(), parts[1].to_string())
                        } else {
                            (ns_id.clone(), reference.target.clone())
                        };

                        // Get the datatype
                        if let Some(target_namespace) = package.namespaces.get(&target_ns) {
                            if let Some(datatype) = target_namespace.datatypes.get(&target_name) {
                                let available = datatype.values.len();

                                if available < reference.max {
                                    result.add_error(ValidationError::UniqueConstraintInfeasible {
                                        requested: reference.max,
                                        available,
                                        datatype: reference.target.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Validate rules (M4)
    fn validate_rules(_package: &Package, _result: &mut ValidationResult) {
        // Rule validation is complex and would check:
        // - Condition expressions are valid
        // - Actions reference valid context keys
        // - Rule dependencies don't create cycles
        // TODO: Implement comprehensive rule validation
    }

    // M9 Task 1.4: Validate rulebooks
    fn validate_rulebooks(package: &Package, result: &mut ValidationResult) {
        for (ns_id, namespace) in &package.namespaces {
            for (rulebook_name, rulebook) in &namespace.rulebooks {
                // Validate rulebook structure using its built-in validation
                if let Err(e) = rulebook.validate() {
                    result.add_error(ValidationError::InvalidNaming {
                        name: format!("{}:{}", ns_id, rulebook_name),
                        reason: e,
                    });
                    continue; // Skip further validation if structure is invalid
                }

                // Validate entry points reference existing promptsections
                for (idx, entry_point) in rulebook.entry_points.iter().enumerate() {
                    // Parse the prompt_section reference
                    let (target_ns, target_name) = if entry_point.prompt_section.contains(':') {
                        let parts: Vec<&str> = entry_point.prompt_section.split(':').collect();
                        if parts.len() != 2 {
                            result.add_error(ValidationError::ReferenceNotFound {
                                reference: entry_point.prompt_section.clone(),
                                defined_in: format!(
                                    "{}:{} entry_point[{}]",
                                    ns_id, rulebook_name, idx
                                ),
                                suggestion: Some("Format should be 'namespace:name'".to_string()),
                            });
                            continue;
                        }
                        (parts[0].to_string(), parts[1].to_string())
                    } else {
                        // Same namespace
                        (ns_id.clone(), entry_point.prompt_section.clone())
                    };

                    // Check if the promptsection exists
                    let found = if let Some(target_namespace) = package.namespaces.get(&target_ns) {
                        target_namespace.prompt_sections.contains_key(&target_name)
                    } else {
                        false
                    };

                    if !found {
                        result.add_error(ValidationError::ReferenceNotFound {
                            reference: entry_point.prompt_section.clone(),
                            defined_in: format!("{}:{} entry_point[{}]", ns_id, rulebook_name, idx),
                            suggestion: Self::find_similar_promptsection(
                                package,
                                &target_ns,
                                &target_name,
                            ),
                        });
                    }
                }

                // Validate context_defaults have valid key formats
                for key in rulebook.context_defaults.keys() {
                    // Keys can be "key" or "scope:key"
                    if key.is_empty() {
                        result.add_error(ValidationError::InvalidNaming {
                            name: format!("{}:{}", ns_id, rulebook_name),
                            reason: "Context key cannot be empty".to_string(),
                        });
                    } else if key.contains(':') {
                        // Validate scope:key format
                        let parts: Vec<&str> = key.split(':').collect();
                        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
                            result.add_error(ValidationError::InvalidNaming {
                                name: format!("{}:{} context key '{}'", ns_id, rulebook_name, key),
                                reason: "Context keys with ':' must be in format 'scope:key'"
                                    .to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    // M9 Phase 2.4: Validate package dependencies
    // M9 Phase 3: Updated to accept semver ranges (user-controlled determinism)
    fn validate_dependencies(package: &Package, result: &mut ValidationResult) {
        for dep in &package.dependencies {
            // Validate package ID format
            if dep.package.is_empty() {
                result.add_error(ValidationError::InvalidDependency {
                    package: dep.package.clone(),
                    reason: "Package ID cannot be empty".to_string(),
                });
                continue;
            }

            // Validate package ID doesn't reference itself
            if dep.package == package.id {
                result.add_error(ValidationError::InvalidDependency {
                    package: dep.package.clone(),
                    reason: "Package cannot depend on itself".to_string(),
                });
            }

            // M9 Phase 3: Validate semver format (accepts flexible or exact)
            if !Self::is_valid_semver(&dep.version) {
                result.add_error(ValidationError::InvalidDependencyVersion {
                    package: dep.package.clone(),
                    version: dep.version.clone(),
                    reason: "Invalid semver format. Examples: 1.0.0, ^1.0.0, ~1.2.0, >=1.0.0"
                        .to_string(),
                });
                continue;
            }

            // M9 Phase 3: Warn about major version ranges (potentially breaking)
            if Self::is_major_version_range(&dep.version) {
                result.add_warning(ValidationWarning::MajorVersionRange {
                    package: dep.package.clone(),
                    version: dep.version.clone(),
                    suggestion: "This may introduce breaking changes. Consider using tilde (~) for safer patch updates.".to_string(),
                });
            }

            // M9 Phase 3: Info about flexible versions
            if Self::is_flexible_version(&dep.version) {
                result.add_warning(ValidationWarning::FlexibleDependency {
                    package: dep.package.clone(),
                    version: dep.version.clone(),
                    info: "Latest matching version will be used during development. Users can pin to exact versions for deterministic output.".to_string(),
                });
            }

            // Validate path exists if specified (only check format, not actual file existence)
            if let Some(path) = &dep.path {
                if path.is_empty() {
                    result.add_error(ValidationError::InvalidDependency {
                        package: dep.package.clone(),
                        reason: "Path cannot be empty string (omit field instead)".to_string(),
                    });
                }
            }
        }

        // Check for duplicate dependencies
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        for dep in &package.dependencies {
            if !seen.insert(&dep.package) {
                result.add_error(ValidationError::InvalidDependency {
                    package: dep.package.clone(),
                    reason: "Duplicate dependency declaration".to_string(),
                });
            }
        }
    }

    // M9 Phase 3: Helper functions for semver validation

    /// Check if version string is valid semver (exact or with prefix)
    fn is_valid_semver(version: &str) -> bool {
        // Pattern: optional prefix (^, ~, >=, etc.) + three numbers separated by dots
        // Simpler regex without the regex crate
        let parts: Vec<&str> = version
            .trim_start_matches(['^', '~', '>', '<', '='])
            .split('.')
            .collect();

        if parts.len() != 3 {
            return false;
        }

        // Check each part is a valid number
        parts.iter().all(|p| p.parse::<u32>().is_ok())
    }

    /// Check if version range allows major version changes
    fn is_major_version_range(version: &str) -> bool {
        // Caret (^) and >= allow major version changes
        version.starts_with('^') || version.starts_with(">=")
    }

    /// Check if version is flexible (not exact)
    fn is_flexible_version(version: &str) -> bool {
        version.contains('^')
            || version.contains('~')
            || version.contains('>')
            || version.contains('<')
    }

    // Check for unused components
    fn check_unused_components(package: &Package, result: &mut ValidationResult) {
        use std::collections::HashSet;

        // M8.5 Blocker 3: Track usage across ALL namespaces, not just within same namespace
        // A datatype used from ANY namespace should not be marked as unused
        let mut used_datatypes: HashMap<String, HashSet<String>> = HashMap::new(); // ns_id -> Set<dt_name>
        let mut used_promptsections: HashMap<String, HashSet<String>> = HashMap::new();
        let mut used_separators: HashMap<String, HashSet<String>> = HashMap::new();

        // First pass: Collect ALL usage from ALL namespaces
        for (_source_ns_id, source_namespace) in &package.namespaces {
            for promptsection in source_namespace.prompt_sections.values() {
                for reference in promptsection.references.values() {
                    // Skip context references
                    if reference.target.starts_with("context:") {
                        continue;
                    }

                    // Parse target (format: "namespace:name" or just "name")
                    let (target_ns, target_name) = if reference.target.contains(':') {
                        let parts: Vec<&str> = reference.target.split(':').collect();
                        (parts[0].to_string(), parts[1].to_string())
                    } else {
                        // Same namespace as the promptsection
                        (_source_ns_id.clone(), reference.target.clone())
                    };

                    // Mark as used in the target namespace (cross-namespace usage counts!)
                    if let Some(target_namespace) = package.namespaces.get(&target_ns) {
                        if target_namespace.datatypes.contains_key(&target_name) {
                            used_datatypes
                                .entry(target_ns.clone())
                                .or_default()
                                .insert(target_name);
                        } else if target_namespace.prompt_sections.contains_key(&target_name) {
                            used_promptsections
                                .entry(target_ns.clone())
                                .or_default()
                                .insert(target_name);
                        }
                    }

                    // Check separator usage (separators are same-namespace only)
                    if let Some(sep) = &reference.separator {
                        used_separators
                            .entry(_source_ns_id.clone())
                            .or_default()
                            .insert(sep.clone());
                    }
                }
            }
        }

        // Second pass: Warn about unused components in each namespace
        for (ns_id, namespace) in &package.namespaces {
            let ns_used_datatypes = used_datatypes.get(ns_id).cloned().unwrap_or_default();
            let ns_used_promptsections =
                used_promptsections.get(ns_id).cloned().unwrap_or_default();
            let ns_used_separators = used_separators.get(ns_id).cloned().unwrap_or_default();

            // Warn about unused datatypes
            for dt_name in namespace.datatypes.keys() {
                if !ns_used_datatypes.contains(dt_name) {
                    result.add_warning(ValidationWarning::UnusedDatatype {
                        datatype: dt_name.clone(),
                        namespace: ns_id.clone(),
                    });
                }
            }

            // Warn about unused promptsections (except if they might be entry points)
            // TODO: Better heuristic for entry points
            for ps_name in namespace.prompt_sections.keys() {
                if !ns_used_promptsections.contains(ps_name) {
                    // Don't warn - could be an entry point
                }
            }

            // Warn about unused separator sets
            for sep_name in namespace.separator_sets.keys() {
                if !ns_used_separators.contains(sep_name) {
                    result.add_warning(ValidationWarning::UnusedSeparatorSet {
                        separator: sep_name.clone(),
                        namespace: ns_id.clone(),
                    });
                }
            }
        }
    }

    // Check naming conventions
    fn check_naming_conventions(package: &Package, result: &mut ValidationResult) {
        for (ns_id, namespace) in &package.namespaces {
            // Check namespace ID
            if !Self::is_valid_name(ns_id) {
                result.add_error(ValidationError::InvalidNaming {
                    name: ns_id.clone(),
                    reason: "Namespace IDs should be lowercase alphanumeric with hyphens, underscores, or dots".to_string(),
                });
            }

            // Check datatype names
            for dt_name in namespace.datatypes.keys() {
                if !Self::is_valid_name(dt_name) {
                    result.add_error(ValidationError::InvalidNaming {
                        name: dt_name.clone(),
                        reason: "Datatype names should be lowercase alphanumeric with hyphens, underscores, or dots".to_string(),
                    });
                }
            }

            // Check promptsection names
            for ps_name in namespace.prompt_sections.keys() {
                if !Self::is_valid_name(ps_name) {
                    result.add_error(ValidationError::InvalidNaming {
                        name: ps_name.clone(),
                        reason: "PromptSection names should be lowercase alphanumeric with hyphens, underscores, or dots".to_string(),
                    });
                }
            }
        }
    }

    fn is_valid_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        // Should start with a letter
        if !name.chars().next().unwrap().is_ascii_lowercase() {
            return false;
        }

        // Should only contain lowercase letters, numbers, hyphens, underscores, and dots
        // Dots allow for hierarchical namespacing (e.g., common.fantasy, common.visual)
        name.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_' || c == '.'
        })
    }

    // Check weight sums
    fn check_weight_sums(package: &Package, result: &mut ValidationResult) {
        for (ns_id, namespace) in &package.namespaces {
            for (dt_name, datatype) in &namespace.datatypes {
                let sum: f32 = datatype.values.iter().map(|v| v.weight).sum();

                // Warn if sum is very large (might cause issues)
                if sum > 1000.0 {
                    result.add_warning(ValidationWarning::LargeWeightSum {
                        datatype: format!("{}:{}", ns_id, dt_name),
                        sum,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::*;
    use std::collections::HashMap;

    fn create_test_package() -> Package {
        let mut namespaces = HashMap::new();

        let mut namespace = Namespace {
            id: "test".to_string(),
            datatypes: HashMap::new(),
            prompt_sections: HashMap::new(),
            separator_sets: HashMap::new(),
            rules: HashMap::new(), // Changed from Vec to HashMap
            decisions: Vec::new(),
            rulebooks: HashMap::new(), // M9
        };

        // Add a datatype
        let datatype = Datatype {
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
        };
        namespace.datatypes.insert("colors".to_string(), datatype);

        // Add a separator set
        let separator = SeparatorSet {
            name: "comma_and".to_string(),
            primary: ", ".to_string(),
            secondary: " and ".to_string(),
            tertiary: None,
        };
        namespace
            .separator_sets
            .insert("comma_and".to_string(), separator);

        namespaces.insert("test".to_string(), namespace);

        Package {
            id: "test.package".to_string(),
            version: "1.0.0".to_string(),
            metadata: PackageMetadata {
                name: "Test Package".to_string(),
                description: Some("A test package".to_string()),
                authors: vec!["Test".to_string()],
                bypass_filters: false,
            },
            namespaces,
            dependencies: Vec::new(),
        }
    }

    #[test]
    fn test_validation_result_new() {
        let result = ValidationResult::new();
        assert!(result.is_valid());
        assert!(!result.has_warnings());
    }

    #[test]
    fn test_validation_result_add_error() {
        let mut result = ValidationResult::new();
        result.add_error(ValidationError::ReferenceNotFound {
            reference: "test:missing".to_string(),
            defined_in: "test:prompt".to_string(),
            suggestion: None,
        });
        assert!(!result.is_valid());
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_validation_result_add_warning() {
        let mut result = ValidationResult::new();
        result.add_warning(ValidationWarning::UnusedDatatype {
            datatype: "colors".to_string(),
            namespace: "test".to_string(),
        });
        assert!(result.is_valid()); // Still valid with warnings
        assert!(result.has_warnings());
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_valid_package() {
        let mut package = create_test_package();

        // Add a valid promptsection
        let mut ps = PromptSection {
            name: "test_prompt".to_string(),
            template: "{color}".to_string(),
            references: HashMap::new(),
        };

        ps.references.insert(
            "color".to_string(),
            Reference {
                target: "colors".to_string(),
                filter: None,
                min: 1,
                max: 1,
                separator: None,
                unique: false,
            },
        );

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .prompt_sections
            .insert("test_prompt".to_string(), ps);

        let result = PackageValidator::validate(&package);
        assert!(result.is_valid(), "Package should be valid");
    }

    #[test]
    fn test_reference_not_found() {
        let mut package = create_test_package();

        let mut ps = PromptSection {
            name: "test_prompt".to_string(),
            template: "{color}".to_string(),
            references: HashMap::new(),
        };

        // Reference non-existent datatype
        ps.references.insert(
            "color".to_string(),
            Reference {
                target: "missing_datatype".to_string(),
                filter: None,
                min: 1,
                max: 1,
                separator: None,
                unique: false,
            },
        );

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .prompt_sections
            .insert("test_prompt".to_string(), ps);

        let result = PackageValidator::validate(&package);
        assert!(!result.is_valid(), "Should find reference error");
        assert_eq!(result.errors.len(), 1);
        assert!(matches!(
            result.errors[0],
            ValidationError::ReferenceNotFound { .. }
        ));
    }

    #[test]
    fn test_min_max_invalid() {
        let mut package = create_test_package();

        let mut ps = PromptSection {
            name: "test_prompt".to_string(),
            template: "{colors}".to_string(),
            references: HashMap::new(),
        };

        // min > max
        ps.references.insert(
            "colors".to_string(),
            Reference {
                target: "colors".to_string(),
                filter: None,
                min: 5,
                max: 2,
                separator: None,
                unique: false,
            },
        );

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .prompt_sections
            .insert("test_prompt".to_string(), ps);

        let result = PackageValidator::validate(&package);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| matches!(e, ValidationError::MinMaxInvalid { .. })));
    }

    #[test]
    fn test_separator_not_found() {
        let mut package = create_test_package();

        let mut ps = PromptSection {
            name: "test_prompt".to_string(),
            template: "{colors}".to_string(),
            references: HashMap::new(),
        };

        // Reference non-existent separator
        ps.references.insert(
            "colors".to_string(),
            Reference {
                target: "colors".to_string(),
                filter: None,
                min: 2,
                max: 3,
                separator: Some("missing_separator".to_string()),
                unique: false,
            },
        );

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .prompt_sections
            .insert("test_prompt".to_string(), ps);

        let result = PackageValidator::validate(&package);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| matches!(e, ValidationError::SeparatorNotFound { .. })));
    }

    #[test]
    fn test_unique_constraint_infeasible() {
        let mut package = create_test_package();

        let mut ps = PromptSection {
            name: "test_prompt".to_string(),
            template: "{colors}".to_string(),
            references: HashMap::new(),
        };

        // Request 5 unique values but only 2 available
        ps.references.insert(
            "colors".to_string(),
            Reference {
                target: "colors".to_string(),
                filter: None,
                min: 5,
                max: 5,
                separator: None,
                unique: true,
            },
        );

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .prompt_sections
            .insert("test_prompt".to_string(), ps);

        let result = PackageValidator::validate(&package);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| matches!(e, ValidationError::UniqueConstraintInfeasible { .. })));
    }

    #[test]
    fn test_unused_datatype_warning() {
        let package = create_test_package(); // Has colors datatype but no references

        let result = PackageValidator::validate(&package);
        assert!(result.is_valid()); // Valid but has warnings
        assert!(result.has_warnings());
        assert!(result
            .warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::UnusedDatatype { .. })));
    }

    #[test]
    fn test_invalid_naming() {
        let mut package = create_test_package();

        // Add datatype with invalid name (uppercase)
        let datatype = Datatype {
            name: "InvalidName".to_string(),
            values: vec![],
            extends: None,
            override_tags: HashMap::new(),
        };
        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .datatypes
            .insert("InvalidName".to_string(), datatype);

        let result = PackageValidator::validate(&package);
        assert!(!result.is_valid());
        assert!(result
            .errors
            .iter()
            .any(|e| matches!(e, ValidationError::InvalidNaming { .. })));
    }

    #[test]
    fn test_is_valid_name() {
        assert!(PackageValidator::is_valid_name("valid_name"));
        assert!(PackageValidator::is_valid_name("valid-name"));
        assert!(PackageValidator::is_valid_name("valid.name")); // Dots are allowed for hierarchical namespacing
        assert!(PackageValidator::is_valid_name("valid123"));
        assert!(PackageValidator::is_valid_name("v"));

        assert!(!PackageValidator::is_valid_name("Invalid"));
        assert!(!PackageValidator::is_valid_name("invalid Name"));
        assert!(!PackageValidator::is_valid_name("123invalid"));
        assert!(!PackageValidator::is_valid_name(""));
    }

    // M9 Task 1.4: Rulebook validation tests

    #[test]
    fn test_rulebook_valid() {
        use crate::core::rulebook::{EntryPoint, Rulebook};

        let mut package = create_test_package();

        // Add a promptsection
        let mut refs = HashMap::new();
        refs.insert(
            "color".to_string(),
            Reference {
                target: "test:colors".to_string(),
                filter: None,
                min: 1,
                max: 1,
                separator: None,
                unique: false,
            },
        );

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .prompt_sections
            .insert(
                "simple".to_string(),
                PromptSection {
                    name: "simple".to_string(),
                    template: "{color} item".to_string(),
                    references: refs,
                },
            );

        // Add a valid rulebook
        let rulebook = Rulebook {
            name: "test_rulebook".to_string(),
            description: "Test".to_string(),
            entry_points: vec![EntryPoint {
                prompt_section: "test:simple".to_string(),
                weight: 1.0,
            }],
            batch_variety: false,
            context_defaults: HashMap::new(),
        };

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .rulebooks
            .insert("test_rulebook".to_string(), rulebook);

        let result = PackageValidator::validate(&package);
        assert!(result.is_valid());
    }

    #[test]
    fn test_rulebook_invalid_entry_point_reference() {
        use crate::core::rulebook::{EntryPoint, Rulebook};

        let mut package = create_test_package();

        // Add rulebook with non-existent promptsection
        let rulebook = Rulebook {
            name: "test_rulebook".to_string(),
            description: "Test".to_string(),
            entry_points: vec![EntryPoint {
                prompt_section: "test:nonexistent".to_string(),
                weight: 1.0,
            }],
            batch_variety: false,
            context_defaults: HashMap::new(),
        };

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .rulebooks
            .insert("test_rulebook".to_string(), rulebook);

        let result = PackageValidator::validate(&package);
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| matches!(
            e,
            ValidationError::ReferenceNotFound { reference, .. }
            if reference.contains("nonexistent")
        )));
    }

    #[test]
    fn test_rulebook_invalid_context_key() {
        use crate::core::rulebook::{EntryPoint, Rulebook};

        let mut package = create_test_package();

        // Add a promptsection
        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .prompt_sections
            .insert(
                "simple".to_string(),
                PromptSection {
                    name: "simple".to_string(),
                    template: "test".to_string(),
                    references: HashMap::new(),
                },
            );

        // Add rulebook with invalid context key format
        let mut context_defaults = HashMap::new();
        context_defaults.insert("invalid::key".to_string(), "value".to_string()); // Double colon is invalid

        let rulebook = Rulebook {
            name: "test_rulebook".to_string(),
            description: "Test".to_string(),
            entry_points: vec![EntryPoint {
                prompt_section: "test:simple".to_string(),
                weight: 1.0,
            }],
            batch_variety: false,
            context_defaults,
        };

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .rulebooks
            .insert("test_rulebook".to_string(), rulebook);

        let result = PackageValidator::validate(&package);
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| matches!(
            e,
            ValidationError::InvalidNaming { reason, .. }
            if reason.contains("scope:key")
        )));
    }

    #[test]
    fn test_rulebook_empty_context_key() {
        use crate::core::rulebook::{EntryPoint, Rulebook};

        let mut package = create_test_package();

        // Add a promptsection
        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .prompt_sections
            .insert(
                "simple".to_string(),
                PromptSection {
                    name: "simple".to_string(),
                    template: "test".to_string(),
                    references: HashMap::new(),
                },
            );

        // Add rulebook with empty context key
        let mut context_defaults = HashMap::new();
        context_defaults.insert("".to_string(), "value".to_string());

        let rulebook = Rulebook {
            name: "test_rulebook".to_string(),
            description: "Test".to_string(),
            entry_points: vec![EntryPoint {
                prompt_section: "test:simple".to_string(),
                weight: 1.0,
            }],
            batch_variety: false,
            context_defaults,
        };

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .rulebooks
            .insert("test_rulebook".to_string(), rulebook);

        let result = PackageValidator::validate(&package);
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| matches!(
            e,
            ValidationError::InvalidNaming { reason, .. }
            if reason.contains("cannot be empty")
        )));
    }

    #[test]
    fn test_rulebook_cross_namespace_reference() {
        use crate::core::rulebook::{EntryPoint, Rulebook};

        let mut package = create_test_package();

        // Add another namespace
        let mut namespace2 = Namespace {
            id: "other".to_string(),
            datatypes: HashMap::new(),
            prompt_sections: HashMap::new(),
            separator_sets: HashMap::new(),
            rules: HashMap::new(),
            decisions: Vec::new(),
            rulebooks: HashMap::new(),
        };

        // Add a promptsection in other namespace
        namespace2.prompt_sections.insert(
            "scene".to_string(),
            PromptSection {
                name: "scene".to_string(),
                template: "test scene".to_string(),
                references: HashMap::new(),
            },
        );

        package.namespaces.insert("other".to_string(), namespace2);

        // Add rulebook in test namespace that references other namespace
        let rulebook = Rulebook {
            name: "cross_ns".to_string(),
            description: "Cross-namespace test".to_string(),
            entry_points: vec![EntryPoint {
                prompt_section: "other:scene".to_string(),
                weight: 1.0,
            }],
            batch_variety: false,
            context_defaults: HashMap::new(),
        };

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .rulebooks
            .insert("cross_ns".to_string(), rulebook);

        let result = PackageValidator::validate(&package);
        assert!(result.is_valid()); // Cross-namespace references should be valid
    }

    #[test]
    fn test_rulebook_invalid_entry_point_format() {
        use crate::core::rulebook::{EntryPoint, Rulebook};

        let mut package = create_test_package();

        // Add rulebook with invalid reference format (too many colons)
        let rulebook = Rulebook {
            name: "test_rulebook".to_string(),
            description: "Test".to_string(),
            entry_points: vec![EntryPoint {
                prompt_section: "test:invalid:format".to_string(),
                weight: 1.0,
            }],
            batch_variety: false,
            context_defaults: HashMap::new(),
        };

        package
            .namespaces
            .get_mut("test")
            .unwrap()
            .rulebooks
            .insert("test_rulebook".to_string(), rulebook);

        let result = PackageValidator::validate(&package);
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| matches!(
            e,
            ValidationError::ReferenceNotFound { suggestion, .. }
            if suggestion.as_ref().map(|s| s.contains("namespace:name")).unwrap_or(false)
        )));
    }
}

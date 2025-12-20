// M7 Phase 3: Validation Commands

use crate::core::models::Package;
use crate::validator::{PackageValidator, ValidationError};
use serde::Serialize;
use tauri::command;

#[derive(Serialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ErrorInfo>,
    pub warnings: Vec<String>,
}

#[derive(Serialize)]
pub struct ErrorInfo {
    pub message: String,
    pub location: Option<String>,
    pub suggestion: Option<String>,
}

impl From<ValidationError> for ErrorInfo {
    fn from(err: ValidationError) -> Self {
        match err {
            ValidationError::ReferenceNotFound {
                reference,
                defined_in,
                suggestion,
            } => ErrorInfo {
                message: format!("Reference not found: '{}' in {}", reference, defined_in),
                location: Some(defined_in),
                suggestion,
            },
            ValidationError::CircularReference { chain } => ErrorInfo {
                message: format!("Circular reference detected: {}", chain),
                location: None,
                suggestion: Some("Break the circular dependency".to_string()),
            },
            ValidationError::InvalidTagFilter { expression, reason } => ErrorInfo {
                message: format!("Invalid tag filter '{}': {}", expression, reason),
                location: None,
                suggestion: Some("Check tag filter syntax".to_string()),
            },
            ValidationError::SeparatorNotFound {
                separator,
                defined_in,
            } => ErrorInfo {
                message: format!("Separator set not found: '{}' in {}", separator, defined_in),
                location: Some(defined_in),
                suggestion: None,
            },
            ValidationError::MinMaxInvalid {
                min,
                max,
                defined_in,
            } => ErrorInfo {
                message: format!("Min ({}) must be <= Max ({}) in {}", min, max, defined_in),
                location: Some(defined_in),
                suggestion: Some(format!("Change min to <= {}", max)),
            },
            ValidationError::UniqueConstraintInfeasible {
                requested,
                available,
                datatype,
            } => ErrorInfo {
                message: format!(
                    "Unique constraint infeasible: requested {} unique values but only {} available in {}",
                    requested, available, datatype
                ),
                location: Some(datatype),
                suggestion: Some(format!("Reduce max to <= {}", available)),
            },
            ValidationError::InvalidRule { rule_name, reason } => ErrorInfo {
                message: format!("Invalid rule '{}': {}", rule_name, reason),
                location: Some(rule_name),
                suggestion: None,
            },
            ValidationError::DuplicateId { id, namespace } => ErrorInfo {
                message: format!("Duplicate ID '{}' in namespace '{}'", id, namespace),
                location: Some(namespace.clone()),
                suggestion: Some("Use a unique ID".to_string()),
            },
            ValidationError::InvalidNaming { name, reason } => ErrorInfo {
                message: format!("Invalid naming '{}': {}", name, reason),
                location: Some(name.clone()),
                suggestion: Some("Use lowercase alphanumeric with underscores/hyphens".to_string()),
            },
            ValidationError::InvalidDependency { package_id, reason } => ErrorInfo {
                message: format!("Invalid dependency '{}': {}", package_id, reason),
                location: Some(format!("dependencies.{}", package_id)),
                suggestion: Some("Check dependency declaration".to_string()),
            },
            ValidationError::InvalidDependencyVersion {
                package_id,
                version,
                reason,
            } => ErrorInfo {
                message: format!(
                    "Invalid version '{}' for dependency '{}': {}",
                    version, package_id, reason
                ),
                location: Some(format!("dependencies.{}", package_id)),
                suggestion: Some("Use semver format (e.g., 1.0.0)".to_string()),
            },
        }
    }
}

#[command]
pub async fn validate_package(package: Package) -> Result<ValidationResult, String> {
    let result = PackageValidator::validate(&package);
    
    if result.is_valid() {
        Ok(ValidationResult {
            is_valid: true,
            errors: vec![],
            warnings: result.warnings.iter().map(|w| w.to_string()).collect(),
        })
    } else {
        let error_infos: Vec<ErrorInfo> = result.errors.into_iter().map(|e| e.into()).collect();

        Ok(ValidationResult {
            is_valid: false,
            errors: error_infos,
            warnings: result.warnings.iter().map(|w| w.to_string()).collect(),
        })
    }
}

/// M9 Phase 3: Validate package with dependencies
#[command]
pub async fn validate_package_with_dependencies(
    package: Package,
    dependencies: std::collections::HashMap<String, Package>,
) -> Result<ValidationResult, String> {
    let result = PackageValidator::validate_with_dependencies(&package, &dependencies);

    if result.is_valid() {
        Ok(ValidationResult {
            is_valid: true,
            errors: vec![],
            warnings: result.warnings.iter().map(|w| w.to_string()).collect(),
        })
    } else {
        let error_infos: Vec<ErrorInfo> = result.errors.into_iter().map(|e| e.into()).collect();

        Ok(ValidationResult {
            is_valid: false,
            errors: error_infos,
            warnings: result.warnings.iter().map(|w| w.to_string()).collect(),
        })
    }
}


// M9 Phase 2.3: Version Matching (Exact Only)
//
// We enforce exact version matching to guarantee deterministic rendering.
// See DESIGN_DECISION_EXACT_VERSION_MATCHING.md for rationale.

use std::fmt;

/// Semantic version (major.minor.patch)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    /// Parse a version string like "1.0.0"
    pub fn parse(s: &str) -> Result<Self, VersionError> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 3 {
            return Err(VersionError::InvalidFormat {
                input: s.to_string(),
                reason: format!("Expected 3 parts (major.minor.patch), found {}", parts.len()),
            });
        }

        let major = parts[0].parse::<u32>().map_err(|_| VersionError::InvalidFormat {
            input: s.to_string(),
            reason: format!("Invalid major version: '{}'", parts[0]),
        })?;

        let minor = parts[1].parse::<u32>().map_err(|_| VersionError::InvalidFormat {
            input: s.to_string(),
            reason: format!("Invalid minor version: '{}'", parts[1]),
        })?;

        let patch = parts[2].parse::<u32>().map_err(|_| VersionError::InvalidFormat {
            input: s.to_string(),
            reason: format!("Invalid patch version: '{}'", parts[2]),
        })?;

        Ok(Version { major, minor, patch })
    }

    /// Check if this version exactly matches another
    pub fn matches_exact(&self, other: &Version) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Validate that a found version matches the required version
/// M9 Phase 3: Now accepts semver ranges (strips prefixes for matching)
pub fn validate_exact_match(found: &str, required: &str) -> Result<(), VersionError> {
    let found_version = Version::parse(found)?;

    // M9 Phase 3: Strip semver prefix from required version for parsing
    let required_clean = required.trim_start_matches(|c| c == '^' || c == '~' || c == '>' || c == '<' || c == '=');
    let required_version = Version::parse(required_clean)?;

    // Check if found version satisfies the requirement
    if required.starts_with('^') {
        // Caret: Compatible updates (same major version)
        if found_version.major != required_version.major {
            return Err(VersionError::Mismatch {
                required: required.to_string(),
                found: found.to_string(),
            });
        }
        // For ^0.x.y, minor version must also match
        if required_version.major == 0 && found_version.minor != required_version.minor {
            return Err(VersionError::Mismatch {
                required: required.to_string(),
                found: found.to_string(),
            });
        }
    } else if required.starts_with('~') {
        // Tilde: Patch updates only (same major.minor)
        if found_version.major != required_version.major || found_version.minor != required_version.minor {
            return Err(VersionError::Mismatch {
                required: required.to_string(),
                found: found.to_string(),
            });
        }
    } else if required.starts_with(">=") {
        // Greater than or equal
        if !version_gte(&found_version, &required_version) {
            return Err(VersionError::Mismatch {
                required: required.to_string(),
                found: found.to_string(),
            });
        }
    } else {
        // Exact match
        if !found_version.matches_exact(&required_version) {
            return Err(VersionError::Mismatch {
                required: required.to_string(),
                found: found.to_string(),
            });
        }
    }

    Ok(())
}

/// Helper: Check if version a >= version b
fn version_gte(a: &Version, b: &Version) -> bool {
    if a.major != b.major {
        return a.major > b.major;
    }
    if a.minor != b.minor {
        return a.minor > b.minor;
    }
    a.patch >= b.patch
}

/// Version-related errors
#[derive(Debug, Clone)]
pub enum VersionError {
    /// Invalid version format
    InvalidFormat {
        input: String,
        reason: String,
    },

    /// Version mismatch (exact match required)
    Mismatch {
        required: String,
        found: String,
    },
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionError::InvalidFormat { input, reason } => {
                write!(f, "Invalid version format '{}': {}", input, reason)
            }
            VersionError::Mismatch { required, found } => {
                write!(
                    f,
                    "Version mismatch: required {} (exact), found {}\n\n\
                     Note: Exact version matching is required for deterministic rendering.\n\
                     Different versions may produce different outputs with the same seed.",
                    required, found
                )
            }
        }
    }
}

impl std::error::Error for VersionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_version() {
        let v = Version::parse("1.0.0").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 0);
        assert_eq!(v.patch, 0);

        let v = Version::parse("2.3.4").unwrap();
        assert_eq!(v.major, 2);
        assert_eq!(v.minor, 3);
        assert_eq!(v.patch, 4);
    }

    #[test]
    fn test_parse_invalid_format() {
        assert!(Version::parse("1.0").is_err());
        assert!(Version::parse("1.0.0.0").is_err());
        assert!(Version::parse("1").is_err());
        assert!(Version::parse("").is_err());
    }

    #[test]
    fn test_parse_invalid_numbers() {
        assert!(Version::parse("a.b.c").is_err());
        assert!(Version::parse("1.0.x").is_err());
        assert!(Version::parse("-1.0.0").is_err());
    }

    #[test]
    fn test_exact_match() {
        let v1 = Version::parse("1.0.0").unwrap();
        let v2 = Version::parse("1.0.0").unwrap();
        assert!(v1.matches_exact(&v2));

        let v3 = Version::parse("1.0.1").unwrap();
        assert!(!v1.matches_exact(&v3));

        let v4 = Version::parse("1.1.0").unwrap();
        assert!(!v1.matches_exact(&v4));

        let v5 = Version::parse("2.0.0").unwrap();
        assert!(!v1.matches_exact(&v5));
    }

    #[test]
    fn test_validate_exact_match_success() {
        assert!(validate_exact_match("1.0.0", "1.0.0").is_ok());
        assert!(validate_exact_match("2.3.4", "2.3.4").is_ok());
    }

    #[test]
    fn test_validate_exact_match_failure() {
        assert!(validate_exact_match("1.0.0", "1.0.1").is_err());
        assert!(validate_exact_match("1.0.0", "2.0.0").is_err());
        assert!(validate_exact_match("1.2.3", "1.2.4").is_err());
    }

    #[test]
    fn test_version_display() {
        let v = Version::parse("1.2.3").unwrap();
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_error_messages() {
        let err = Version::parse("1.0").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Invalid version format"));
        assert!(msg.contains("Expected 3 parts"));

        let err = validate_exact_match("1.0.0", "2.0.0").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Version mismatch"));
        assert!(msg.contains("deterministic rendering"));
    }
}


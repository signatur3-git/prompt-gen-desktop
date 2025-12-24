// Integration test: Validate all existing good packages

#[cfg(test)]
mod integration_tests {
    use crate::parser::parse_yaml;
    use crate::validator::PackageValidator;
    use std::fs;

    #[test]
    fn test_validate_minimal_yaml() {
        let content = fs::read_to_string("../test-packages/minimal.yaml")
            .expect("Failed to read minimal.yaml");
        let package = parse_yaml(&content).expect("Failed to parse minimal.yaml");

        let result = PackageValidator::validate(&package);

        // Should be valid (though may have warnings about unused components)
        assert!(
            result.is_valid(),
            "minimal.yaml should be valid. Errors: {:?}",
            result.errors
        );
    }

    #[test]
    fn test_validate_article_test() {
        let content = fs::read_to_string("../test-packages/article-test.yaml")
            .expect("Failed to read article-test.yaml");
        let package = parse_yaml(&content).expect("Failed to parse article-test.yaml");

        let result = PackageValidator::validate(&package);

        assert!(
            result.is_valid(),
            "article-test.yaml should be valid. Errors: {:?}",
            result.errors
        );
    }

    #[test]
    fn test_validate_lists_test() {
        let content = fs::read_to_string("../test-packages/lists-test.yaml")
            .expect("Failed to read lists-test.yaml");
        let package = parse_yaml(&content).expect("Failed to parse lists-test.yaml");

        let result = PackageValidator::validate(&package);

        assert!(
            result.is_valid(),
            "lists-test.yaml should be valid. Errors: {:?}",
            result.errors
        );
    }

    #[test]
    fn test_validate_missing_reference() {
        let content = fs::read_to_string("../test-packages/invalid/missing-reference.yaml")
            .expect("Failed to read missing-reference.yaml");
        let package = parse_yaml(&content).expect("Failed to parse missing-reference.yaml");

        let result = PackageValidator::validate(&package);

        assert!(
            !result.is_valid(),
            "missing-reference.yaml should be invalid. Errors: {:?}",
            result.errors
        );
        assert!(result.errors.len() > 0, "Should have at least one error");
    }

    #[test]
    fn test_validate_min_max_reversed() {
        let content = fs::read_to_string("../test-packages/invalid/min-max-reversed.yaml")
            .expect("Failed to read min-max-reversed.yaml");
        let package = parse_yaml(&content).expect("Failed to parse min-max-reversed.yaml");

        let result = PackageValidator::validate(&package);

        assert!(
            !result.is_valid(),
            "min-max-reversed.yaml should be invalid. Errors: {:?}",
            result.errors
        );
        assert!(
            result
                .errors
                .iter()
                .any(|e| matches!(e, crate::validator::ValidationError::MinMaxInvalid { .. })),
            "Should have MinMaxInvalid error. Got: {:?}",
            result.errors
        );
    }

    #[test]
    fn test_validate_circular_refs() {
        let content = fs::read_to_string("../test-packages/invalid/circular-refs.yaml")
            .expect("Failed to read circular-refs.yaml");
        let package = parse_yaml(&content).expect("Failed to parse circular-refs.yaml");

        let result = PackageValidator::validate(&package);

        assert!(
            !result.is_valid(),
            "circular-refs.yaml should be invalid. Errors: {:?}",
            result.errors
        );
        assert!(
            result.errors.iter().any(|e| matches!(
                e,
                crate::validator::ValidationError::CircularReference { .. }
            )),
            "Should have CircularReference error. Got: {:?}",
            result.errors
        );
    }
}

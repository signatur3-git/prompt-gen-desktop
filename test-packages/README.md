# Test Packages

This directory contains test YAML packages used for integration testing the validator.

## Valid Packages

- **minimal.yaml**: Basic package with minimal structure
- **article-test.yaml**: Tests article handling with tags
- **lists-test.yaml**: Tests list references with separators

## Invalid Packages (in `invalid/` subdirectory)

- **missing-reference.yaml**: Contains a reference to a non-existent datatype
- **min-max-reversed.yaml**: Has min > max in a reference constraint
- **circular-refs.yaml**: Contains circular references between prompt sections

These packages are used by the integration tests in `src-tauri/src/validator/integration_tests.rs`.


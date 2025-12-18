/// Fixture loading utilities for tests
///
/// This module provides functions to load JSON fixtures from the resources/test directory

use std::fs;
use std::path::PathBuf;

/// Gets the path to the test resources directory
fn get_resources_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources").join("test")
}

/// Loads a JSON fixture file as a string
///
/// # Arguments
///
/// * `filename` - The name of the fixture file (e.g., "result-1.json")
///
/// # Returns
///
/// The contents of the fixture file as a string
///
/// # Panics
///
/// Panics if the file cannot be read
pub fn load_fixture(filename: &str) -> String {
    let path = get_resources_path().join(filename);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read fixture file: {:?}", path))
}

/// Loads the request-1.json fixture
pub fn load_request_1() -> String {
    load_fixture("request-1.json")
}

/// Loads the result-1.json fixture
pub fn load_result_1() -> String {
    load_fixture("result-1.json")
}

/// Loads the result-2.json fixture
pub fn load_result_2() -> String {
    load_fixture("result-2.json")
}

/// Loads a fixture and parses it as JSON
///
/// # Arguments
///
/// * `filename` - The name of the fixture file
///
/// # Returns
///
/// A `serde_json::Value` representing the parsed JSON
pub fn load_fixture_json(filename: &str) -> serde_json::Value {
    let content = load_fixture(filename);
    serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse JSON from {}: {}", filename, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_fixture() {
        let content = load_fixture("result-1.json");
        assert!(!content.is_empty());
        assert!(content.contains("\"from\""));
    }

    #[test]
    fn test_load_request_1() {
        let content = load_request_1();
        assert!(!content.is_empty());
        assert!(content.contains("query"));
    }

    #[test]
    fn test_load_result_1() {
        let content = load_result_1();
        assert!(!content.is_empty());
        assert!(content.contains("results"));
    }

    #[test]
    fn test_load_result_2() {
        let content = load_result_2();
        assert!(!content.is_empty());
        assert!(content.contains("results"));
    }

    #[test]
    fn test_load_fixture_json() {
        let json = load_fixture_json("result-1.json");
        assert!(json.is_object());
        assert!(json["from"].is_number());
    }

    #[test]
    #[should_panic(expected = "Failed to read fixture file")]
    fn test_load_nonexistent_fixture() {
        load_fixture("nonexistent.json");
    }
}

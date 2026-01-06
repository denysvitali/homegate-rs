/// Integration tests for homegate-rs
///
/// These tests verify end-to-end functionality using mocked HTTP responses
/// to ensure all layers work together correctly.
mod common;

#[cfg(test)]
mod tests {
    use crate::common::fixtures::{load_result_1, load_result_2};
    use homegate::api::search::{default_search, Location};
    use homegate::models::paginated::{parse_search_result, Paginated};
    use homegate::models::realestate::RealEstate;

    #[test]
    fn test_end_to_end_search_result_parsing() {
        // Test complete flow: fixture loading -> parsing -> validation
        let fixture = load_result_2();
        let parsed = parse_search_result(&fixture).unwrap();

        // Verify paginated structure
        assert_eq!(parsed.from, 0);
        assert_eq!(parsed.size, 20);
        assert!(parsed.total > 0);
        assert!(!parsed.results.is_empty());

        // Verify first result has complete structure
        let first = &parsed.results[0];
        assert!(!first.id.is_empty());
        assert!(!first.listing.id.is_empty());
        assert!(!first.listing.categories.is_empty());
        assert!(!first.listing.address.postal_code.is_empty());
    }

    #[test]
    fn test_multiple_search_results_parsing() {
        // Test parsing multiple result fixtures
        let result1 = load_result_1();
        let result2 = load_result_2();

        let parsed1: Paginated<RealEstate> = serde_json::from_str(&result1).unwrap();
        let parsed2: Paginated<RealEstate> = serde_json::from_str(&result2).unwrap();

        // Both should be valid
        assert!(parsed1.total > 0);

        // Result 2 should have more results
        assert!(parsed2.total > parsed1.total);
    }

    #[test]
    fn test_search_request_to_response_flow() {
        // Test creating a search request and validating response structure
        let search_req = default_search();

        // Verify request is serializable
        let req_json = serde_json::to_string(&search_req).unwrap();
        assert!(!req_json.is_empty());

        // Load and parse response
        let response = load_result_2();
        let parsed: Paginated<RealEstate> = serde_json::from_str(&response).unwrap();

        // Verify response matches request expectations
        assert!(parsed.results.len() <= search_req.size as usize);
    }

    #[test]
    fn test_error_propagation_invalid_json() {
        // Test that invalid JSON is properly handled
        let invalid_json = r#"{"invalid": json"#;
        let result: Result<Paginated<RealEstate>, _> = serde_json::from_str(invalid_json);

        assert!(result.is_err());
    }

    #[test]
    fn test_complete_listing_data_extraction() {
        // Test extracting all relevant data from a listing
        let fixture = load_result_2();
        let parsed: Paginated<RealEstate> = serde_json::from_str(&fixture).unwrap();

        for result in &parsed.results {
            // Verify ID
            assert!(!result.id.is_empty());

            // Verify listing has required fields
            assert!(!result.listing.id.is_empty());
            assert!(!result.listing.categories.is_empty());

            // Verify address
            assert!(!result.listing.address.postal_code.is_empty());

            // Verify characteristics
            assert!(result.listing.characteristics.living_space > 0);
            assert!(result.listing.characteristics.number_of_rooms > 0.0);

            // Verify localization
            assert!(!result.listing.localization.primary.is_empty());
        }
    }

    #[test]
    fn test_location_based_search_structure() {
        // Test creating a location-based search
        let location = Location {
            latitude: 47.36667,
            longitude: 8.55,
            radius: 1000,
        };

        let mut search = default_search();
        search.query.location = location.clone();

        // Verify location is properly integrated
        assert_eq!(search.query.location.latitude, 47.36667);
        assert_eq!(search.query.location.longitude, 8.55);
        assert_eq!(search.query.location.radius, 1000);

        // Verify it can be serialized
        let json = serde_json::to_string(&search).unwrap();
        assert!(json.contains("latitude"));
        assert!(json.contains("longitude"));
        assert!(json.contains("radius"));
    }

    #[test]
    fn test_pagination_handling() {
        // Test pagination parameters
        let fixture = load_result_2();
        let parsed: Paginated<RealEstate> = serde_json::from_str(&fixture).unwrap();

        // Verify pagination metadata
        assert!(parsed.size > 0);

        // Verify results fit within page size
        assert!(parsed.results.len() <= parsed.size as usize);

        // Calculate if there are more pages
        let has_more = (parsed.from + parsed.size) < parsed.total;
        if has_more {
            assert!(parsed.total > parsed.results.len() as u32);
        }
    }

    #[test]
    fn test_real_fixture_data_consistency() {
        // Test that real fixture data maintains consistency
        let fixture = load_result_2();
        let parsed: Paginated<RealEstate> = serde_json::from_str(&fixture).unwrap();

        // All listings should have the same offer type (RENT in test data)
        for result in &parsed.results {
            // Just verify the structure exists, actual value may vary
            assert!(!result.listing.id.is_empty());
        }

        // Verify at least one listing has localization in German
        let has_german = parsed
            .results
            .iter()
            .any(|r| r.listing.localization.de.is_some());
        assert!(
            has_german,
            "At least one listing should have German localization"
        );
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires network access
    async fn test_live_api_integration() {
        // This test would make a real API call
        // It's marked as ignored to prevent running during normal test runs
        use homegate::api::search::search;

        let location = Location {
            latitude: 47.36667,
            longitude: 8.55,
            radius: 1000,
        };

        let result = search(&location).await;
        assert!(result.is_ok());
    }
}

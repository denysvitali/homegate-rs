use crate::models::realestate::RealEstate;
use serde::{Deserialize, Serialize};

/// Paginated response container for API results.
///
/// This struct wraps API responses that return paginated data, providing
/// metadata about the pagination state and the actual results.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T> {
    /// Starting index of the current page
    pub from: u32,
    /// Maximum allowed starting index
    pub max_from: u32,
    /// Results for the current page
    pub results: Vec<T>,
    /// Number of items per page
    pub size: u32,
    /// Total number of items across all pages
    pub total: u32,
}

/// Parses a search result JSON string into a paginated real estate listing.
///
/// # Arguments
///
/// * `str` - JSON string containing the search results
///
/// # Returns
///
/// Returns a `Paginated<RealEstate>` on success.
///
/// # Errors
///
/// Returns an error if the JSON cannot be parsed into the expected format.
pub fn parse_search_result(str: &str) -> crate::Result<Paginated<RealEstate>> {
    serde_json::from_str(str).map_err(Into::into)
}

#[cfg(test)]
mod test {
    use crate::models::paginated::parse_search_result;
    use std::fs;

    #[test]
    pub fn parse_result_2() {
        let file =
            fs::read_to_string("./resources/test/result-2.json").expect("Failed to read test file");
        let paginated_result = parse_search_result(&file).expect("Failed to parse search result");

        assert!(paginated_result.total > 0)
    }
}

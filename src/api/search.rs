//! Search functionality for real estate listings.
//!
//! This module provides search request structures and functions for querying
//! the Homegate API for real estate listings based on various criteria.

use serde::{Deserialize, Serialize};

use crate::api::request::HomegateClient;
use crate::models::listing::Category;
use crate::models::paginated::Paginated;
use crate::models::realestate::{OfferType, RealEstate};

/// Range filter for numeric values.
///
/// Used to specify minimum and maximum values for search criteria like price,
/// living space, or number of rooms.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FromTo {
    /// Minimum value (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u32>,
    /// Maximum value (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<u32>,
}

impl FromTo {
    /// Validates that the range is valid (from <= to if both are specified).
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if valid, or an error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if let (Some(from), Some(to)) = (self.from, self.to) {
            if from > to {
                return Err(format!(
                    "Invalid range: 'from' ({}) must be less than or equal to 'to' ({})",
                    from, to
                ));
            }
        }
        Ok(())
    }
}

/// Range filter for floating-point numeric values.
///
/// Used to specify minimum and maximum values for search criteria that support
/// fractional values, such as number of rooms (2.5 rooms, 3.5 rooms, etc.).
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FromToFloat {
    /// Minimum value (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<f32>,
    /// Maximum value (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<f32>,
}

impl FromToFloat {
    /// Validates that the range is valid (from <= to if both are specified).
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if valid, or an error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if let (Some(from), Some(to)) = (self.from, self.to) {
            if from > to {
                return Err(format!(
                    "Invalid range: 'from' ({}) must be less than or equal to 'to' ({})",
                    from, to
                ));
            }
        }
        Ok(())
    }
}

/// Geographic location with radius for search queries.
///
/// Defines a circular search area centered on the given coordinates.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Location {
    /// Latitude in decimal degrees
    pub latitude: f32,
    /// Longitude in decimal degrees
    pub longitude: f32,
    /// Search radius in meters
    pub radius: u32,
}

impl Location {
    /// Validates the location parameters.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if valid, or an error message if invalid.
    ///
    /// # Validation Rules
    ///
    /// - Latitude must be between -90 and 90 degrees
    /// - Longitude must be between -180 and 180 degrees
    /// - Radius must be greater than 0 and less than 50000 meters
    pub fn validate(&self) -> Result<(), String> {
        if self.latitude < -90.0 || self.latitude > 90.0 {
            return Err(format!(
                "Invalid latitude: {} (must be between -90 and 90)",
                self.latitude
            ));
        }
        if self.longitude < -180.0 || self.longitude > 180.0 {
            return Err(format!(
                "Invalid longitude: {} (must be between -180 and 180)",
                self.longitude
            ));
        }
        if self.radius == 0 {
            return Err("Invalid radius: must be greater than 0".to_string());
        }
        if self.radius >= 50000 {
            return Err(format!(
                "Invalid radius: {} (must be less than 50000 meters)",
                self.radius
            ));
        }
        Ok(())
    }
}

/// Main search query parameters.
///
/// Specifies all filtering criteria for a real estate search.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    /// Property categories to include in search
    pub categories: Vec<String>,
    /// Property categories to exclude from search
    pub exclude_categories: Vec<String>,
    /// Living space filter in square meters
    pub living_space: FromTo,
    /// Geographic location and search radius
    pub location: Location,
    /// Monthly rent filter in CHF
    pub monthly_rent: FromTo,
    /// Number of rooms filter (supports fractional values like 2.5, 3.5)
    pub number_of_rooms: FromToFloat,
    /// Type of offer (RENT, BUY, etc.)
    pub offer_type: OfferType,
}

/// Template for geographic coordinate fields in search results.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GeoCoordsTemplate {
    pub latitude: bool,
    pub longitude: bool,
}

/// Template for address fields to include in search results.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddressTemplate {
    pub country: bool,
    pub geo_coordinates: GeoCoordsTemplate,
    pub locality: bool,
    pub post_office_box_number: bool,
    pub postal_code: bool,
    pub region: bool,
    pub street: bool,
    pub street_addition: bool,
}

/// Template for property characteristic fields in search results.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicsTemplate {
    pub living_space: bool,
    pub lot_size: bool,
    pub number_of_rooms: bool,
    pub single_floor_space: bool,
    pub total_floor_space: bool,
}

/// Template for lister information fields in search results.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListerTemplate {
    pub logo_url: bool,
    pub phone: bool,
}

/// Template for localized text fields in search results.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocaleTextTemplate {
    pub title: bool,
}

/// Template for localized URL fields in search results.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocaleUrlsTemplate {
    #[serde(rename = "type")]
    pub t: bool,
}

/// Template for a single locale's fields in search results.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocaleTemplate {
    pub attachments: bool,
    pub text: LocaleTextTemplate,
    pub urls: LocaleUrlsTemplate,
}

/// Template for all localization fields in search results.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationTemplate {
    pub de: LocaleTemplate,
    pub en: LocaleTemplate,
    pub fr: LocaleTemplate,
    pub it: LocaleTemplate,
    pub primary: bool,
}

/// Template specifying which listing fields to include in search results.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListingTemplate {
    pub address: AddressTemplate,
    pub categories: bool,
    pub characteristics: CharacteristicsTemplate,
    pub id: bool,
    pub lister: ListerTemplate,
    pub localization: LocalizationTemplate,
    pub offer_type: bool,
    pub prices: bool,
}

/// Template specifying which result fields to include in search response.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResultTemplate {
    pub id: bool,
    pub lister_branding: bool,
    pub listing: ListingTemplate,
    pub listing_type: bool,
    pub remote_viewing: bool,
}

/// Complete search request structure.
///
/// Combines query parameters, result template, pagination, and sorting options.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    /// Starting index for pagination
    pub from: i32,
    /// Search query parameters
    pub query: Query,
    /// Template specifying which fields to return
    pub result_template: ResultTemplate,
    /// Number of results per page
    pub size: i32,
    /// Field to sort results by
    pub sort_by: String,
    /// Sort direction ("asc" or "desc")
    pub sort_direction: String,
    /// Whether to track total number of hits
    pub track_total_hits: bool,
}

const LT: LocaleTemplate = LocaleTemplate {
    urls: LocaleUrlsTemplate {
        t: true
    },
    attachments: true,
    text: LocaleTextTemplate {
        title: true,
    },
};

/// Creates a default search request with common parameters.
///
/// This function returns a pre-configured `SearchRequest` with sensible defaults
/// for searching rental properties in the Zurich area. You can modify the returned
/// request to customize the search parameters.
///
/// # Returns
///
/// A `SearchRequest` configured with:
/// - Location: Zurich coordinates (47.36, 8.54) with 622m radius
/// - Categories: All apartment and house types (excluding furnished flats)
/// - Living space: Minimum 60m²
/// - Monthly rent: Minimum 500 CHF
/// - Number of rooms: Minimum 2
/// - Result template with all fields enabled
///
/// # Examples
///
/// ```no_run
/// use homegate::api::search::{default_search, Location};
///
/// let mut search_req = default_search();
/// search_req.query.location = Location {
///     latitude: 47.36667,
///     longitude: 8.55,
///     radius: 1000,
/// };
/// ```
pub fn default_search() -> SearchRequest {
    SearchRequest {
        from: 0,
        query: Query {
            categories: [
                Category::Apartment,
                Category::Maisonette,
                Category::Duplex,
                Category::AtticFlat,
                Category::RoofFlat,
                Category::Studio,
                Category::SingleRoom,
                Category::TerraceFlat,
                Category::BachelorFlat,
                Category::Loft,
                Category::Attic,
                Category::RowHouse,
                Category::BifamiliarHouse,
                Category::TerraceHouse,
                Category::Villa,
                Category::FarmHouse,
                Category::CaveHouse,
                Category::Castle,
                Category::GrannyFlat,
                Category::Chalet,
                Category::Rustico,
                Category::SingleHouse,
                Category::HobbyRoom,
                Category::CellarCompartment,
                Category::AtticCompartment,
            ].iter().map(|c| c.to_string()).collect(),
            exclude_categories: [
                Category::FurnishedFlat,
            ].iter().map(|c| c.to_string()).collect(),
            living_space: FromTo { from: Some(60), to: None },
            location: Location {
                latitude: 47.359_856,
                longitude: 8.541_819,
                radius: 622,
            },
            monthly_rent: FromTo { from: Some(500), to: None },
            number_of_rooms: FromToFloat {
                from: Some(2.0),
                to: None,
            },
            offer_type: OfferType::RENT,
        },
        result_template: ResultTemplate {
            id: true,
            lister_branding: true,
            listing: ListingTemplate {
                address: AddressTemplate {
                    country: true,
                    geo_coordinates: GeoCoordsTemplate { latitude: true, longitude: true },
                    locality: true,
                    post_office_box_number: true,
                    postal_code: true,
                    region: true,
                    street: true,
                    street_addition: true,
                },
                categories: true,
                characteristics: CharacteristicsTemplate {
                    living_space: true,
                    lot_size: true,
                    number_of_rooms: true,
                    single_floor_space: true,
                    total_floor_space: true,
                },
                id: true,
                lister: ListerTemplate { logo_url: true, phone: true },
                localization: LocalizationTemplate {
                    de: LT.clone(),
                    en: LT.clone(),
                    fr: LT.clone(),
                    it: LT.clone(),
                    primary: true,
                },
                offer_type: true,
                prices: true,
            },
            listing_type: true,
            remote_viewing: true,
        },
        size: 20,
        sort_by: String::from("listingType"),
        sort_direction: String::from("desc"),
        track_total_hits: true,
    }
}

/// Searches for real estate listings at the specified location.
///
/// Performs a search using default parameters with the provided location.
/// This is a convenience function that uses `default_search()` and overrides
/// only the location parameter.
///
/// # Arguments
///
/// * `location` - Geographic location and search radius
///
/// # Returns
///
/// Returns a `Paginated<RealEstate>` containing the search results on success,
/// or a `HomegateError` if the request fails or validation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - Location validation fails (invalid coordinates or radius)
/// - The network request fails
/// - The API returns an error response
/// - The response cannot be parsed
///
/// # Examples
///
/// ```no_run
/// use homegate::api::search::{search, Location};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let results = search(&Location {
///     latitude: 47.36667,
///     longitude: 8.55,
///     radius: 1000,
/// }).await?;
///
/// println!("Found {} total listings", results.total);
/// for listing in results.results {
///     println!("Listing ID: {}", listing.id);
/// }
/// # Ok(())
/// # }
/// ```
#[tracing::instrument(level = "info", fields(lat = %location.latitude, lon = %location.longitude, radius = %location.radius))]
pub async fn search(location: &Location) -> crate::Result<Paginated<RealEstate>> {
    // Create a client and delegate to it
    // This provides backward compatibility while using the optimized client internally
    let client = HomegateClient::new()?;
    client.search(location).await
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::api::search::{default_search, Location, search, SearchRequest};

    const ZURICH_LATLNG: (f64, f64) = (47.36667, 8.55);

    #[tokio::test]
    pub async fn search_apartment() {
        let paginated_result = search(
            &Location {
                latitude: ZURICH_LATLNG.0 as f32,
                longitude: ZURICH_LATLNG.1 as f32,
                radius: 1000,
            }).await;
        assert!(paginated_result.is_ok());

        if let Ok(pr) = paginated_result {
            println!("{:?}", pr);
        }
    }

    #[test]
    pub fn create_json() {
        let req = default_search();
        let _v = serde_json::to_string(&req).expect("Failed to serialize request");
        let f_json = fs::read_to_string("./resources/test/request-1.json")
            .expect("Failed to read test file");

        let decoded_json: SearchRequest = serde_json::from_str(f_json.as_str())
            .expect("Failed to deserialize JSON");
        assert_eq!(decoded_json, req);
    }
}

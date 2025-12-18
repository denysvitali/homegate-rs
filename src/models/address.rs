//! Address and location data structures.
//!
//! This module defines the address structure used throughout the Homegate API.

use serde::{Deserialize, Serialize};

use crate::models::geo_coords::GeoCoords;

/// Physical address of a property.
///
/// Contains both structured address fields and geographic coordinates.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// Country name (usually "Schweiz" or "Switzerland")
    pub country: Option<String>,
    /// Geographic coordinates (latitude/longitude)
    pub geo_coordinates: GeoCoords,
    /// City or locality name
    pub locality: Option<String>,
    /// Postal code (Swiss format, e.g., "8001")
    pub postal_code: String,
    /// Region or canton (e.g., "Zürich")
    pub region: Option<String>,
    /// Street name and number
    pub street: Option<String>,
}
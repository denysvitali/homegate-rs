use serde::{Deserialize, Serialize};

use crate::models::geo_coords::GeoCoords;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub country: Option<String>,
    pub geo_coordinates: GeoCoords,
    pub locality: Option<String>,
    pub postal_code: String,
    pub region: Option<String>,
    pub street: Option<String>,
}
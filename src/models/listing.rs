//! Real estate listing models and property categories.
//!
//! This module defines the structure of real estate listings including property
//! categories, characteristics, prices, and localization information.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::models::address::Address;
use crate::models::realestate::OfferType;

/// Property category classification.
///
/// Homegate categorizes properties into various types. This enum covers all
/// supported property categories from apartments to specialized rooms.
///
/// # Categories
///
/// ## Apartments
/// - `Apartment` - Standard apartment
/// - `Flat` - Generic flat
/// - `Maisonette` - Multi-level apartment
/// - `Duplex` - Two-level apartment
/// - `AtticFlat` - Apartment in the attic
/// - `RoofFlat` - Apartment with roof access
/// - `Studio` - Single-room apartment
/// - `SingleRoom` - Individual room
/// - `TerraceFlat` - Apartment with terrace
/// - `BachelorFlat` - Small one-person apartment
/// - `Loft` - Open-plan apartment
/// - `Attic` - Attic space
/// - `FurnishedFlat` - Fully furnished apartment
///
/// ## Houses
/// - `House` - Generic house
/// - `RowHouse` - Terraced house
/// - `BifamiliarHouse` - Two-family house
/// - `TerraceHouse` - House with terrace
/// - `Villa` - Detached villa
/// - `FarmHouse` - Rural farmhouse
/// - `CaveHouse` - Underground dwelling
/// - `Castle` - Castle or manor
/// - `GrannyFlat` - Self-contained unit
/// - `Chalet` - Mountain chalet
/// - `Rustico` - Traditional stone house
/// - `SingleHouse` - Detached single-family home
///
/// ## Other
/// - `HobbyRoom` - Hobby or utility room
/// - `CellarCompartment` - Cellar storage
/// - `AtticCompartment` - Attic storage
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Category {
    Flat,
    Apartment,
    Maisonette,
    Duplex,
    AtticFlat,
    RoofFlat,
    Studio,
    SingleRoom,
    TerraceFlat,
    BachelorFlat,
    Loft,
    Attic,
    House,
    RowHouse,
    BifamiliarHouse,
    TerraceHouse,
    Villa,
    FarmHouse,
    CaveHouse,
    Castle,
    GrannyFlat,
    Chalet,
    Rustico,
    SingleHouse,
    HobbyRoom,
    CellarCompartment,
    AtticCompartment,
    FurnishedFlat,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Serialize::serialize(self, f)
    }
}

/// Property characteristics.
///
/// Contains key physical attributes of a property.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Characteristics {
    /// Living space in square meters
    pub living_space: u32,
    /// Number of rooms (can be fractional, e.g., 2.5 rooms)
    pub number_of_rooms: f32,
}

/// Information about the property lister.
///
/// Contains contact information for the person or agency listing the property.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Lister {
    /// Contact phone number
    pub phone: Option<String>,
}

/// Attachment (image, document, etc.) associated with a listing.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    /// Type of attachment (e.g., "IMAGE")
    #[serde(rename = "type")]
    pub t: String,
    /// URL to access the attachment
    pub url: String,
    /// Filename of the attachment
    pub file: String,
}

/// Text content for a localized listing entry.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationEntryText {
    /// Listing title in the specific language
    pub title: String,
}

/// Localized content for a listing in a specific language.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationEntry {
    /// Attachments (images, documents) for this locale
    pub attachments: Vec<Attachment>,
    /// Text content in this locale
    pub text: LocalizationEntryText,
}

/// Multi-language localization data for a listing.
///
/// Homegate supports multiple languages (German, French, Italian, English).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Localization {
    /// German localization (if available)
    pub de: Option<LocalizationEntry>,
    /// Primary language code for this listing
    pub primary: String,
}

/// Price interval enumeration.
///
/// Specifies how often a price applies (monthly, weekly, etc.).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PriceInterval {
    /// Monthly payment
    MONTH,
}

/// Price information for rent or purchase.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    /// Payment interval (monthly, weekly, etc.)
    pub interval: Option<PriceInterval>,
    /// Net price (excluding additional costs)
    pub net: Option<u32>,
    /// Gross price (including additional costs)
    pub gross: Option<u32>,
    /// Extra costs (utilities, etc.)
    pub extra: Option<u32>,
}

/// Currency enumeration.
///
/// Currently only Swiss Francs are supported.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Currency {
    /// Swiss Francs
    CHF,
}

/// Complete pricing information for a listing.
///
/// Includes both rental and purchase prices (though typically only one applies).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    /// Rental price information
    pub rent: Option<Price>,
    /// Currency used for all prices
    pub currency: Currency,
    /// Purchase price information
    pub buy: Option<Price>,
}

/// Complete real estate listing information.
///
/// This is the main structure containing all details about a property listing
/// including address, characteristics, pricing, and localized content.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    /// Property address
    pub address: Address,
    /// Property categories (apartment, house, etc.)
    pub categories: Vec<Category>,
    /// Physical characteristics (size, rooms)
    pub characteristics: Characteristics,
    /// Unique listing identifier
    pub id: String,
    /// Lister contact information
    pub lister: Lister,
    /// Multi-language content
    pub localization: Localization,
    /// Type of offer (rent, buy)
    pub offer_type: OfferType,
    /// Pricing information
    pub prices: Prices,
}

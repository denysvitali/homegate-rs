use serde::{Deserialize, Serialize};

use crate::models::address::Address;
use crate::models::realestate::OfferType;

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Characteristics {
    pub living_space: u32,
    pub number_of_rooms: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Lister {
    pub phone: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "type")]
    pub t: String,
    pub url: String,
    pub file: String,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationEntryText {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationEntry {
    pub attachments: Vec<Attachment>,
    pub text: LocalizationEntryText,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Localization {
    pub de: Option<LocalizationEntry>,
    pub primary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PriceInterval {
    MONTH
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub interval: Option<PriceInterval>,
    pub net: Option<u32>,
    pub gross: Option<u32>,
    pub extra: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Currency {
    CHF
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    pub rent: Option<Price>,
    pub currency: Currency,
    pub buy: Option<Price>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub address: Address,
    pub categories: Vec<Category>,
    pub characteristics: Characteristics,
    pub id: String,
    pub lister: Lister,
    pub localization: Localization,
    pub offer_type: OfferType,
    pub prices: Prices,

}
use serde::{Serialize, Deserialize};


use crate::models::listing::{Listing};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RealEstate {
    pub id: String,
    pub listing: Listing,

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListingTypeWrapper {
    #[serde(rename = "type")]
    pub t: ListingType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OfferType {
    RENT
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PriceUnit {
    MONTHLY,
    M2YEARLY,
    WEEKLY
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ListingType {
    PREMIUM,
    TOP,
    STANDARD,
}
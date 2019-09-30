use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstate {
    #[serde(rename = "advertisementId")]
    pub advertisement_id: i64,
    pub title: String,
    pub street: Option<String>,
    pub zip: String,
    pub city: String,
    #[serde(rename = "geoLocation")]
    pub geo_location: String,

    #[serde(rename = "offerType")]
    pub offer_type: OfferType,

    #[serde(rename = "numberRooms")]
    pub number_rooms: Option<f32>,
    
    #[serde(rename = "surfaceLiving")]
    pub surface_living: Option<i32>,
    pub currency: String,
    
    #[serde(rename = "sellingPrice")]
    pub selling_price: Option<i32>,

    #[serde(rename = "priceUnit")]
    pub price_unit: PriceUnit,

    pub pictures: Vec<String>,

    #[serde(rename = "objectTypeLabel")]
    pub object_type_labe: String,

    #[serde(rename = "listingType")]
    pub listing_type: ListingType,

    #[serde(rename = "contactPerson")]
    pub contact_person: Option<String>,

    #[serde(rename = "contactPhone")]
    pub contact_phone: Option<String>,

    #[serde(rename = "interestedFormType")]
    pub interested_form_type: i32,

    #[serde(rename = "externalUrls")]
    pub external_urls: Vec<HashMap<String, String>>,

    #[serde(rename = "picFileName1")]
    pub pic_file_name_1: Option<String>,

}

#[derive(Serialize, Deserialize, Debug)]
pub enum OfferType {
    RENT
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PriceUnit {
    MONTHLY,
    M2YEARLY,
    WEEKLY
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ListingType {
    PREMIUM,
    TOP,
    STANDARD,
}
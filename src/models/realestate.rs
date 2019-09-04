use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstate {
    #[serde(rename = "advertisementId")]
    advertisement_id: i64,
    title: String,
    street: Option<String>,
    zip: String,
    city: String,
    #[serde(rename = "geoLocation")]
    geo_location: String,
    #[serde(rename = "offerType")]
    offer_type: OfferType,
    #[serde(rename = "numberRooms")]
    number_rooms: Option<f32>,
    #[serde(rename = "surfaceLiving")]
    surface_living: Option<i32>,
    currency: String,
    #[serde(rename = "sellingPrice")]
    selling_price: Option<i32>,
    #[serde(rename = "priceUnit")]
    price_unit: PriceUnit,
    pictures: Vec<String>,
    #[serde(rename = "objectTypeLabel")]
    object_type_labe: String,
    #[serde(rename = "listingType")]
    listing_type: ListingType,
    #[serde(rename = "contactPerson")]
    contact_person: Option<String>,
    #[serde(rename = "contactPhone")]
    contact_phone: Option<String>,
    #[serde(rename = "interestedFormType")]
    interested_form_type: i32,
    #[serde(rename = "externalUrls")]
    external_urls: Vec<HashMap<String, String>>,
    #[serde(rename = "picFileName1")]
    pic_file_name_1: Option<String>,

}

#[derive(Serialize, Deserialize, Debug)]
pub enum OfferType {
    RENT
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PriceUnit {
    MONTHLY,
    M2YEARLY,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ListingType {
    PREMIUM,
    TOP,
    STANDARD,
}
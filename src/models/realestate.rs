use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RealEstate {
    advertisementId: i64,
    title: String,
    street: Option<String>,
    zip: String,
    city: String,
    geoLocation: String,
    offerType: OfferType,
    numberRooms: Option<f32>,
    surfaceLiving: Option<i32>,
    currency: String,
    sellingPrice: Option<i32>,
    priceUnit: PriceUnit,
    pictures: Vec<String>,
    objectTypeLabel: String,
    listingType: ListingType,
    contactPerson: Option<String>,
    contactPhone: Option<String>,
    interestedFormType: i32,
    externalUrls: Vec<HashMap<String, String>>,
    picFileName1: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OfferType {
    RENT
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PriceUnit {
    MONTHLY, M2YEARLY
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ListingType {
    PREMIUM, TOP, STANDARD
}
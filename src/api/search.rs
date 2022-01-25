use reqwest::{Response, Url};
use serde::{Serialize, Deserialize};


use crate::api::BACKEND_URL;
use crate::api::request::{get_url, post_url};
use crate::models::paginated::Paginated;
use crate::models::realestate::{OfferType, RealEstate};

#[derive(Serialize, Deserialize)]
struct FromTo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct Location {
    pub latitude: f32,
    pub longitude: f32,
    pub radius: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Query<'se> {
    pub categories: Vec<&'se str>,
    pub exclude_categories: Vec<&'se str>,
    pub living_space: FromTo,
    pub location: Location,
    pub monthly_rent: FromTo,
    pub number_of_rooms: FromTo,
    pub offer_type: OfferType,
}

#[derive(Serialize, Deserialize)]
struct GeoCoordsTemplate {
    pub latitude: bool,
    pub longitude: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddressTemplate {
    pub country: bool,
    pub geo_coordinates: GeoCoordsTemplate,
    pub locality: bool,
    pub post_office_box_number: bool,
    pub postal_code: bool,
    pub region: bool,
    pub street: bool,
    pub street_addition: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CharacteristicsTemplate {
    pub living_space: bool,
    pub lot_size: bool,
    pub number_of_rooms: bool,
    pub single_floor_space: bool,
    pub total_floor_space: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListerTemplate {
    pub logo_url: bool,
    pub phone: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LocaleTextTemplate {
    pub title: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LocaleUrlsTemplate {
    #[serde(rename = "type")]
    pub t: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LocaleTemplate {
    pub attachments: bool,
    pub text: LocaleTextTemplate,
    pub urls: LocaleUrlsTemplate,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocalizationTemplate {
    pub de: LocaleTemplate,
    pub en: LocaleTemplate,
    pub fr: LocaleTemplate,
    pub it: LocaleTemplate,
    pub primary: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListingTemplate {
    pub address: AddressTemplate,
    pub categories: bool,
    pub characteristics: CharacteristicsTemplate,
    pub id: bool,
    pub lister: ListerTemplate,
    pub localization: LocalizationTemplate,
    pub offer_type: bool,
    pub prices: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResultTemplate {
    pub id: bool,
    pub lister_branding: bool,
    pub listing: ListingTemplate,
    pub listing_type: bool,
    pub remote_viewing: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchRequest<'se> {
    pub from: i32,
    pub query: Query<'se>,
    pub result_template: ResultTemplate,
    pub size: i32,
    pub sort_by: &'se str,
    pub sort_direction: &'se str,
    pub track_total_hits: bool,
}

pub async fn search(location: &str, radius: i32) -> Result<Paginated<RealEstate>, reqwest::Error> {
    let mut url: Url = Url::parse(&format!("{}{}", BACKEND_URL, "/search/listings")).unwrap();

    let lt = LocaleTemplate {
        urls: LocaleUrlsTemplate {
            t: true
        },
        attachments: true,
        text: LocaleTextTemplate {
            title: true,
        },
    };

    let search_request = SearchRequest {
        from: 0,
        query: Query {
            categories: Vec::from(vec![
                "APARTMENT",
                "MAISONETTE",
                "DUPLEX",
                "ATTIC_FLAT",
                "ROOF_FLAT",
                "STUDIO",
                "SINGLE_ROOM",
                "TERRACE_FLAT",
                "BACHELOR_FLAT",
                "LOFT",
                "ATTIC",
                "ROW_HOUSE",
                "BIFAMILIAR_HOUSE",
                "TERRACE_HOUSE",
                "VILLA",
                "FARM_HOUSE",
                "CAVE_HOUSE",
                "CASTLE",
                "GRANNY_FLAT",
                "CHALET",
                "RUSTICO",
                "SINGLE_HOUSE",
                "HOBBY_ROOM",
                "CELLAR_COMPARTMENT",
                "ATTIC_COMPARTMENT",
            ]),
            exclude_categories: Vec::from(vec![
                "FURNISHED_FLAT"
            ]),
            living_space: FromTo { from: Some(60), to: None },
            location: Location {
                latitude: 47.36660529240991,
                longitude: 8.541818987578154,
                radius: 1245,
            },
            monthly_rent: FromTo { from: Some(500), to: None },
            number_of_rooms: FromTo {
                from: Some(2), to: None
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
                    de: lt.clone(),
                    en: lt.clone(),
                    fr: lt.clone(),
                    it: lt.clone(),
                    primary: true,
                },
                offer_type: true,
                prices: true,
            },
            listing_type: true,
            remote_viewing: true,
        },
        size: 0,
        sort_by: "listingType",
        sort_direction: "desc",
        track_total_hits: true,
    };
    let search_request_json = serde_json::to_string(&search_request).unwrap();

    println!("json={}", search_request_json);

    let resp: Response = post_url(url, &search_request_json).await?;
    let resp_text = resp.text().await?;
    let r: Paginated<RealEstate> = parse_search_result(&resp_text);
    Ok(r)
}

pub fn parse_search_result(str: &str) -> Paginated<RealEstate> {
    serde_json::from_str(str).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::api::search::{parse_search_result, search};

    #[tokio::test]
    pub async fn search_apartment() {
        let paginated_result = search("Zurich", 10000).await;
        assert!(paginated_result.is_ok());

        let pr = paginated_result.unwrap();
        println!("{:?}", pr);
    }

    #[test]
    pub fn parse_json() {
        let file = fs::read_to_string("./resources/test/search.json").unwrap();
        let paginated_result = parse_search_result(&file);

        assert!(paginated_result.result_count > 0)
    }
}
/// Unit tests for search module
///
/// Tests search request creation, validation, and response handling
use homegate::api::search::{default_search, FromTo, FromToFloat, Location, Query, SearchRequest};
use homegate::models::listing::Category;
use homegate::models::realestate::OfferType;
use std::fs;

// Test constants
const ZURICH_LATLNG: (f32, f32) = (47.36667, 8.55);

#[test]
fn test_create_json() {
    // Test that default_search creates valid JSON (from existing test)
    let req = default_search();
    let json_str = serde_json::to_string(&req).unwrap();
    let f_json = fs::read_to_string("./resources/test/request-1.json").unwrap();

    let decoded_json: SearchRequest = serde_json::from_str(f_json.as_str()).unwrap();
    assert_eq!(decoded_json, req);
}

#[test]
fn test_default_search_structure() {
    let req = default_search();

    // Verify basic structure
    assert_eq!(req.from, 0);
    assert_eq!(req.size, 20);
    assert_eq!(req.sort_by, "listingType");
    assert_eq!(req.sort_direction, "desc");
    assert!(req.track_total_hits);
}

#[test]
fn test_default_search_query_fields() {
    let req = default_search();

    // Verify query structure
    assert_eq!(req.query.offer_type, OfferType::RENT);
    assert!(!req.query.categories.is_empty());
    assert_eq!(req.query.exclude_categories.len(), 1);
}

#[test]
fn test_default_search_has_categories() {
    let req = default_search();

    // Verify categories are included
    assert!(req.query.categories.len() > 0);
    assert!(req
        .query
        .categories
        .contains(&Category::Apartment.to_string()));
    assert!(req.query.categories.contains(&Category::Studio.to_string()));
}

#[test]
fn test_default_search_excludes_furnished() {
    let req = default_search();

    // Verify FURNISHED_FLAT is excluded
    assert_eq!(req.query.exclude_categories.len(), 1);
    assert_eq!(
        req.query.exclude_categories[0],
        Category::FurnishedFlat.to_string()
    );
}

#[test]
fn test_default_search_price_range() {
    let req = default_search();

    // Verify price range
    assert_eq!(req.query.monthly_rent.from, Some(500));
    assert_eq!(req.query.monthly_rent.to, None);
}

#[test]
fn test_default_search_living_space() {
    let req = default_search();

    // Verify living space requirements
    assert_eq!(req.query.living_space.from, Some(60));
    assert_eq!(req.query.living_space.to, None);
}

#[test]
fn test_default_search_number_of_rooms() {
    let req = default_search();

    // Verify room requirements
    assert_eq!(req.query.number_of_rooms.from, Some(2.0));
    assert_eq!(req.query.number_of_rooms.to, None);
}

#[test]
fn test_location_creation() {
    let location = Location {
        latitude: ZURICH_LATLNG.0,
        longitude: ZURICH_LATLNG.1,
        radius: 1000,
    };

    assert_eq!(location.latitude, 47.36667);
    assert_eq!(location.longitude, 8.55);
    assert_eq!(location.radius, 1000);
}

#[test]
fn test_location_serialization() {
    let location = Location {
        latitude: 47.36667,
        longitude: 8.55,
        radius: 1000,
    };

    let json = serde_json::to_string(&location).unwrap();
    assert!(json.contains("latitude"));
    assert!(json.contains("longitude"));
    assert!(json.contains("radius"));
}

#[test]
fn test_location_deserialization() {
    let json = r#"{"latitude":47.36667,"longitude":8.55,"radius":1000}"#;
    let location: Location = serde_json::from_str(json).unwrap();

    assert_eq!(location.latitude, 47.36667);
    assert_eq!(location.longitude, 8.55);
    assert_eq!(location.radius, 1000);
}

#[test]
fn test_from_to_both_values() {
    let from_to = FromTo {
        from: Some(100),
        to: Some(200),
    };

    assert_eq!(from_to.from, Some(100));
    assert_eq!(from_to.to, Some(200));
}

#[test]
fn test_from_to_only_from() {
    let from_to = FromTo {
        from: Some(100),
        to: None,
    };

    assert_eq!(from_to.from, Some(100));
    assert_eq!(from_to.to, None);
}

#[test]
fn test_from_to_serialization_skips_none() {
    let from_to = FromTo {
        from: Some(100),
        to: None,
    };

    let json = serde_json::to_string(&from_to).unwrap();
    assert!(json.contains("from"));
    assert!(!json.contains("to"));
}

#[test]
fn test_search_request_serialization() {
    let req = default_search();
    let json = serde_json::to_string(&req).unwrap();

    // Verify it can be serialized
    assert!(!json.is_empty());

    // Verify key fields are present
    assert!(json.contains("query"));
    assert!(json.contains("resultTemplate"));
    assert!(json.contains("sortBy"));
}

#[test]
fn test_search_request_roundtrip() {
    let req = default_search();
    let json = serde_json::to_string(&req).unwrap();
    let deserialized: SearchRequest = serde_json::from_str(&json).unwrap();

    assert_eq!(req, deserialized);
}

// Mark live API test as ignored
#[tokio::test]
#[ignore]
async fn search_apartment() {
    use homegate::api::search::search;

    let paginated_result = search(&Location {
        latitude: ZURICH_LATLNG.0,
        longitude: ZURICH_LATLNG.1,
        radius: 1000,
    })
    .await;

    assert!(paginated_result.is_ok());

    let pr = paginated_result.unwrap();
    println!("{:?}", pr);
}

#[test]
fn test_result_template_structure() {
    let req = default_search();

    // Verify result template requests required fields
    assert!(req.result_template.id);
    assert!(req.result_template.lister_branding);
    assert!(req.result_template.listing_type);
    assert!(req.result_template.remote_viewing);
}

#[test]
fn test_listing_template_has_address() {
    let req = default_search();

    // Verify address fields are requested
    assert!(req.result_template.listing.address.country);
    assert!(req.result_template.listing.address.locality);
    assert!(req.result_template.listing.address.postal_code);
    assert!(req.result_template.listing.address.geo_coordinates.latitude);
    assert!(
        req.result_template
            .listing
            .address
            .geo_coordinates
            .longitude
    );
}

#[test]
fn test_listing_template_has_characteristics() {
    let req = default_search();

    // Verify characteristics fields are requested
    assert!(req.result_template.listing.characteristics.living_space);
    assert!(req.result_template.listing.characteristics.number_of_rooms);
}

#[test]
fn test_listing_template_has_localization() {
    let req = default_search();

    // Verify localization is requested for all languages
    assert!(req.result_template.listing.localization.primary);
    assert!(req.result_template.listing.localization.de.attachments);
    assert!(req.result_template.listing.localization.de.text.title);
    assert!(req.result_template.listing.localization.en.attachments);
    assert!(req.result_template.listing.localization.fr.attachments);
    assert!(req.result_template.listing.localization.it.attachments);
}

#[test]
fn test_query_clone() {
    let query = Query {
        categories: vec!["APARTMENT".to_string()],
        exclude_categories: vec![],
        living_space: FromTo {
            from: Some(50),
            to: None,
        },
        location: Location {
            latitude: 47.36667,
            longitude: 8.55,
            radius: 1000,
        },
        monthly_rent: FromTo {
            from: Some(500),
            to: Some(2000),
        },
        number_of_rooms: FromToFloat {
            from: Some(2.0),
            to: None,
        },
        offer_type: OfferType::RENT,
    };

    let cloned = query.clone();
    assert_eq!(query, cloned);
}

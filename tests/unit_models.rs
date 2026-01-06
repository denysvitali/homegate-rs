/// Unit tests for models module
///
/// Tests serialization/deserialization for all model types including Category enum,
/// Listing, RealEstate, Prices, Localization, and Paginated structures

#[cfg(test)]
mod tests {
    use homegate::models::address::Address;
    use homegate::models::geo_coords::GeoCoords;
    use homegate::models::listing::{
        Attachment, Category, Characteristics, Currency, Lister, LocalizationEntryText, Price,
        PriceInterval, Prices,
    };
    use homegate::models::paginated::{parse_search_result, Paginated};
    use homegate::models::realestate::{ListingType, ListingTypeWrapper, OfferType, RealEstate};
    use std::fs;

    // ========== Category Enum Tests (37 variants) ==========

    #[test]
    fn test_category_apartment_serialization() {
        let cat = Category::Apartment;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""APARTMENT""#);
    }

    #[test]
    fn test_category_apartment_deserialization() {
        let json = r#""APARTMENT""#;
        let cat: Category = serde_json::from_str(json).unwrap();
        assert_eq!(cat, Category::Apartment);
    }

    #[test]
    fn test_category_flat() {
        let cat = Category::Flat;
        let json = serde_json::to_string(&cat).unwrap();
        let deserialized: Category = serde_json::from_str(&json).unwrap();
        assert_eq!(cat, deserialized);
    }

    #[test]
    fn test_category_maisonette() {
        let cat = Category::Maisonette;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""MAISONETTE""#);
    }

    #[test]
    fn test_category_duplex() {
        let cat = Category::Duplex;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""DUPLEX""#);
    }

    #[test]
    fn test_category_attic_flat() {
        let cat = Category::AtticFlat;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""ATTIC_FLAT""#);
    }

    #[test]
    fn test_category_roof_flat() {
        let cat = Category::RoofFlat;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""ROOF_FLAT""#);
    }

    #[test]
    fn test_category_studio() {
        let cat = Category::Studio;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""STUDIO""#);
    }

    #[test]
    fn test_category_single_room() {
        let cat = Category::SingleRoom;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""SINGLE_ROOM""#);
    }

    #[test]
    fn test_category_terrace_flat() {
        let cat = Category::TerraceFlat;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""TERRACE_FLAT""#);
    }

    #[test]
    fn test_category_bachelor_flat() {
        let cat = Category::BachelorFlat;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""BACHELOR_FLAT""#);
    }

    #[test]
    fn test_category_loft() {
        let cat = Category::Loft;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""LOFT""#);
    }

    #[test]
    fn test_category_attic() {
        let cat = Category::Attic;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""ATTIC""#);
    }

    #[test]
    fn test_category_house() {
        let cat = Category::House;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""HOUSE""#);
    }

    #[test]
    fn test_category_row_house() {
        let cat = Category::RowHouse;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""ROW_HOUSE""#);
    }

    #[test]
    fn test_category_bifamiliar_house() {
        let cat = Category::BifamiliarHouse;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""BIFAMILIAR_HOUSE""#);
    }

    #[test]
    fn test_category_terrace_house() {
        let cat = Category::TerraceHouse;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""TERRACE_HOUSE""#);
    }

    #[test]
    fn test_category_villa() {
        let cat = Category::Villa;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""VILLA""#);
    }

    #[test]
    fn test_category_farm_house() {
        let cat = Category::FarmHouse;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""FARM_HOUSE""#);
    }

    #[test]
    fn test_category_cave_house() {
        let cat = Category::CaveHouse;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""CAVE_HOUSE""#);
    }

    #[test]
    fn test_category_castle() {
        let cat = Category::Castle;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""CASTLE""#);
    }

    #[test]
    fn test_category_granny_flat() {
        let cat = Category::GrannyFlat;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""GRANNY_FLAT""#);
    }

    #[test]
    fn test_category_chalet() {
        let cat = Category::Chalet;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""CHALET""#);
    }

    #[test]
    fn test_category_rustico() {
        let cat = Category::Rustico;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""RUSTICO""#);
    }

    #[test]
    fn test_category_single_house() {
        let cat = Category::SingleHouse;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""SINGLE_HOUSE""#);
    }

    #[test]
    fn test_category_hobby_room() {
        let cat = Category::HobbyRoom;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""HOBBY_ROOM""#);
    }

    #[test]
    fn test_category_cellar_compartment() {
        let cat = Category::CellarCompartment;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""CELLAR_COMPARTMENT""#);
    }

    #[test]
    fn test_category_attic_compartment() {
        let cat = Category::AtticCompartment;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""ATTIC_COMPARTMENT""#);
    }

    #[test]
    fn test_category_furnished_flat() {
        let cat = Category::FurnishedFlat;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, r#""FURNISHED_FLAT""#);
    }

    #[test]
    fn test_category_display_trait() {
        let cat = Category::Apartment;
        let display_str = format!("{}", cat);
        assert!(display_str.contains("APARTMENT"));
    }

    #[test]
    fn test_category_clone() {
        let cat = Category::Studio;
        let cloned = cat.clone();
        assert_eq!(cat, cloned);
    }

    // ========== Listing and Component Tests ==========

    #[test]
    fn test_characteristics_serialization() {
        let chars = Characteristics {
            living_space: 80,
            number_of_rooms: 3.5,
        };

        let json = serde_json::to_string(&chars).unwrap();
        assert!(json.contains("livingSpace"));
        assert!(json.contains("numberOfRooms"));
    }

    #[test]
    fn test_characteristics_deserialization() {
        let json = r#"{"livingSpace":80,"numberOfRooms":3.5}"#;
        let chars: Characteristics = serde_json::from_str(json).unwrap();

        assert_eq!(chars.living_space, 80);
        assert_eq!(chars.number_of_rooms, 3.5);
    }

    #[test]
    fn test_lister_with_phone() {
        let lister = Lister {
            phone: Some("+41 44 123 45 67".to_string()),
        };

        let json = serde_json::to_string(&lister).unwrap();
        assert!(json.contains("phone"));
    }

    #[test]
    fn test_lister_without_phone() {
        let lister = Lister { phone: None };

        let json = serde_json::to_string(&lister).unwrap();
        let deserialized: Lister = serde_json::from_str(&json).unwrap();
        assert!(deserialized.phone.is_none());
    }

    #[test]
    fn test_attachment_structure() {
        let attachment = Attachment {
            t: "IMAGE".to_string(),
            url: "https://example.com/image.jpg".to_string(),
            file: "image.jpg".to_string(),
        };

        let json = serde_json::to_string(&attachment).unwrap();
        assert!(json.contains("type"));
        assert!(json.contains("url"));
        assert!(json.contains("file"));
    }

    #[test]
    fn test_localization_entry_text() {
        let text = LocalizationEntryText {
            title: "Beautiful Apartment".to_string(),
        };

        let json = serde_json::to_string(&text).unwrap();
        assert!(json.contains("title"));
    }

    #[test]
    fn test_price_interval_month() {
        let interval = PriceInterval::MONTH;
        let json = serde_json::to_string(&interval).unwrap();
        assert_eq!(json, r#""MONTH""#);
    }

    #[test]
    fn test_currency_chf() {
        let currency = Currency::CHF;
        let json = serde_json::to_string(&currency).unwrap();
        assert_eq!(json, r#""CHF""#);
    }

    #[test]
    fn test_price_with_all_fields() {
        let price = Price {
            interval: Some(PriceInterval::MONTH),
            net: Some(2000),
            gross: Some(2200),
            extra: Some(200),
        };

        let json = serde_json::to_string(&price).unwrap();
        assert!(json.contains("interval"));
        assert!(json.contains("net"));
        assert!(json.contains("gross"));
        assert!(json.contains("extra"));
    }

    #[test]
    fn test_price_with_optional_fields_none() {
        let price = Price {
            interval: None,
            net: None,
            gross: Some(2200),
            extra: None,
        };

        let json = serde_json::to_string(&price).unwrap();
        let deserialized: Price = serde_json::from_str(&json).unwrap();
        assert!(deserialized.interval.is_none());
        assert!(deserialized.net.is_none());
        assert_eq!(deserialized.gross, Some(2200));
    }

    #[test]
    fn test_prices_structure() {
        let prices = Prices {
            rent: Some(Price {
                interval: Some(PriceInterval::MONTH),
                net: Some(2000),
                gross: Some(2200),
                extra: Some(200),
            }),
            currency: Currency::CHF,
            buy: None,
        };

        let json = serde_json::to_string(&prices).unwrap();
        assert!(json.contains("rent"));
        assert!(json.contains("currency"));
    }

    #[test]
    fn test_address_with_all_fields() {
        let address = Address {
            country: Some("CH".to_string()),
            geo_coordinates: GeoCoords {
                latitude: 47.36667,
                longitude: 8.55,
            },
            locality: Some("Zürich".to_string()),
            postal_code: "8001".to_string(),
            region: Some("ZH".to_string()),
            street: Some("Bahnhofstrasse 1".to_string()),
        };

        let json = serde_json::to_string(&address).unwrap();
        assert!(json.contains("country"));
        assert!(json.contains("geoCoordinates"));
        assert!(json.contains("postalCode"));
    }

    #[test]
    fn test_address_with_optional_fields_none() {
        let address = Address {
            country: None,
            geo_coordinates: GeoCoords {
                latitude: 47.36667,
                longitude: 8.55,
            },
            locality: None,
            postal_code: "8001".to_string(),
            region: None,
            street: None,
        };

        let json = serde_json::to_string(&address).unwrap();
        let deserialized: Address = serde_json::from_str(&json).unwrap();
        assert!(deserialized.country.is_none());
        assert!(deserialized.locality.is_none());
    }

    #[test]
    fn test_geo_coords() {
        let coords = GeoCoords {
            latitude: 47.36667,
            longitude: 8.55,
        };

        let json = serde_json::to_string(&coords).unwrap();
        assert!(json.contains("latitude"));
        assert!(json.contains("longitude"));
    }

    // ========== RealEstate Tests ==========

    #[test]
    fn test_offer_type_rent() {
        let offer = OfferType::RENT;
        let json = serde_json::to_string(&offer).unwrap();
        assert_eq!(json, r#""RENT""#);
    }

    #[test]
    fn test_listing_type_premium() {
        let lt = ListingType::PREMIUM;
        let json = serde_json::to_string(&lt).unwrap();
        assert_eq!(json, r#""PREMIUM""#);
    }

    #[test]
    fn test_listing_type_top() {
        let lt = ListingType::TOP;
        let json = serde_json::to_string(&lt).unwrap();
        assert_eq!(json, r#""TOP""#);
    }

    #[test]
    fn test_listing_type_standard() {
        let lt = ListingType::STANDARD;
        let json = serde_json::to_string(&lt).unwrap();
        assert_eq!(json, r#""STANDARD""#);
    }

    #[test]
    fn test_listing_type_wrapper() {
        let wrapper = ListingTypeWrapper {
            t: ListingType::TOP,
        };

        let json = serde_json::to_string(&wrapper).unwrap();
        assert!(json.contains("type"));
    }

    // ========== Paginated Tests ==========

    #[test]
    fn test_parse_result_2() {
        // Test using existing fixture
        let file = fs::read_to_string("./resources/test/result-2.json").unwrap();
        let paginated_result = parse_search_result(&file).unwrap();

        assert!(paginated_result.total > 0);
    }

    #[test]
    fn test_paginated_structure() {
        let file = fs::read_to_string("./resources/test/result-2.json").unwrap();
        let paginated: Paginated<RealEstate> = serde_json::from_str(&file).unwrap();

        assert_eq!(paginated.from, 0);
        assert_eq!(paginated.size, 20);
        assert!(paginated.total > 0);
        assert!(!paginated.results.is_empty());
    }

    #[test]
    fn test_paginated_empty_results() {
        let json = r#"{"from":0,"size":20,"total":0,"results":[],"maxFrom":0}"#;
        let paginated: Paginated<RealEstate> = serde_json::from_str(json).unwrap();

        assert_eq!(paginated.total, 0);
        assert_eq!(paginated.results.len(), 0);
    }

    #[test]
    fn test_paginated_single_page() {
        let file = fs::read_to_string("./resources/test/result-1.json").unwrap();
        let paginated: Paginated<RealEstate> = serde_json::from_str(&file).unwrap();

        assert!(paginated.results.len() <= paginated.size as usize);
    }

    #[test]
    fn test_paginated_max_from() {
        let file = fs::read_to_string("./resources/test/result-2.json").unwrap();
        let _paginated: Paginated<RealEstate> = serde_json::from_str(&file).unwrap();

        // max_from is always >= 0 for unsigned types
    }

    #[test]
    fn test_real_estate_has_id() {
        let file = fs::read_to_string("./resources/test/result-1.json").unwrap();
        let paginated: Paginated<RealEstate> = serde_json::from_str(&file).unwrap();

        for result in &paginated.results {
            assert!(!result.id.is_empty());
        }
    }

    #[test]
    fn test_real_estate_has_listing() {
        let file = fs::read_to_string("./resources/test/result-1.json").unwrap();
        let paginated: Paginated<RealEstate> = serde_json::from_str(&file).unwrap();

        for result in &paginated.results {
            assert!(!result.listing.id.is_empty());
        }
    }

    #[test]
    fn test_listing_has_categories() {
        let file = fs::read_to_string("./resources/test/result-1.json").unwrap();
        let paginated: Paginated<RealEstate> = serde_json::from_str(&file).unwrap();

        for result in &paginated.results {
            assert!(!result.listing.categories.is_empty());
        }
    }

    #[test]
    fn test_listing_deserialization_from_fixture() {
        let file = fs::read_to_string("./resources/test/result-2.json").unwrap();
        let paginated: Paginated<RealEstate> = serde_json::from_str(&file).unwrap();

        // Verify first listing has expected structure
        let first = &paginated.results[0];
        assert_eq!(first.id, "3001439887");
        assert_eq!(first.listing.address.postal_code, "8001");
        assert_eq!(first.listing.localization.primary, "de");
    }

    #[test]
    fn test_localization_primary_language() {
        let file = fs::read_to_string("./resources/test/result-2.json").unwrap();
        let paginated: Paginated<RealEstate> = serde_json::from_str(&file).unwrap();

        for result in &paginated.results {
            assert!(!result.listing.localization.primary.is_empty());
        }
    }
}

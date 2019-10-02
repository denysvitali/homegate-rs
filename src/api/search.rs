use crate::api::request::get_url;
use crate::api::BACKEND_URL;
use crate::models::realestate::RealEstate;
use crate::models::paginated::Paginated;
use reqwest::Url;

pub fn search(location: &str, radius: i32) -> Result<Paginated<RealEstate>, reqwest::Error> {
    let mut url : Url = Url::parse(&format!("{}{}", BACKEND_URL, "/rs/real-estates")).unwrap();
    let mut fields : Vec<&str> = Vec::new();
    fields.push("advertisementId");
    fields.push("geoLocation");
    fields.push("street");
    fields.push("zip");
    fields.push("city");
    fields.push("title");
    fields.push("numberRooms");
    fields.push("sellingPrice");
    fields.push("surfaceLiving");
    fields.push("objectTypeLabel");
    fields.push("currency");
    fields.push("listingType");
    fields.push("contactPerson");
    fields.push("contactPhone");
    fields.push("interestedFormType");
    fields.push("offerType");
    fields.push("priceUnit");
    fields.push("externalUrls");
    fields.push("primeOffer");
    fields.push("pictures");

    {
        url.query_pairs_mut().clear()
            .append_pair("cht","rentflat")
            .append_pair("loc", location)
            .append_pair("nrs", &radius.to_string())
            .append_pair("ver", "3")
            .append_pair("lan", "en")
            .append_pair("rfi", &fields.join(","));
    }

    let resp =  get_url(url)?.text()?;
    let r : Paginated<RealEstate> = parse_search_result(&resp);
    Ok(r)
}

pub fn parse_search_result(str: &str) -> Paginated<RealEstate> {
    serde_json::from_str(str).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::api::search::{search, parse_search_result};
    use std::fs;

    #[test]
    pub fn search_apartment() {
        let paginated_result = search("Zurich", 10000);
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
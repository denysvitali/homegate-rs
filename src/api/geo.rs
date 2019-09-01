use std::vec::Vec;
use crate::models::location::Location;
use crate::api::request;

pub fn get_areas() -> Vec<Location> {
    let r = request::get("/rs/geo-areas?lan=en");
    match r {
        Ok(mut result) => {
            let txt = result.text().unwrap();
            let res : Vec<Location> = serde_json::from_str(&txt).unwrap();
            res
        }
        _  => {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn get_areas(){
        let v = crate::api::geo::get_areas();
        assert_ne!(0, v.len());
    }
}
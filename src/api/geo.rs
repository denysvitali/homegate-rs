use std::vec::Vec;
use crate::models::location::Location;
use crate::api::request;

pub async fn get_areas() -> Vec<Location> {
    let r = request::get("/rs/geo-areas?lan=en").await;
    match r {
        Ok(result) => {
            let text = result.text().await;
            return match text {
                Ok(txt) => {
                    let res: Vec<Location> = serde_json::from_str(&txt).unwrap();
                    res
                }
                _ => {
                    Vec::new()
                }
            }
        }
        _  => {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    pub async fn get_areas(){
        let v = crate::api::geo::get_areas().await;
        assert_ne!(0, v.len());
    }
}
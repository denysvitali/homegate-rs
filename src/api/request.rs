use chrono::Utc;
use reqwest::{Client, ClientBuilder, Error, Response, Url};
use reqwest::header;
use reqwest::header::HeaderValue;

use crate::api::{API_PASSWORD, API_USERNAME, BACKEND_URL, USER_AGENT};
use crate::api::app_id::{app_version, calculate_app_id};

fn build_client<'a>() -> Result<Client, Error> {
    let client_builder: ClientBuilder = reqwest::Client::builder();
    let mut default_headers = header::HeaderMap::new();

    let key = base64::encode(format!("{}:{}", API_USERNAME, API_PASSWORD));
    let app_id = calculate_app_id(&Utc::now().naive_utc());

    const APPL_JSON: &str = "application/json";

    default_headers.insert(header::AUTHORIZATION, HeaderValue::from_str(&format!("Basic {}", key)).unwrap());
    default_headers.insert(header::ACCEPT, HeaderValue::from_static(APPL_JSON));
    default_headers.insert("X-App-Id", app_id.parse().unwrap());
    default_headers.insert("X-App-Version", app_version().parse().unwrap());
    default_headers.insert(header::USER_AGENT, HeaderValue::from_static(USER_AGENT)); // Not a typo!
    default_headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(APPL_JSON));
    return client_builder.default_headers(default_headers).build();
}

pub async fn get(path: &str) -> Result<Response, Error> {
    let url = Url::parse(&format!("{}{}",
                                  BACKEND_URL,
                                  path));

    let c: Client = build_client().unwrap();
    let req = c.get(url.unwrap()).build().unwrap();
    return c.execute(req).await;
}

pub async fn post_url(url: Url, body: &str) -> Result<Response, Error> {
    let c: Client = build_client().unwrap();
    let req = c.post(url).body(body.to_string()).build().unwrap();
    return c.execute(req).await;
}

pub async fn get_url(url: Url) -> Result<Response, Error> {
    let c: Client = build_client()?;
    let req_b = c.get(url);
    let req = req_b.build()?;
    c.execute(req).await
}

#[cfg(tests)]
pub mod tests {
    use reqwest::{Error, Response, Url};

    use crate::api::request;
    use crate::api::request::get_url;

    #[test]
    pub fn test_request() {
        let r: Result<Response, Error> = request::get("/rs/geo-areas?lan=en");
        match r {
            Ok(mut response) => {
                println!("{:?}", response.text().unwrap());
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
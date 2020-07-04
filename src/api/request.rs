use reqwest::{Client, ClientBuilder, Response, Url, Error};
use reqwest::header::HeaderValue;
use crate::api::{BACKEND_URL, KEY};
use reqwest::header;

fn build_client<'a>() -> Result<Client, Error> {
    let client_builder : ClientBuilder = reqwest::Client::builder();
    let mut default_headers = header::HeaderMap::new();
    default_headers.clear();
    default_headers.insert(header::AUTHORIZATION,
                           HeaderValue::from_str(&format!("{}{}", "Basic ", KEY)).unwrap());
    default_headers.insert(header::ACCEPT,
                           HeaderValue::from_static("application/json"));
    default_headers.insert(header::USER_AGENT,
                           HeaderValue::from_static("okhttp/3.14.1"));
    return client_builder.default_headers(default_headers).build()
}

pub async fn get(path: &str) -> Result<Response, Error>{
    let url = Url::parse(&format!("{}{}",
                                  BACKEND_URL,
                                  path));

    let c : Client = build_client().unwrap();
    let req = c.get(url.unwrap()).build().unwrap();
    return c.execute(req).await;
}

pub async fn get_url(url: Url) -> Result<Response, Error> {
    let c : Client = build_client()?;
    let req_b = c.get(url);
    let req = req_b.build()?;
    c.execute(req).await
}

#[cfg(tests)]
pub mod tests {
    use reqwest::{Response, Error, Url};
    use crate::api::request;
    use crate::api::request::get_url;

    #[test]
    pub fn test_request() {
        let r : Result<Response, Error> = request::get( "/rs/geo-areas?lan=en");
        match r {
            Ok(mut response) => {
                println!("{:?}", response.text().unwrap());
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
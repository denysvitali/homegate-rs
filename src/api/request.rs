use reqwest::{Client, Response, Url, Error};
use reqwest::header::HeaderValue;
use crate::api::{BACKEND_URL, KEY};
use reqwest::header;

fn build_client<'a>() -> Result<Client, Error> {
    let client_builder = reqwest::Client::builder();
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

pub fn get(path: &str) -> Result<Response, Error> {
    let url = Url::parse(&format!("{}{}",
                                  BACKEND_URL,
                                  path));

    let c : Client = build_client()?;
    let req = c.get(url.unwrap()).build()?;
    c.execute(req)
}

pub fn get_url(url: Url) -> Result<Response, Error> {
    let c : Client = build_client()?;
    println!("Client: {:?}", c);
    let reqb = c.get(url);
    let req = reqb.build()?;
    println!("Headers: {:?}", req.headers());
    c.execute(req)
}

#[cfg(tests)]
pub mod tests {
    use reqwest::{Response, Error};
    use crate::api::request;

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
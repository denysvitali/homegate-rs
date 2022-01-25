mod request;
pub mod geo;
pub mod search;
mod app_id;

pub static BACKEND_URL: &'static str = "https://api.homegate.ch";
// pub static BACKEND_URL: &'static str = "http://127.0.0.1:1234";
pub static API_USERNAME: &'static str = "hg_android";
pub static API_PASSWORD: &'static str = "6VcGU6ceCFTk8dFm";
pub static SECRET: [u8; 21] = [65, 66, 117, 84, 90, 114, 99, 84, 71, 75, 78, 52, 65, 119, 106, 72, 101, 100, 51, 72, 106];
pub static USER_AGENT: &str = "hoemgate.ch App Android";
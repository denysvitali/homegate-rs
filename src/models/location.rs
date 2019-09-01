use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    r#type: String,
    typeLabel: String
}
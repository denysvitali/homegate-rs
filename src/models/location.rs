use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    r#type: String,
    #[serde(rename="typeLabel")]
    type_label: String
}
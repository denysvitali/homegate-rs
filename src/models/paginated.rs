use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Paginated<T> {
    #[serde(rename="resultCount")]
    pub result_count: i32,

    pub start: i32,

    #[serde(rename="pageCount")]
    pub page_count: i32,

    #[serde(rename="itemsPerPage")]
    pub items_per_page: i32,

    #[serde(rename="hasNextPage")]
    pub has_next_page: bool,

    #[serde(rename="hasPreviousPage")]
    pub has_previous_page: bool,

    pub items: Vec<T>
}
/*
    {"resultCount":933,"start":0,"page":1,"pageCount":1,"itemsPerPage":10000,"hasNextPage":false,"hasPreviousPage":false,
*/
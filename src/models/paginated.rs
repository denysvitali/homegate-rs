use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Paginated<T> {
    resultCount: i32,
    start: i32,
    pageCount: i32,
    itemsPerPage: i32,
    hasNextPage: bool,
    hasPreviousPage: bool,
    items: Vec<T>
}
/*
    {"resultCount":933,"start":0,"page":1,"pageCount":1,"itemsPerPage":10000,"hasNextPage":false,"hasPreviousPage":false,
*/
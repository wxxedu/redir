use crate::schema::urls;
use proc::datamodel;

#[datamodel(urls)]
pub struct UrlData {
    url: String,
    created_at: chrono::NaiveDateTime,
    num_accesses: i32,
}

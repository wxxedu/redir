use crate::schema::accesses;
use chrono::NaiveDateTime;
use proc::datamodel;

#[datamodel(accesses)]
pub struct Access {
    url_id: i32,
    accessed_at: NaiveDateTime,
    ip: String,
}

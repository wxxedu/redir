use std::fmt::Display;

use chrono::NaiveDateTime;
use database::schema::accesses;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = accesses)]
pub struct Access {
    id: Option<i32>,
    pub url_id: i32,
    pub accessed_at: NaiveDateTime,
    pub ip: String,
}

impl Access {
    /// Creates a new `Access` object.
    pub fn new(url_id: i32, ip: impl AsRef<str>) -> Self {
        Self {
            id: None,
            url_id,
            accessed_at: chrono::Local::now().naive_local(),
            ip: ip.as_ref().to_string(),
        }
    }
}

impl Display for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let accessed_at = self.accessed_at.format("%Y-%m-%d %H:%M:%S");
        write!(f, "Accessed at: {}, IP: {}", accessed_at, self.ip)
    }
}

unsafe impl Send for Access {}
unsafe impl Sync for Access {}

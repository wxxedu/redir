use config::Config;
use database::schema::urls;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = urls)]
pub struct UrlData {
    pub id: Option<i32>,
    pub url: String,
    pub created_at: chrono::NaiveDateTime,
    pub num_accesses: i32,
}

impl UrlData {
    /// Creates a new `UrlData` object.
    pub fn new(url: impl AsRef<str>) -> Self {
        Self {
            id: None,
            url: url.as_ref().to_string(),
            created_at: chrono::Local::now().naive_local(),
            num_accesses: 0,
        }
    }
}

impl Display for UrlData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let created_at = self.created_at.format("%Y-%m-%d %H:%M:%S");
        write!(
            f,
            "Created at: {}, URL: {}, Number of accesses: {}",
            created_at, self.url, self.num_accesses
        )
    }
}

unsafe impl Send for UrlData {}
unsafe impl Sync for UrlData {}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ShortenedUrl {
    pub id: Option<i32>,
    pub url: String,
}

impl ShortenedUrl {
    /// Returns the shortened URL.
    pub fn get_shortened_url(&self, config: impl Config) -> String {
        format!("{}/{}", config.get_base_url(), self.url)
    }

    /// Returns the original URL.
    pub fn get_url(&self) -> &str {
        &self.url
    }
}

unsafe impl Send for ShortenedUrl {}
unsafe impl Sync for ShortenedUrl {}

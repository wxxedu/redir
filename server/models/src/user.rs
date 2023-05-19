use config::Config;
use database::schema::users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::{cmp::min, fmt::Display};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    password: String,
    created_at: chrono::NaiveDateTime,
    salt: String,
}

impl User {
    /// Creates a new `User` object.
    pub fn new(
        config: impl Config,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Self {
        let mut salt = [0u8; 16];
        for i in 0..16 {
            salt[i] = rand::random::<u8>();
        }
        let res = bcrypt::hash_with_salt(
            password.as_ref(),
            config.get_hash_cost(),
            salt,
        )
        .unwrap();
        let password = res.format_for_version(bcrypt::Version::TwoB);
        let salt = res.get_salt();
        Self {
            id: None,
            username: username.as_ref().to_string(),
            password,
            created_at: chrono::Local::now().naive_local(),
            salt,
        }
    }

    /// Returns the user's password.
    pub fn get_password(&self) -> &str {
        &self.password
    }

    /// Returns the user's salt.
    pub fn get_salt(&self) -> &str {
        &self.salt
    }

    /// Hashes the given password with the user's salt.
    pub fn hash_password(&self, password: impl AsRef<str>) -> String {
        if self.salt.bytes().len() != 16 {
            log::warn!("Salt is not 16 bytes long: {}", self.salt);
        }
        let mut salt = [0u8; 16];
        for i in 0..min(16, self.salt.len()) {
            salt[i] = self.salt.as_bytes()[i];
        }
        bcrypt::hash_with_salt(password.as_ref(), bcrypt::DEFAULT_COST, salt)
            .unwrap()
            .format_for_version(bcrypt::Version::TwoB)
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let created_at = self.created_at.format("%Y-%m-%d %H:%M:%S");
        write!(f, "Created at: {}, Username: {}", created_at, self.username)
    }
}

unsafe impl Send for User {}
unsafe impl Sync for User {}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserAuthCredential {
    username: String,
    password: String,
}

/// A user's authentication credentials. This will be used to authenticate
/// users.
impl UserAuthCredential {
    /// Creates a new `UserAuthCredential` object.
    pub fn new(username: impl AsRef<str>, password: impl AsRef<str>) -> Self {
        Self {
            username: username.as_ref().to_string(),
            password: password.as_ref().to_string(),
        }
    }

    /// Returns the username.
    pub fn get_username(&self) -> &str {
        &self.username
    }

    /// Returns the password.
    pub fn get_password(&self) -> &str {
        &self.password
    }
}

unsafe impl Send for UserAuthCredential {}
unsafe impl Sync for UserAuthCredential {}

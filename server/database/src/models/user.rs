use crate::schema::users;
use proc::datamodel;
use std::cmp::min;

pub trait AuthCredential {
    fn get_username(&self) -> &str;
    fn get_password(&self) -> &str;
}

#[datamodel(users)]
pub struct User {
    username: String,
    password: String,
    created_at: chrono::NaiveDateTime,
    salt: String,
}

impl User {
    /// Hashes the given password with the user's salt.
    pub fn hash_password(&self, password: impl AsRef<str>) -> String {
        if self.salt().bytes().len() != 16 {
            log::warn!("Salt is not 16 bytes long: {}", self.salt());
        }
        let mut salt = [0u8; 16];
        for i in 0..min(16, self.salt().len()) {
            salt[i] = self.salt().as_bytes()[i];
        }
        bcrypt::hash_with_salt(password.as_ref(), bcrypt::DEFAULT_COST, salt)
            .unwrap()
            .format_for_version(bcrypt::Version::TwoB)
    }
}

impl AuthCredential for User {
    fn get_username(&self) -> &str {
        &self.username()
    }

    fn get_password(&self) -> &str {
        &self.password()
    }
}

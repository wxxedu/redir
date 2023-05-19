use crate::models::user::{AuthCredential, User};

pub trait Auth {
    /// Returns the user with the given username.
    fn get_user(&self, username: impl AsRef<str>) -> Option<User>;

    /// Checks if the user has access to the application.
    fn has_access(&self, credential: impl AuthCredential) -> bool {
        let user = self.get_user(credential.get_username());
        if let Some(user) = user {
            let password = user.hash_password(credential.get_password());
            password == user.get_password()
        } else {
            false
        }
    }
}

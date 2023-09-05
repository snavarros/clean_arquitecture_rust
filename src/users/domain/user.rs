use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub provider: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        id: String,
        name: String,
        email: String,
        password: String,
        role: String,
        photo: String,
        verified: bool,
        provider: String,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Result<Self, String> {
        Ok(Self {
            id,
            name,
            email,
            password,
            role,
            photo,
            verified,
            provider,
            created_at,
            updated_at,
        })
    }
}

mod tests {
    use super::*;

    const USER_ID: &str = "user-id";
    const USER_NAME: &str = "Sebastian";
    const USER_EMAIL: &str = "snavarro@junji.cl";
    const USER_PASSWORD: &str = "123456";
    const USER_ROLE: &str = "user";
    const USER_PHOTO: &str = "";
    const USER_VERIFIED: bool = true;
    const USER_PROVIDER: &str = "google";

    #[test]
    fn should_create_the_expected_user() {
        let datetime = Utc::now();
        let snavarro = User::new(
            USER_ID.to_string(),
            USER_NAME.to_string(),
            USER_EMAIL.to_string(),
            USER_PASSWORD.to_string(),
            USER_ROLE.to_string(),
            USER_PHOTO.to_string(),
            USER_VERIFIED,
            USER_PROVIDER.to_string(),
            Some(datetime),
            Some(datetime),
        );
        let user = snavarro.unwrap();
        println!("{:?}", user);
        assert_eq!(user.id, USER_ID);
    }
}

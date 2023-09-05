use crate::users::domain::user::User;
use chrono::prelude::*;

#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}

pub fn create_user<'a>(
    id: &'a str,
    name: &'a str,
    email: &'a str,
    password: &'a str,
    role: &'a str,
    photo: &'a str,
    verified: bool,
    provider: &'a str,
    created_at: &'a Option<DateTime<Utc>>,
    updated_at: &'a Option<DateTime<Utc>>,
) -> Result<User, CreateError> {
    let user = User::new(
        id.to_string(),
        name.to_string(),
        email.to_string(),
        password.to_string(),
        role.to_string(),
        photo.to_string(),
        verified,
        provider.to_string(),
        *created_at,
        *updated_at,
    )
    .map_err(|e| CreateError::InvalidData(e))?;
    Ok(user)
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_create_the_expected_user() {}
}

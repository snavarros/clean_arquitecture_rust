use crate::common::driving::rest_handler::errors::ApiError;
use crate::common::driving::rest_handler::validate::validate;
use crate::users::domain;
use crate::users::domain::user::User;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserResponse {
    pub name: String,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        UserResponse { name: u.name }
    }
}

pub async fn create_user(request: Json<CreateUserRequest>) -> Result<Json<UserResponse>, ApiError> {
    validate(&request);
    let result = domain::create_user::create_user();
}

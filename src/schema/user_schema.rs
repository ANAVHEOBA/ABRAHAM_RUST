use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUserInput {
    #[validate(email)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginUserInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct RegistrationRq {
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[validate(email)]
    pub email: String,
    pub phone: String,
    
}
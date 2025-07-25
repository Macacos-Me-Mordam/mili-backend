use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponseDto {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenDto {
    pub refresh_token: String,
}

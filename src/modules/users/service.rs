use crate::database::entities::user;
use super::dto::{CreateUserDto, UserResponseDto};
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;
use sea_orm::EntityTrait;


pub struct UserService;

impl UserService {
    pub async fn create_user(
        db: &DatabaseConnection,
        dto: CreateUserDto,
    ) -> Result<UserResponseDto, DbErr> {
        let new_user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(dto.name),
            email: Set(dto.email),
            password: Set(dto.password), // Ideal: hashear com Argon2
            role: Set(dto.role),
            created_at: Set(Utc::now()),
        };

        let inserted = new_user.insert(db).await?;

        Ok(UserResponseDto {
            id: inserted.id.to_string(),
            name: inserted.name,
            email: inserted.email,
            role: inserted.role,
            created_at: inserted.created_at.to_string(),
        })
    }

    pub async fn get_user_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn get_all_users(
    db: &DatabaseConnection,
) -> Result<Vec<UserResponseDto>, DbErr> {
    let users = user::Entity::find().all(db).await?;

    Ok(users.into_iter().map(|u| UserResponseDto {
        id: u.id.to_string(),
        name: u.name,
        email: u.email,
        role: u.role,
        created_at: u.created_at.to_string(),
    }).collect())
}
}

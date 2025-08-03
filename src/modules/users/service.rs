use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;

use crate::database::entities::user;
use crate::modules::keycloak::client::KeycloakAdminClient;
use crate::modules::keycloak::dto::{KeycloakUserCredential, NewKeycloakUser};
use super::dto::{CreateUserDto, UserResponseDto};

pub struct UserService<'a> {
    db: &'a DatabaseConnection,
    keycloak_client: &'a KeycloakAdminClient,
}

impl<'a> UserService<'a> {
    pub fn new(db: &'a DatabaseConnection, keycloak_client: &'a KeycloakAdminClient) -> Self {
        Self { db, keycloak_client }
    }

    pub async fn create_user(
        &self,
        user_data: CreateUserDto,
    ) -> Result<user::Model, String> {
        let admin_token = self.keycloak_client
            .get_admin_token()
            .await
            .map_err(|e| format!("Falha ao obter token de admin: {}", e))?;

        if self.keycloak_client.find_user_by_email(&admin_token, &user_data.email).await
            .map_err(|e| format!("Erro ao buscar usuário: {}", e))?
            .is_some() {
            return Err("Usuário com este email já existe.".to_string());
        }

        let credentials = vec![KeycloakUserCredential {
            r#type: "password",
            value: &user_data.password,
            temporary: false,
        }];

        let new_keycloak_user = NewKeycloakUser {
            username: &user_data.email,
            email: &user_data.email,
            enabled: true,
            credentials,
        };

        let created_user = self.keycloak_client
            .create_user(&admin_token, &new_keycloak_user)
            .await
            .map_err(|e| format!("Erro na requisição para criar usuário: {}", e))?;

        // AQUI ESTÁ A CORREÇÃO: Declarar a variável 'now'
        let now = Utc::now();

        let new_user_db = user::ActiveModel {
            id: Set(Uuid::parse_str(&created_user.id).map_err(|_| "ID inválido do Keycloak".to_string())?),
            name: Set(user_data.name),
            email: Set(user_data.email),
            role: Set(user_data.role),
            // Usar a variável 'now' aqui
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
        };

        new_user_db
            .insert(self.db)
            .await
            .map_err(|e| format!("Falha ao salvar usuário no banco de dados local: {}", e))
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponseDto>, DbErr> {
        let users = user::Entity::find().all(self.db).await?;

        Ok(users.into_iter().map(|u| UserResponseDto {
            id: u.id.to_string(),
            name: u.name,
            email: u.email,
            role: u.role,
            created_at: u.created_at.to_string(),
        }).collect())
    }
}
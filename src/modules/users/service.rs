use crate::database::entities::user;
use super::dto::{CreateUserDto, UserResponseDto};
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;
use sea_orm::EntityTrait;


pub struct UserService;

impl UserService {
    
    pub async fn register_user(
    db: &DbConn,
    keycloak_client: &KeycloakAdminClient, // Receba o cliente do AppState
    user_data: CreateUser,
) -> Result<user::Model, String> {
    // 1. Obter o token de admin
    let admin_token = keycloak_client
        .get_admin_token()
        .await
        .map_err(|e| format!("Falha ao obter token de admin: {}", e))?;

    // 2. Verificar se o usuário já existe no Keycloak
    if keycloak_client.find_user_by_email(&admin_token, &user_data.email).await
        .map_err(|e| format!("Erro ao buscar usuário: {}", e))?
        .is_some() {
        return Err("Usuário com este email já existe.".to_string());
    }

    // 3. Preparar e criar o usuário no Keycloak
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

    let create_response = keycloak_client
        .create_user(&admin_token, &new_keycloak_user)
        .await
        .map_err(|e| format!("Erro na requisição para criar usuário: {}", e))?;

    if !create_response.status().is_success() {
        let error_body = create_response.text().await.unwrap_or_default();
        return Err(format!("Falha ao criar usuário no Keycloak: {}", error_body));
    }
    
    // 4. Buscar o usuário recém-criado para obter seu ID
    let created_user = keycloak_client
        .find_user_by_email(&admin_token, &user_data.email)
        .await
        .map_err(|e| format!("Erro ao buscar usuário recém-criado: {}", e))?
        .ok_or("Não foi possível encontrar o usuário recém-criado no Keycloak.")?;


    // 5. Salvar o usuário no banco de dados local
    let new_user_db = user::ActiveModel {
        id: Set(created_user.id),
        name: Set(user_data.name),
        email: Set(user_data.email),
        role: Set(user_data.role),
    };

    new_user_db
        .insert(db)
        .await
        .map_err(|e| format!("Falha ao salvar usuário no banco de dados local: {}", e))
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

use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;

use crate::database::entities::user;
use crate::modules::keycloak::client::KeycloakAdminClient;
use crate::modules::keycloak::dto::{KeycloakUserCredential, NewKeycloakUser };
use super::dto::{CreateUserDto, UserResponseDto, LoginUserDto, LoginResponseDto, RefreshTokenDto};

pub struct UserService<'a> {
    db: &'a DatabaseConnection,
    keycloak_client: &'a KeycloakAdminClient,
}

impl<'a> UserService<'a> {
    pub fn new(db: &'a DatabaseConnection, keycloak_client: &'a KeycloakAdminClient) -> Self {
        println!("🔧 Inicializando UserService");
        Self { db, keycloak_client }
    }

    pub async fn create_user(
        &self,
        user_data: CreateUserDto,
    ) -> Result<user::Model, String> {
        println!("⚙️  [UserService::create_user] email: {}", user_data.email);

        // 1. token admin
        let admin_token = self.keycloak_client
            .get_admin_token()
            .await
            .map_err(|e| {
                let msg = format!("Falha ao obter token de admin: {}", e);
                println!("❌ {}", msg);
                msg
            })?;
        println!("✅ [create_user] token admin obtido");

        // 2. verifica existing
        let existing = self.keycloak_client
            .find_user_by_email(&admin_token, &user_data.email)
            .await
            .map_err(|e| {
                let msg = format!("Erro ao buscar usuário: {}", e);
                println!("❌ {}", msg);
                msg
            })?;
        println!("🔍 [create_user] usuário existe? {}", existing.is_some());
        if existing.is_some() {
            println!("⚠️  [create_user] abortando: já existe");
            return Err("Usuário com este email já existe.".to_string());
        }

        // 3. sanitização
        let sanitized_username: String = user_data.name
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        println!("📝 [create_user] username sanitizado: {}", sanitized_username);

        // 4. payload KC
        let credentials = vec![KeycloakUserCredential {
            cred_type: "password",
            value: &user_data.password,
            temporary: false,
        }];
        let new_keycloak_user = NewKeycloakUser {
            username: &sanitized_username,
            email: &user_data.email,
            enabled: true,
            email_verified: true,
            credentials,
            required_actions: vec![],
        };
        println!("📦 [create_user] NewKeycloakUser: {:?}", new_keycloak_user);

        // 5. cria no KC
        let created_user = self.keycloak_client
            .create_user(&admin_token, &new_keycloak_user)
            .await
            .map_err(|e| {
                let msg = format!("Erro ao criar usuário no Keycloak: {}", e);
                println!("❌ {}", msg);
                msg
            })?;
        println!("🎉 [create_user] criado KC id: {}", created_user.id);

        // 6. insere DB
        let now = Utc::now();
        let new_user_db = user::ActiveModel {
            id: Set(Uuid::parse_str(&created_user.id).map_err(|e| {
                let msg = format!("ID inválido: {} ({})", created_user.id, e);
                println!("❌ {}", msg);
                msg
            })?),
            name: Set(user_data.name),
            email: Set(user_data.email),
            role: Set(user_data.role),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
        };
        println!("💾 [create_user] inserindo no DB local...");
        let inserted = new_user_db.insert(self.db).await
            .map_err(|e| {
                let msg = format!("Falha ao salvar no banco local: {}", e);
                println!("❌ {}", msg);
                msg
            })?;
        println!("✅ [create_user] salvo no DB local: {:?}", inserted);

        Ok(inserted)
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponseDto>, DbErr> {
        println!("⚙️  [UserService::get_all_users] buscando todos usuários");
        let users = user::Entity::find().all(self.db).await?;
        println!("👥 [get_all_users] {} usuários encontrados", users.len());
        Ok(users
            .into_iter()
            .map(|u| UserResponseDto {
                id: u.id.to_string(),
                name: u.name,
                email: u.email,
                role: u.role,
                created_at: u.created_at.to_string(),
            })
            .collect())
    }

    pub async fn login_user(
    &self,
    login_data: LoginUserDto,
) -> Result<LoginResponseDto, String> {
    println!("⚙️  [UserService::login_user] email: {}", login_data.email);

    let token_response = self
        .keycloak_client
        .login_user(&login_data.email, &login_data.password)
        .await
        .map_err(|e| {
            let msg = format!("Falha na autenticação: {}", e);
            println!("❌ {}", msg);
            msg
        })?;

    println!(
        "✅ [login_user] access_token recebido ({} chars)",
        token_response.access_token.len()
    );

    Ok(LoginResponseDto {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token,
    })
}

pub async fn refresh_access_token(
        &self,
        refresh_token_data: RefreshTokenDto,
    ) -> Result<LoginResponseDto, String> {
        println!("⚙️  [UserService::refresh_access_token] refrescando token");

        let token_response = self
            .keycloak_client
            .refresh_token(&refresh_token_data.refresh_token)
            .await
            .map_err(|e| {
                let msg = format!("Falha ao refrescar o token: {}", e);
                println!("❌ {}", msg);
                msg
            })?;

        println!(
            "✅ [refresh_access_token] access_token recebido ({} chars)",
            token_response.access_token.len()
        );

        Ok(LoginResponseDto {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
        })
    }
}

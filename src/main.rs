mod config;
mod modules;
mod database;
mod auth;

use axum::{
    Router,
    middleware,
};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, EntityTrait, QueryFilter, ColumnTrait};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::config::app_state::AppState;
use crate::modules::keycloak::client::KeycloakAdminClient;
use crate::modules::keycloak::config::KeycloakAdminConfig;
use crate::modules::users::service::UserService;
use crate::modules::users::dto::CreateUserDto;
use crate::database::entities::user;
use crate::modules::users::routes::{private_user_routes, public_user_routes};
use crate::modules::occurrences::routes::occurrence_routes;
use crate::modules::evidence::routes::evidence_simulation_routes;
use crate::auth::middleware::auth_middleware;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // --- CONEXÃO, MIGRAÇÕES E SEEDER ---
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_conn = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    tracing::info!("Conectado à base de dados com sucesso.");

    tracing::info!("A executar migrações da base de dados...");
    Migrator::up(&db_conn, None)
        .await
        .expect("Failed to run database migrations");
    tracing::info!("Migrações terminadas com sucesso.");
    
 
    let keycloak_admin_config = KeycloakAdminConfig::from_env();
    let keycloak_client_id = Arc::new(keycloak_admin_config.client_id.clone());
    let keycloak_client = KeycloakAdminClient::new(keycloak_admin_config);

    // --- SEEDER DO USUÁRIO ADMIN ---
    let admin_email = env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be set for seeding");
    let admin_exists = user::Entity::find()
        .filter(user::Column::Email.eq(admin_email.clone()))
        .one(&db_conn)
        .await
        .expect("Failed to query database for admin user");

    if admin_exists.is_none() {
        tracing::info!("Usuário admin não encontrado. Criando admin padrão...");
        let user_service = UserService::new(&db_conn, &keycloak_client);
        let admin_password = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set for seeding");
        let admin_dto = CreateUserDto {
            name: "Admin".to_string(),
            email: admin_email,
            password: admin_password,
            role: "admin".to_string(),
        };
        match user_service.create_user(admin_dto).await {
            Ok(_) => tracing::info!("Usuário admin criado com sucesso!"),
            Err(e) => tracing::error!("Falha ao criar usuário admin: {}", e),
        }
    } else {
        tracing::info!("Usuário admin já existe. Nenhuma ação necessária.");
    }
    
    let raw_key = env::var("KEYCLOAK_PUBLIC_KEY")
        .expect("KEYCLOAK_PUBLIC_KEY is not set in .env");
    let keycloak_public_key = Arc::new(format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
        raw_key
    ));

    // --- ESTADO DA APLICAÇÃO ---
    let app_state = AppState {
        db: db_conn,
        keycloak_public_key,
        keycloak_client,
        keycloak_client_id, 
    };

    // --- DEFINIÇÃO DAS ROTAS ---
    let private_routes = Router::new()
        .nest("/users", private_user_routes())
        .nest("/occurrences", occurrence_routes())
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    let public_routes = Router::new()
        .nest("/users", public_user_routes())
        .nest("/evidence", evidence_simulation_routes());

    let app = Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .with_state(app_state);

    // --- INICIALIZAÇÃO DO SERVIDOR ---
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Servidor a escutar em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
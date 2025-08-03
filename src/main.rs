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
use sea_orm::Database;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::config::app_state::AppState;
use crate::modules::keycloak::client::KeycloakAdminClient;
use crate::modules::keycloak::config::KeycloakAdminConfig;
use crate::modules::users::routes::{private_user_routes, public_user_routes};
use crate::auth::middleware::auth_middleware;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // --- CONEXÃO COM O BANCO DE DADOS ---
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_conn = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    tracing::info!("Conectado à base de dados com sucesso.");


    // --- MIGRAÇÕES ---
    tracing::info!("A executar migrações da base de dados...");
    Migrator::up(&db_conn, None)
        .await
        .expect("Failed to run database migrations");
    tracing::info!("Migrações terminadas com sucesso.");


    // --- CONFIGURAÇÃO DO KEYCLOAK ---
    let raw_key = env::var("KEYCLOAK_PUBLIC_KEY")
        .expect("KEYCLOAK_PUBLIC_KEY is not set in .env");

    let keycloak_public_key = Arc::new(format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
        raw_key
    ));

    let keycloak_admin_config = KeycloakAdminConfig::from_env();
    let keycloak_client_id = Arc::new(keycloak_admin_config.client_id.clone()); // CORREÇÃO: Clonamos o client_id aqui
    let keycloak_client = KeycloakAdminClient::new(keycloak_admin_config);


    // --- ESTADO DA APLICAÇÃO ---
    let app_state = AppState {
        db: db_conn,
        keycloak_public_key,
        keycloak_client,
        keycloak_client_id,
    };


    // --- ROTAS ---
    let private_routes = private_user_routes()
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware
        ));

    let app = Router::new()
        .nest("/users", public_user_routes().merge(private_routes))
        .with_state(app_state);


    // --- INICIALIZAÇÃO DO SERVIDOR ---
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Servidor a escutar em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
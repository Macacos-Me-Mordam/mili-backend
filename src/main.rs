mod modules;
mod config;
mod database;

use axum::Router;
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::config::database::connect;
use crate::config::app_state::AppState;
use crate::modules::users::routes::user_routes;
use crate::modules::keycloak::{KeycloakAdminClient, KeycloakAdminConfig};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Conexão com o banco
    let db_conn = connect().await;

    // Pega a chave pública do Keycloak do .env
    let keycloak_public_key = Arc::new(
        env::var("KEYCLOAK_PUBsLIC_KEY").expect("Missing KEYCLOAK_PUBLIC_KEY"),
    );

    // Carrega a config do admin e cria o cliente
    let keycloak_admin_config = KeycloakAdminConfig::from_env();
    let keycloak_client = KeycloakAdminClient::new(keycloak_admin_config);

    // Adiciona o keycloak_client ao estado da aplicação
    let app_state = AppState {
        db_conn,
        keycloak_public_key,
        keycloak_client,
    };

    // Cria as rotas e injeta o estado
    let app = Router::new()
        .nest("/user", user_routes())
        .with_state(app_state);

    // Define a porta a partir do .env, com fallback
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16 number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Server running on http://{}", addr);

    // Inicia o servidor
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
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

// ✅ importações para migrations
use migration::{Migrator, MigratorTrait}; // <-- aqui está seu migrator

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Conexão com o banco
    let db = connect().await;

    // ✅ Executa as migrations antes de subir o servidor
    if let Err(err) = Migrator::up(&db, None).await {
        eprintln!("Erro ao rodar migrations: {:?}", err);
        std::process::exit(1);
    }

    // Pega a chave pública do Keycloak do .env
    let keycloak_public_key = Arc::new(
        env::var("KEYCLOAK_PUBLIC_KEY").expect("Missing KEYCLOAK_PUBLIC_KEY"),
    );

    let app_state = AppState { db, keycloak_public_key };

    let app = Router::new()
        .nest("/user", user_routes())
        .with_state(app_state);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3020".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16 number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

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
use tracing::{error, info};

use crate::config::app_state::AppState;
use crate::modules::keycloak::client::KeycloakAdminClient;
use crate::modules::keycloak::config::KeycloakAdminConfig;
use crate::modules::users::service::UserService;
use crate::modules::users::dto::CreateUserDto;
use crate::database::entities::user;
use crate::modules::users::routes::{private_user_routes, public_user_routes};
use crate::modules::occurrences::routes::occurrence_routes;
use crate::modules::evidence::routes::evidence_simulation_routes;
use crate::modules::settings::routes::settings_routes;
use crate::auth::middleware::auth_middleware;

#[tokio::main]
async fn main() {
    // Carrega variáveis de ambiente do arquivo .env
    dotenv().ok();
    // Inicializa o sistema de logging
    tracing_subscriber::fmt::init();

    info!("🚀 Iniciando o servidor...");

    // --- VARIÁVEIS DE AMBIENTE E BANCO DE DADOS ---
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            error!("❌ A variável de ambiente DATABASE_URL não foi definida.");
            std::process::exit(1);
        }
    };

    let db_conn = match Database::connect(&database_url).await {
        Ok(conn) => {
            info!("✅ Conectado à base de dados com sucesso.");
            conn
        },
        Err(e) => {
            error!("❌ Falha ao conectar à base de dados: {}", e);
            std::process::exit(1);
        }
    };

    // --- MIGRAÇÕES ---
    info!("A executar migrações da base de dados...");
    if let Err(e) = Migrator::up(&db_conn, None).await {
        error!("❌ Falha ao executar migrações: {}", e);
        std::process::exit(1);
    }
    info!("✅ Migrações terminadas com sucesso.");

    // --- CONFIGURAÇÃO DO KEYCLOAK ---
    info!("Configurando cliente do Keycloak...");
    let keycloak_admin_config = KeycloakAdminConfig::from_env();
    let keycloak_client_id_clone = keycloak_admin_config.client_id.clone();
    let keycloak_client = KeycloakAdminClient::new(keycloak_admin_config);
    
    // --- SEEDER DO USUÁRIO ADMIN ---
    info!("Verificando a existência do usuário admin...");
    let admin_email = match env::var("ADMIN_EMAIL") {
        Ok(email) => email,
        Err(_) => {
            error!("❌ A variável de ambiente ADMIN_EMAIL não foi definida para o seeder.");
            std::process::exit(1);
        }
    };
    
    match user::Entity::find().filter(user::Column::Email.eq(admin_email.clone())).one(&db_conn).await {
        Ok(Some(_)) => {
            info!("✅ Usuário admin já existe. Nenhuma ação necessária.");
        },
        Ok(None) => {
            info!("Usuário admin não encontrado. Criando admin padrão...");
            let admin_password = match env::var("ADMIN_PASSWORD") {
                Ok(pass) => pass,
                Err(_) => {
                    error!("❌ A variável de ambiente ADMIN_PASSWORD não foi definida para o seeder.");
                    std::process::exit(1);
                }
            };

            let user_service = UserService::new(&db_conn, &keycloak_client);
            let admin_dto = CreateUserDto {
                name: "Admin".to_string(),
                email: admin_email,
                password: admin_password,
                role: "admin".to_string(),
            };

            if let Err(e) = user_service.create_user(admin_dto).await {
                error!("❌ Falha ao criar usuário admin: {}", e);
            } else {
                info!("✅ Usuário admin criado com sucesso!");
            }
        },
        Err(e) => {
            error!("❌ Erro ao consultar o banco de dados pelo usuário admin: {}", e);
        }
    }
    
    let raw_key = match env::var("KEYCLOAK_PUBLIC_KEY") {
        Ok(key) => key,
        Err(_) => {
            error!("❌ A variável de ambiente KEYCLOAK_PUBLIC_KEY não foi definida.");
            std::process::exit(1);
        }
    };
    let keycloak_public_key = Arc::new(format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", raw_key));
    let keycloak_client_id = Arc::new(keycloak_client_id_clone);

    // --- ESTADO DA APLICAÇÃO ---
    let app_state = AppState {
        db: db_conn,
        keycloak_public_key,
        keycloak_client,
        keycloak_client_id,
    };

    // --- DEFINIÇÃO DAS ROTAS ---
    info!("Configurando rotas da aplicação...");
    let private_routes = Router::new()
        .nest("/users", private_user_routes())
        .nest("/occurrences", occurrence_routes())
        .nest("/settings", settings_routes())
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
    let port_str = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_) => {
            error!("❌ A variável PORT='{}' não é uma porta válida.", port_str);
            std::process::exit(1);
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("✅ Servidor configurado para escutar em http://{}", addr);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            error!("❌ Falha ao iniciar o listener do servidor na porta {}: {}", port, e);
            std::process::exit(1);
        }
    };

    info!("🎉 Servidor iniciado com sucesso!");
    if let Err(e) = axum::serve(listener, app).await {
        error!("🔥 Erro fatal no servidor: {}", e);
    }
}
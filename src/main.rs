mod modules;
mod config;
mod database;

use axum::{
    routing::get,
    Router,
};

use modules::users::handler::{find_user};
use modules::cameras::handler::{find_camera};
use modules::occurrences::handler::{find_occurrences};
use modules::evidences::handler::{find_evidences};
use modules::status::handler::{find_status};
 
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/user", get(find_user))
        .route("/cameras", get(find_camera))
        .route("/occurrences", get(find_occurrences))
        .route("/evidences", get(find_evidences))
        .route("/status", get(find_status));
    // run our app with hyper, listening globally on port 3000 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

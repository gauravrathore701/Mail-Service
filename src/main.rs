mod controllers;
mod services;
mod models;

use axum::{routing::{get, post}, Router};
use services::greeting::GreetingService;
use services::database_service::DatabaseService;

#[derive(Clone)]
pub struct AppState {
    pub greeting_service: GreetingService,
    pub database_service: DatabaseService,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let greeting_service = GreetingService::new();
    let database_service = DatabaseService::new().await;

    let state = AppState {
        greeting_service,
        database_service,
    };

    let app = Router::new()
        .route("/", get(controllers::root::say_hello))
        .route("/save/subscriber", post(controllers::root::save_subscriber))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


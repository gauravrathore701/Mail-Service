mod controllers;
mod services;
mod models;

use axum::{routing::{get, post}, Router};
use services::greeting::GreetingService;
use services::database_service::DatabaseService;
use tower_http::cors::CorsLayer;
use tower_http::cors::Any;  // for allow_origin(Any)

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

    let cors = CorsLayer::new()
        .allow_origin(Any)  // * (any origin)
        .allow_methods(vec![hyper::Method::GET, hyper::Method::POST])
        .allow_headers(tower_http::cors::Any);  // all headers

    let app = Router::new()
        .route("/", get(controllers::root::say_hello))
        .route("/save/subscriber", post(controllers::root::save_subscriber))
        .layer(cors)  // Add CORS layer HERE
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7070").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
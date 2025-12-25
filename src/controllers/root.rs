use axum::{extract::{State, Json}, response::IntoResponse, http::{StatusCode, HeaderMap}};
use crate::AppState;
use crate::models::subscriber::Subscriber;

pub async fn say_hello(State(state): State<AppState>) -> impl IntoResponse {
    state.greeting_service.say_hello()
}

pub async fn save_subscriber(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(mut subscriber): Json<Subscriber>,
) -> impl IntoResponse {
    let client_id = match headers.get("clientId") {
        Some(value) => value.to_str().unwrap_or_default().to_string(),
        None => return (StatusCode::BAD_REQUEST, "ClientId is missing").into_response(),
    };

    subscriber.client_id = client_id;

    match state.database_service.save_subscriber(subscriber).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
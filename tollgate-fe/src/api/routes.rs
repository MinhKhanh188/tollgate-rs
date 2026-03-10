// tollgate-fe\src\api\routes.rs
use crate::api::handshake_service::{
    HandshakeRequest, HandshakeResponse, call_check_connection, call_handshake, call_terminate,
};
use crate::api::transaction_service::{
    CreateTransactionRequest, TransactionResponse, call_create_transaction,
};
use axum::{Json, Router, extract::State, routing::post};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
// keep 
pub struct AppState {
    pub stream: Arc<Mutex<Option<TcpStream>>>,
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/handshake", post(handshake_handler))
        .route("/transaction", post(transaction_handler))
        .route("/check-connection", post(check_connection_handler))
        .route("/terminate", post(terminate_handler))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn handshake_handler(
    State(state): State<AppState>,
    Json(payload): Json<HandshakeRequest>,
) -> Result<Json<HandshakeResponse>, String> {
    call_handshake(state, payload)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn transaction_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>, String> {
    call_create_transaction(state, payload)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn check_connection_handler(State(state): State<AppState>) -> Result<Json<String>, String> {
    call_check_connection(state).await.map(Json)
}

async fn terminate_handler(State(state): State<AppState>) -> Result<Json<String>, String> {
    call_terminate(state).await.map(Json)
}

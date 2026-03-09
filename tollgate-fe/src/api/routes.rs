// tollgate-fe\src\api\routes.rs
use crate::api::handshake_service::{HandshakeRequest, HandshakeResponse, call_handshake};
use crate::api::transaction_service::{
    CreateTransactionRequest, TransactionResponse, call_create_transaction,
};
use axum::{Json, Router, routing::post};
use tower_http::cors::CorsLayer;

pub fn app() -> Router {
    Router::new()
        .route("/handshake", post(handshake_handler))
        .route("/transaction", post(transaction_handler))
        .layer(CorsLayer::permissive())
}

async fn handshake_handler(
    Json(payload): Json<HandshakeRequest>,
) -> Result<Json<HandshakeResponse>, String> {
    call_handshake(payload)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn transaction_handler(
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>, String> {
    call_create_transaction(payload)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

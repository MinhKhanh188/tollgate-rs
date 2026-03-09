// backend\src\api\routes.rs
use crate::domain::handshake::{HandshakeRequest, HandshakeResponse};
use crate::infrastructure::db_context::DbPool;
use axum::{Json, extract::State};
use uuid::Uuid;

pub async fn handshake(
    State(pool): State<DbPool>,
    Json(payload): Json<HandshakeRequest>,
) -> Result<Json<HandshakeResponse>, String> {
    let session_id = Uuid::new_v4().to_string();
    let status = "SUCCESS".to_string();

    sqlx::query(
        r#"
        INSERT INTO handshake_history (request_id, session_id, station_id, lane_id, status)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(payload.request_id)
    .bind(&session_id)
    .bind(&payload.station_id)
    .bind(&payload.lane_id)
    .bind(&status)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(HandshakeResponse { session_id, status }))
}

// backend\src\api\routes.rs
use crate::domain::handshake::{HandshakeRequest, HandshakeResponse};
use crate::domain::transaction::{CreateTransactionRequest, TransactionResponse};
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

pub async fn create_transaction(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>, String> {
    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO transport_transactions
            (request_id, vehicle_id, etag_id, plate_number, account_id,
             checkin_toll_id, checkin_lane_id, checkin_time,
             checkout_toll_id, checkout_lane_id, checkout_time,
             amount, charge_status, status)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8::timestamp,$9,$10,$11::timestamp,$12,$13,$14)
        RETURNING id
        "#,
    )
    .bind(payload.request_id)
    .bind(payload.vehicle_id)
    .bind(payload.etag_id)
    .bind(&payload.plate_number)
    .bind(payload.account_id)
    .bind(payload.checkin_toll_id)
    .bind(payload.checkin_lane_id)
    .bind(&payload.checkin_time)
    .bind(payload.checkout_toll_id)
    .bind(payload.checkout_lane_id)
    .bind(&payload.checkout_time)
    .bind(payload.amount)
    .bind(&payload.charge_status)
    .bind(&payload.status)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(TransactionResponse {
        id: row.0,
        request_id: payload.request_id,
        status: payload.status,
        amount: payload.amount,
    }))
}

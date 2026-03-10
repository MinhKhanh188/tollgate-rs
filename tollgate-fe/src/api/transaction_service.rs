// tollgate-fe\src\api\transaction_service.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Compact request DTO — aliases match backend's serde(rename).
#[derive(Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    #[serde(rename = "rid")]
    pub request_id: Option<i64>,

    #[serde(rename = "vid")]
    pub vehicle_id: Option<i64>,

    #[serde(rename = "eid")]
    pub etag_id: Option<i64>,

    #[serde(rename = "pn")]
    pub plate_number: Option<String>,

    #[serde(rename = "aid")]
    pub account_id: Option<i64>,

    #[serde(rename = "citi")]
    pub checkin_toll_id: Option<i64>,

    #[serde(rename = "cili")]
    pub checkin_lane_id: Option<i64>,

    #[serde(rename = "cit")]
    pub checkin_time: Option<String>,

    #[serde(rename = "coti")]
    pub checkout_toll_id: Option<i64>,

    #[serde(rename = "coli")]
    pub checkout_lane_id: Option<i64>,

    #[serde(rename = "cot")]
    pub checkout_time: Option<String>,

    #[serde(rename = "amt")]
    pub amount: Option<f64>,

    #[serde(rename = "cs")]
    pub charge_status: Option<String>,

    #[serde(rename = "st")]
    pub status: Option<String>,
}

/// Compact response DTO — only essential fields.
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionResponse {
    pub id: i64,

    #[serde(rename = "rid")]
    pub request_id: Option<i64>,

    #[serde(rename = "st")]
    pub status: Option<String>,

    #[serde(rename = "amt")]
    pub amount: Option<f64>,
}

use crate::api::routes::AppState;
use crate::api::tcoc::{
    CheckinRequest, CheckinResponse, CommitRequest, CommitResponse, TcocHeader, read_header,
    write_header,
};
use tokio::net::TcpStream;

pub async fn call_create_transaction(
    state: AppState,
    req: CreateTransactionRequest,
) -> Result<TransactionResponse, String> {
    let mut lock = state.stream.lock().await;
    let stream = lock
        .as_mut()
        .ok_or("Connection not established (call handshake first)".to_string())?;

    // Step 1: Checkin
    let checkin_req = CheckinRequest {
        etag: format!("{}", req.etag_id.unwrap_or(0)),
        station: req.checkin_toll_id.unwrap_or(0) as u32,
        lane: req.checkin_lane_id.unwrap_or(0) as u32,
        plate: req.plate_number.unwrap_or_default(),
        tid: "".to_string(),
        hash_value: "".to_string(),
    };

    let checkin_header = TcocHeader {
        length: 98,
        command_id: 0x04,
        request_id: req.request_id.unwrap_or(0) as u32,
        session_id: 9999, // use existing connection fake session
    };

    write_header(stream, &checkin_header)
        .await
        .map_err(|e| e.to_string())?;
    checkin_req.write(stream).await.map_err(|e| e.to_string())?;

    let checkin_resp_header = read_header(stream).await.map_err(|e| e.to_string())?;
    let checkin_resp = CheckinResponse::read(stream)
        .await
        .map_err(|e| e.to_string())?;

    // Step 2: Commit
    let commit_req = CommitRequest {
        etag: checkin_resp.etag.clone(),
        station: checkin_resp.station,
        lane: checkin_resp.lane,
        ticket_id: checkin_resp.ticket_id,
        status: 1, // Success match
        plate: checkin_resp.plate,
        image_count: 1,
        vehicle_length: 500,
        transaction_amount: req.amount.unwrap_or(0.0) as u32,
        weight: 1500,
        reason_id: 1,
    };

    let commit_header = TcocHeader {
        length: 86,
        command_id: 0x06,
        request_id: checkin_header.request_id + 1,
        session_id: checkin_header.session_id,
    };

    write_header(stream, &commit_header)
        .await
        .map_err(|e| e.to_string())?;
    commit_req.write(stream).await.map_err(|e| e.to_string())?;

    let commit_resp_header = read_header(stream).await.map_err(|e| e.to_string())?;
    let _commit_resp = CommitResponse::read(stream)
        .await
        .map_err(|e| e.to_string())?;

    Ok(TransactionResponse {
        id: checkin_resp.ticket_id as i64,
        request_id: req.request_id,
        status: Some("SUCCESS".to_string()),
        amount: req.amount,
    })
}

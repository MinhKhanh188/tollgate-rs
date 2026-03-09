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

pub async fn call_create_transaction(
    req: CreateTransactionRequest,
) -> Result<TransactionResponse, reqwest::Error> {
    let client = Client::new();
    let res = client
        .post("http://localhost:8080/transactions")
        .json(&req)
        .send()
        .await?
        .json::<TransactionResponse>()
        .await?;
    Ok(res)
}

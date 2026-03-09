// backend\src\domain\transaction.rs
use serde::{Deserialize, Serialize};

/// Compact request DTO — short aliases to minimize JSON payload size.
#[derive(Debug, Deserialize)]
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

/// Compact response DTO — only return what the frontend needs.
#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: i64,

    #[serde(rename = "rid")]
    pub request_id: Option<i64>,

    #[serde(rename = "st")]
    pub status: Option<String>,

    #[serde(rename = "amt")]
    pub amount: Option<f64>,
}

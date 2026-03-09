use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HandshakeRequest {
    pub request_id: i64,
    pub station_id: String,
    pub lane_id: String,
}

#[derive(Debug, Serialize)]
pub struct HandshakeResponse {
    pub session_id: String,
    pub status: String,
}

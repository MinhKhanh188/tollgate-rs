use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HandshakeRequest {
    pub request_id: i64,
    pub station_id: String,
    pub lane_id: String,
}

#[derive(Deserialize, Debug)]
pub struct HandshakeResponse {
    pub session_id: String,
    pub status: String,
}

pub async fn call_handshake(req: HandshakeRequest) -> Result<HandshakeResponse, reqwest::Error> {
    let client = Client::new();
    let res = client
        .post("http://localhost:8080/handshake")
        .json(&req)
        .send()
        .await?
        .json::<HandshakeResponse>()
        .await?;
    Ok(res)
}

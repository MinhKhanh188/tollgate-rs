use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HandshakeRequest {
    pub request_id: i64,
    pub station_id: String,
    pub lane_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HandshakeResponse {
    pub session_id: String,
    pub status: String,
}

use crate::api::tcoc::{ConnectRequest, ConnectResponse, TcocHeader, read_header, write_header};
use tokio::net::TcpStream;

pub async fn call_handshake(req: HandshakeRequest) -> Result<HandshakeResponse, String> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .await
        .map_err(|e| e.to_string())?;

    let tcoc_req = ConnectRequest {
        username: "admin".to_string(),
        password: "password".to_string(),
        station: req.station_id.parse().unwrap_or(0),
        timeout: 5000,
    };

    let header = TcocHeader {
        length: 44,
        command_id: 0x00,
        request_id: req.request_id as u32,
        session_id: 0,
    };

    write_header(&mut stream, &header)
        .await
        .map_err(|e| e.to_string())?;
    tcoc_req
        .write(&mut stream)
        .await
        .map_err(|e| e.to_string())?;

    let resp_header = read_header(&mut stream).await.map_err(|e| e.to_string())?;
    if resp_header.command_id != 0x01 {
        return Err("Unexpected command ID".to_string());
    }

    let resp = ConnectResponse::read(&mut stream)
        .await
        .map_err(|e| e.to_string())?;

    Ok(HandshakeResponse {
        session_id: resp_header.session_id.to_string(),
        status: if resp.status == 0 {
            "SUCCESS".to_string()
        } else {
            "FAILED".to_string()
        },
    })
}

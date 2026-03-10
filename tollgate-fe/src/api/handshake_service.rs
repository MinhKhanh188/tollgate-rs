// tollgate-fe\src\api\handshake_service.rs
use crate::api::routes::AppState;
use crate::api::tcoc::{ConnectRequest, ConnectResponse, TcocHeader, read_header, write_header};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

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

pub async fn call_handshake(
    state: AppState,
    req: HandshakeRequest,
) -> Result<HandshakeResponse, String> {
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

    // Store connection for persistence
    let mut lock = state.stream.lock().await;
    *lock = Some(stream);

    Ok(HandshakeResponse {
        session_id: resp_header.session_id.to_string(),
        status: if resp.status == 0 {
            "SUCCESS".to_string()
        } else {
            "FAILED".to_string()
        },
    })
}

pub async fn call_check_connection(state: AppState) -> Result<String, String> {
    let mut lock = state.stream.lock().await;
    let stream = lock
        .as_mut()
        .ok_or("Connection not established (call handshake first)".to_string())?;

    // Command 0x02 is "SHAKE" in your backend socket_server.rs
    let header = TcocHeader {
        length: 16,
        command_id: 0x02,
        request_id: 777,
        session_id: 9999,
    };

    write_header(stream, &header)
        .await
        .map_err(|e| e.to_string())?;

    let resp_header = read_header(stream).await.map_err(|e| e.to_string())?;

    if resp_header.command_id == 0x03 {
        // 0x03 is SHAKE response
        Ok("Connection Alive".to_string())
    } else {
        Err("Connection Check Failed".to_string())
    }
}

pub async fn call_terminate(state: AppState) -> Result<String, String> {
    let mut lock = state.stream.lock().await;
    let stream = lock
        .as_mut()
        .ok_or("Connection not established (call handshake first)".to_string())?;

    // Command 0x0A is "TERMINATE" in your backend socket_server.rs
    let header = TcocHeader {
        length: 16,
        command_id: 0x0A,
        request_id: 888,
        session_id: 9999,
    };

    write_header(stream, &header)
        .await
        .map_err(|e| e.to_string())?;

    // The backend sends a response 0x0B before breaking the loop
    let _ = read_header(stream).await;

    // Drop the connection locally
    *lock = None;

    Ok("Connection Terminated Successfully".to_string())
}

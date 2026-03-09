use crate::api::handshake_service::{HandshakeRequest, call_handshake};

pub async fn test_handshake() {
    println!("Testing Handshake...");
    let req = HandshakeRequest {
        request_id: 12345,
        station_id: "STATION_01".to_string(),
        lane_id: "LANE_01".to_string(),
    };

    match call_handshake(req).await {
        Ok(res) => println!("Handshake Success: {:?}", res),
        Err(e) => eprintln!("Handshake Failed: {}", e),
    }
}

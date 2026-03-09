// tollgate-fe\src\ui\commands.rs
use crate::api::handshake_service::{HandshakeRequest, call_handshake};
use crate::api::transaction_service::{CreateTransactionRequest, call_create_transaction};

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

pub async fn test_transaction() {
    println!("Testing Create Transaction...");
    let req = CreateTransactionRequest {
        request_id: Some(99001),
        vehicle_id: Some(1001),
        etag_id: Some(2001),
        plate_number: Some("59A-12345".to_string()),
        account_id: Some(3001),
        checkin_toll_id: Some(10),
        checkin_lane_id: Some(1),
        checkin_time: Some("2026-03-09 10:00:00".to_string()),
        checkout_toll_id: None,
        checkout_lane_id: None,
        checkout_time: None,
        amount: Some(50000.00),
        charge_status: Some("PENDING".to_string()),
        status: Some("CHECKIN".to_string()),
    };

    match call_create_transaction(req).await {
        Ok(res) => println!("Transaction Created: {:?}", res),
        Err(e) => eprintln!("Transaction Failed: {}", e),
    }
}

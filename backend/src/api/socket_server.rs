// backend\src\api\socket_server.rs
use crate::infrastructure::db_context::DbPool;
use crate::infrastructure::tcoc::*;
use tokio::net::TcpStream;

pub async fn handle_client(mut stream: TcpStream, pool: DbPool) {
    loop {
        let header = match read_header(&mut stream).await {
            Ok(h) => h,
            Err(_) => break, // Connection closed or error
        };

        match header.command_id {
            0x00 => {
                // CONNECT
                let req = match ConnectRequest::read(&mut stream).await {
                    Ok(r) => r,
                    Err(_) => break,
                };

                let session_id = 9999; // Simple session id generation for now
                let status = "SUCCESS".to_string();
                let lane_id = "0".to_string();
                let station_id_str = req.station.to_string();

                // Track handshake in DB (re-using old logic)
                let _ = sqlx::query(
                    r#"
                    INSERT INTO handshake_history (request_id, session_id, station_id, lane_id, status)
                    VALUES ($1, $2, $3, $4, $5)
                    "#,
                )
                .bind(header.request_id as i64)
                .bind(session_id.to_string())
                .bind(&station_id_str)
                .bind(&lane_id)
                .bind(&status)
                .execute(&pool)
                .await;

                let resp = ConnectResponse { status: 0 };
                let resp_header = TcocHeader {
                    length: 20,
                    command_id: 0x01,
                    request_id: header.request_id,
                    session_id,
                };
                if write_header(&mut stream, &resp_header).await.is_err() {
                    break;
                }
                if resp.write(&mut stream).await.is_err() {
                    break;
                }
            }
            0x02 => {
                // SHAKE
                let resp = ConnectResponse { status: 0 };
                let resp_header = TcocHeader {
                    length: 20,
                    command_id: 0x03,
                    request_id: header.request_id,
                    session_id: header.session_id,
                };
                if write_header(&mut stream, &resp_header).await.is_err() {
                    break;
                }
                if resp.write(&mut stream).await.is_err() {
                    break;
                }
            }
            0x04 => {
                // CHECKIN
                let req = match CheckinRequest::read(&mut stream).await {
                    Ok(r) => r,
                    Err(_) => break,
                };

                // Create a temporary Checkin logic
                let resp = CheckinResponse {
                    status: 0,
                    etag: req.etag.clone(),
                    station: req.station,
                    lane: req.lane,
                    ticket_id: header.request_id, // fake ticket id
                    ticket_type: 1,
                    price: 35000,
                    vehicle_type: 1,
                    plate: req.plate.clone(),
                    plate_type: 1,
                    price_ticket_type: 1,
                };

                let resp_header = TcocHeader {
                    length: 86,
                    command_id: 0x05,
                    request_id: header.request_id,
                    session_id: header.session_id,
                };
                if write_header(&mut stream, &resp_header).await.is_err() {
                    break;
                }
                if resp.write(&mut stream).await.is_err() {
                    break;
                }
            }
            0x06 => {
                // COMMIT
                let req = match CommitRequest::read(&mut stream).await {
                    Ok(r) => r,
                    Err(_) => break,
                };

                // Try old route logic
                let _row: Result<(i64,), _> = sqlx::query_as(
                    r#"
                    INSERT INTO transport_transactions
                        (request_id, vehicle_id, etag_id, plate_number, account_id,
                         checkin_toll_id, checkin_lane_id, checkin_time,
                         checkout_toll_id, checkout_lane_id, checkout_time,
                         amount, charge_status, status)
                    VALUES ($1,$2,$3,$4,$5,$6,$7,NOW(),$8,$9,NOW(),$10,$11,$12)
                    RETURNING id
                    "#,
                )
                .bind(header.request_id as i64)
                .bind(1_i64) // vehicle_id
                .bind(1_i64) // etag_id related logic
                .bind(&req.plate)
                .bind(1_i64) // account_id
                .bind(req.station as i64)
                .bind(req.lane as i64)
                .bind(req.station as i64) // checkout_toll
                .bind(req.lane as i64) // checkout_lane
                .bind(req.transaction_amount as f64)
                .bind("SUCCESS")
                .bind("SETTLED")
                .fetch_one(&pool)
                .await;

                let resp = CommitResponse { status: 0 };
                let resp_header = TcocHeader {
                    length: 20,
                    command_id: 0x07,
                    request_id: header.request_id,
                    session_id: header.session_id,
                };
                if write_header(&mut stream, &resp_header).await.is_err() {
                    break;
                }
                if resp.write(&mut stream).await.is_err() {
                    break;
                }
            }
            0x0A => {
                // TERMINATE
                let resp = CommitResponse { status: 0 };
                let resp_header = TcocHeader {
                    length: 20,
                    command_id: 0x0B,
                    request_id: header.request_id,
                    session_id: header.session_id,
                };
                let _ = write_header(&mut stream, &resp_header).await;
                let _ = resp.write(&mut stream).await;
                break;
            }
            _ => {
                break; // Unknown command
            }
        }
    }
}

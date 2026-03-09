mod api;
mod ui;

#[tokio::main]
async fn main() {
    println!("Starting Tollgate Frontend...");
    ui::commands::test_handshake().await;
}

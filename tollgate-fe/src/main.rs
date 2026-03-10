// tollgate-fe\src\main.rs
mod api;
mod ui;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Tollgate Frontend API on port 4000...");

    let state = api::routes::AppState {
        stream: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
    };
    let app = api::routes::app(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// backend\src\main.rs
mod api;
mod domain;
mod infrastructure;

use axum::{Router, routing::post};
use dotenvy::dotenv;
use infrastructure::db_context::create_pool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("Connecting to the database...");

    match create_pool().await {
        Ok(pool) => {
            println!("Successfully connected to the database!");

            // Basic verification query
            let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await?;
            println!("Verification query (SELECT 1) returned: {}", row.0);

            // Build our application with a route
            let app = Router::new()
                .route("/handshake", post(api::routes::handshake))
                .layer(CorsLayer::permissive())
                .with_state(pool);

            // Run it
            let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
            println!("Backend listening on {}", addr);
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, app).await?;
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

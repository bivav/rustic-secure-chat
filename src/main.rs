use std::sync::Arc;
use tokio::sync::Mutex;
use rustic_secure_chat::SecureConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Server started");

    let secure_conn = SecureConnection::new("0.0.0.0".to_string(), 8888);
    let listener = secure_conn.connect().await?;

    let connections = Arc::new(Mutex::new(Vec::new()));

    let server = secure_conn.start_async_server(listener, connections).await
        .expect("Error starting server!");

    // Wait for Ctrl+C
    println!("Press Ctrl+C to shut down the server.");
    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    println!("Shutting down server...");
    server.abort();

    Ok(())
}
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct SecureConnection {
    pub address: String,
    pub port: u32,
}

impl SecureConnection {
    pub fn new(address: String, port: u32) -> SecureConnection {
        SecureConnection {
            address,
            port,
        }
    }

    pub async fn connect(&self) -> Result<TcpListener, Box<dyn Error>> {
        Ok(TcpListener::bind(format!("{}:{}", self.address, self.port)).await?)
    }

    pub async fn start_async_server(&self, listener: TcpListener, connections: Arc<Mutex<Vec<SocketAddr>>>) -> Result<JoinHandle<()>, Box<dyn Error>> {

        // Start the server
        let handle = tokio::spawn(async move { // Spawn a new task
            // Accept connections in a loop
            loop {
                // Accept a new connection
                match listener.accept().await {
                    // If the connection is successful
                    Ok((mut socket, addr)) => {

                        // Add the socket to the active connections list
                        let conn = Arc::clone(&connections);
                        tokio::spawn(async move {
                            let mut buf = [0; 1024];

                            // Add the socket to the active connections list
                            loop {
                                match socket.read(&mut buf).await {
                                    Ok(0) => {
                                        println!("Connection has been closed by client: {}", addr);
                                        break;
                                    }
                                    Ok(n) => {

                                        // Get the message from the buffer
                                        let mut message = String::from_utf8(buf[..n].to_vec());

                                        println!("Received message from {}: {:?}", addr, &message);

                                        // Send the message back to the client
                                        let msg = &*format!("You said: {:?}\nSay something: ", &message.unwrap().trim());

                                        // Write the message to the socket
                                        if let Err(e) = socket.write_all(&msg.as_ref()).await {
                                            eprintln!("Failed to write to socket; err = {:?}", e);
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to read from socket; err = {:?}", e);
                                        break;
                                    }
                                }
                            }

                            // Remove the socket from the active connections list
                            let mut connections_lock = conn.lock().await;
                            connections_lock.retain(|conn_addr| *conn_addr != addr);
                        });
                    }
                    Err(e) => eprintln!("Failed to accept connection; err = {:?}", e),
                }
            }
        });
        Ok(handle)
    }
}
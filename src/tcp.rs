use anyhow::Result;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct TcpServer;

impl TcpServer {
    pub async fn start(port: u16) -> Result<()> {
        let listener = TcpListener::bind(("0.0.0.0", port)).await?;
        println!("TCP server listening on port {}", port);

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("Connection from {}", addr);

            tokio::spawn(async move {
                let mut buf = vec![0; 1024];
                match socket.read(&mut buf).await {
                    Ok(len) => {
                        let received = String::from_utf8_lossy(&buf[..len]);
                        println!("Received: {}", received);

                        // Process received data
                        let response = format!("Echo: {}", received);
                        let _ = socket.write_all(response.as_bytes()).await;
                    }
                    Err(e) => eprintln!("Failed to read from socket: {}", e),
                }
            });
        }
    }
}

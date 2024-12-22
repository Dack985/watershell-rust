use anyhow::Result;
use tokio::net::UdpSocket;

pub struct UdpServer;

impl UdpServer {
    pub async fn start(port: u16) -> Result<()> {
        let socket = UdpSocket::bind(("0.0.0.0", port)).await?;
        println!("UDP server listening on port {}", port);

        let mut buf = vec![0; 1024];
        loop {
            let (len, addr) = socket.recv_from(&mut buf).await?;
            let received = String::from_utf8_lossy(&buf[..len]);
            println!("Received from {}: {}", addr, received);

            // Process received data
            let response = format!("Echo: {}", received);
            socket.send_to(response.as_bytes(), addr).await?;
        }
    }
}

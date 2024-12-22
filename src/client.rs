use clap::Parser;
use tokio::net::{TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;

#[derive(Parser)]
pub struct ClientArgs {
    #[arg(short, long)]
    target: String,
    #[arg(short, long, default_value_t = 53)]
    port: u16,
    #[arg(short, long)]
    command: String,
    #[arg(long)]
    tcp: bool,
}

pub async fn run(args: ClientArgs) -> Result<()> {
    if args.tcp {
        let mut stream = TcpStream::connect((args.target.as_str(), args.port)).await?;
        stream.write_all(args.command.as_bytes()).await?;
        let mut response = vec![0; 1024];
        let len = stream.read(&mut response).await?;
        println!("Response: {}", String::from_utf8_lossy(&response[..len]));
    } else {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.send_to(args.command.as_bytes(), (args.target.as_str(), args.port)).await?;
        let mut response = vec![0; 1024];
        let len = socket.recv(&mut response).await?;
        println!("Response: {}", String::from_utf8_lossy(&response[..len]));
    }
    Ok(())
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::time::Duration;

fn recv_timeout(socket: &UdpSocket, timeout: Duration) -> io::Result<String> {
    let mut buf = [0; 8192];
    socket.set_read_timeout(Some(timeout))?;
    match socket.recv(&mut buf) {
        Ok(size) => Ok(String::from_utf8_lossy(&buf[..size]).to_string()),
        Err(e) => Err(e),
    }
}

fn send_command_udp(socket: &UdpSocket, target: &SocketAddr, command: &str) -> io::Result<()> {
    let message = format!("run:{}", command);
    socket.send_to(message.as_bytes(), target)?;
    match recv_timeout(socket, Duration::from_secs(4)) {
        Ok(response) => println!("Response from {}: {}", target, response),
        Err(_) => println!("No response from {}", target),
    }
    Ok(())
}

fn send_command_tcp(target: &SocketAddr, command: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(target)?;
    stream.write_all(format!("run:{}", command).as_bytes())?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;
    println!("Response from {}: {}", target, String::from_utf8_lossy(&buf));
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: <program> -t <target> -p <port> -c <command> [--tcp]");
        return Ok(());
    }

    let mut target_ip = "";
    let mut port = 53;
    let mut command = "";
    let mut use_tcp = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-t" => {
                i += 1;
                target_ip = &args[i];
            }
            "-p" => {
                i += 1;
                port = args[i].parse().unwrap_or(53);
            }
            "-c" => {
                i += 1;
                command = &args[i];
            }
            "--tcp" => {
                use_tcp = true;
            }
            _ => {}
        }
        i += 1;
    }

    let target: SocketAddr = format!("{}:{}", target_ip, port).parse().expect("Invalid target address");

    if use_tcp {
        send_command_tcp(&target, command)?;
    } else {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        send_command_udp(&socket, &target, command)?;
    }

    Ok(())
}

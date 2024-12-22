use crate::udp::UdpServer;
use crate::tcp::TcpServer;
use crate::utils::*;
use anyhow::Result;

pub struct Watershell {
    port: u16,
    debug: bool,
    promiscuous: bool,
    tcp_mode: bool,
}

impl Watershell {
    pub fn new(port: u16, debug: bool, promiscuous: bool, tcp_mode: bool) -> Self {
        Self {
            port,
            debug,
            promiscuous,
            tcp_mode,
        }
    }

    pub async fn init(&self) -> Result<()> {
        if self.tcp_mode {
            TcpServer::start(self.port).await?;
        } else {
            UdpServer::start(self.port).await?;
        }
        Ok(())
    }
}

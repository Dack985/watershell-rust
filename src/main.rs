mod udp;
mod tcp;
mod utils;
mod watershell;

use watershell::Watershell;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = Watershell::new(53, false, false, true);
    server.init().await;
    Ok(())
}

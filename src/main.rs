mod udp;
mod tcp;
mod utils;
mod watershell;

use watershell::Watershell;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = Watershell::new(53, false, false, true);
    let _ = server.init().await;
    Ok(())
}

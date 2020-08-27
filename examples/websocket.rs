use bitmax_rs::{model, request, BitMaxClient};
use failure::Fallible;

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let private_key: String = std::env::var("BITMAX_PRIVATE").unwrap();
    let public_key: String = std::env::var("BITMAX_PUBLIC").unwrap();

    let c = BitMaxClient::with_auth(&public_key, &private_key, Some(6))?;
    let ws = c.websocket_all().await?;

    Ok(())
}

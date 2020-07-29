use bitmax_rs::{model, request, BitMaxClient};
use failure::Fallible;

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let private_key: String = std::env::var("BITMAX_PRIVATE").unwrap();
    let public_key: String = std::env::var("BITMAX_PUBLIC").unwrap();

    let client = BitMaxClient::with_auth(&public_key, &private_key, Some(6))?;

    let response = client.request(request::MarginRisk).await?;

    println!("{:#?}", response);
    Ok(())
}

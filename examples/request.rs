use bitmax_rs::{model, request, BitMaxClient};
use failure::Fallible;

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let private_key: String = std::env::var("BITMAX_PRIVATE").unwrap();
    let public_key: String = std::env::var("BITMAX_PUBLIC").unwrap();

    let client = BitMaxClient::with_auth(&public_key, &private_key, Some(6))?;

    let response = client
        .request(request::CancelOrder {
            account_type: model::AccountType::Cash,
            id: None,
            order_id: "a173c7f2ddbfU6638994975sbnbuT1oB",
            symbol: "BNB/USDT",
            time: chrono::Utc::now().timestamp_millis(),
        })
        .await?;

    /*
    request::CancelOrder {
        account_type: model::AccountType::Cash,
        id: None,
        order_id: "a173c7d7067fU6638994975sbnbuknTq",
        symbol: "BNB/USDT",
        time: chrono::Utc::now().timestamp_millis(),
    }
    */

    println!("{:#?}", response);
    Ok(())
}

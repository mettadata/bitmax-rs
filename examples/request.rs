#[allow(unused_imports)]
use bitmax_rs::{model, request, BitMaxClient};
use failure::Fallible;

async fn req<Q: request::Request + std::fmt::Debug>(c: &BitMaxClient, req: Q) {
    let s = format!("{:#?}", req);
    println!(
        "###\nrequest: {}\nresponse: {:#?}\n",
        s,
        c.request(req).await
    )
}

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let private_key: String = std::env::var("BITMAX_PRIVATE").unwrap();
    let public_key: String = std::env::var("BITMAX_PUBLIC").unwrap();

    let c = BitMaxClient::with_auth(&public_key, &private_key, Some(6))?;

    req(&c, request::Products).await;

    //req(&c, request::Ticker { symbol: "BNB/USDT" }).await;

    //req(&c, request::Tickers { symbols: &["BNB/USDT", "BTC/USDT"] }).await;

    //req(&c, request::AllTickers).await;

    //req(
    //    &c,
    //    request::OrderHistoryCurrent {
    //        account_type: model::AccountType::Cash,
    //        n: None,
    //        symbol: Some("BNB/USDT"),
    //        executed_only: false,
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::TransactionHistory {
    //        asset: Some("BNB"),
    //        page: None,
    //        page_size: None,
    //        tx_type: None,
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::PlaceOrder {
    //        account_type: model::AccountType::Cash,
    //        id: None,
    //        time: chrono::Utc::now().timestamp_millis(),
    //        order_qty: "0.5".parse().unwrap(),
    //        order_price: Some("30.123".parse().unwrap()),
    //        side: model::OrderSide::Sell,
    //        stop_price: None,
    //        post_only: None,
    //        time_in_force: model::TimeInForce::GTC,
    //        resp_inst: request::ResponseInstruction::Accept,
    //        symbol: "BNB/USDT",
    //        order_type: model::OrderType::Limit,
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::CancelAllOrders {
    //        symbol: Some("BNB/USDT"),
    //        account_type: model::AccountType::Cash,
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::CancelOrder {
    //        account_type: model::AccountType::Cash,
    //        id: None,
    //        order_id: "a173c7d7067fU6638994975sbnbuknTq",
    //        symbol: "BNB/USDT",
    //        time: chrono::Utc::now().timestamp_millis(),
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::PlaceOrder {
    //        account_type: model::AccountType::Cash,
    //        id: None,
    //        time: chrono::Utc::now().timestamp_millis(),
    //        order_qty: "0.5".parse().unwrap(),
    //        order_price: Some("30.123".parse().unwrap()),
    //        side: model::OrderSide::Sell,
    //        stop_price: None,
    //        post_only: None,
    //        time_in_force: model::TimeInForce::GTC,
    //        resp_inst: request::ResponseInstruction::Accept,
    //        symbol: "BNB/USDT",
    //        order_type: model::OrderType::Limit,
    //    },
    //)
    //.await;

    Ok(())
}

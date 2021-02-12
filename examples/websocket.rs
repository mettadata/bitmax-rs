use std::time::{Duration, Instant};

use bitmax_rs::{model, request, BitMaxClient};
use failure::Fallible;
use futures::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let private_key: String = std::env::var("BITMAX_PRIVATE").unwrap();
    let public_key: String = std::env::var("BITMAX_PUBLIC").unwrap();

    let c = BitMaxClient::with_auth(&public_key, &private_key, Some(6))?;
    let mut ws = c.websocket_all().await?;

    // ------ Subscription examples -------

    //ws.send(model::websocket::WsOutMessage::Subscribe {
    //    id: None,
    //    ch: model::websocket::SubscribeTopic::Trades {
    //        symbol: "BTMX/USDT",
    //    },
    //})
    //.await?;

    //ws.send(model::websocket::WsOutMessage::Subscribe {
    //    id: None,
    //    ch: model::websocket::SubscribeTopic::Bbo {
    //        symbol: "BTMX/USDT",
    //    },
    //})
    //.await?;

    //ws.send(model::websocket::WsOutMessage::Subscribe {
    //    id: None,
    //    ch: model::websocket::SubscribeTopic::RefPx {
    //        symbol: "BTMX/USDT",
    //    },
    //})
    //.await?;

    //ws.send(model::websocket::WsOutMessage::Subscribe {
    //    id: None,
    //    ch: model::websocket::SubscribeTopic::Bar {
    //        symbol: "BTMX/USDT",
    //        interval: model::Interval::T1m,
    //    },
    //})
    //.await?;

    // ------- Request examples -----------

    //ws.send(model::websocket::WsOutMessage::Request {
    //    action: model::websocket::WsRequest::DepthSnapshot {
    //        symbol: "KCS/USDT",
    //    },
    //    id: Some("abcdefg"),
    //    account: None,
    //})
    //.await?;

    //ws.send(model::websocket::WsOutMessage::Request {
    //    action: model::websocket::WsRequest::PlaceOrder(request::PlaceOrder {
    //        account_type: model::AccountType::Cash,
    //        id: None,
    //        time: chrono::Utc::now().timestamp_millis(),
    //        order_qty: "0.5".parse().unwrap(),
    //        order_price: Some("200.00".parse().unwrap()),
    //        side: model::OrderSide::Sell,
    //        stop_price: None,
    //        post_only: None,
    //        time_in_force: model::TimeInForce::GTC,
    //        resp_inst: request::ResponseInstruction::Accept,
    //        symbol: "BNB/USDT",
    //        order_type: model::OrderType::Limit,
    //    }),
    //    id: Some("abcdefg"),
    //    account: Some(model::AccountType::Cash),
    //})
    //.await?;

    // ------------------------------------

    //let ch = model::websocket::SubscribeTopic::Depth {
    //    symbol: "BTMX/USDT",
    //};
    //
    //ws.send(model::websocket::WsOutMessage::Subscribe { id: None, ch })
    //    .await?;
    //
    //let time = std::time::Instant::now();
    //let mut subbed = true;
    //let dt = Duration::from_secs(10);

    // Iterate through message queue
    while let Some(msg) = ws.next().await {
        // Send pong responses
        if let Ok(model::websocket::WsInMessage::Ping { .. }) = msg {
            ws.send(model::websocket::WsOutMessage::Pong).await?;
        }

        // when 10 seconds elapse, unsub from the messages
        //if subbed && (Instant::now() - time) > dt {
        //    ws.send(model::websocket::WsOutMessage::Unsubscribe { id: None, ch })
        //        .await?;
        //    subbed = false;
        //}

        println!("{:#?}", msg);
    }
    Ok(())
}

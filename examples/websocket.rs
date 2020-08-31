use bitmax_rs::{model, BitMaxClient};
use failure::Fallible;
use futures::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let private_key: String = std::env::var("BITMAX_PRIVATE").unwrap();
    let public_key: String = std::env::var("BITMAX_PUBLIC").unwrap();

    let c = BitMaxClient::with_auth(&public_key, &private_key, Some(6))?;
    let mut ws = c.websocket_all().await?;

    /*
    ws.send(model::websocket::WsOutMessage::Subscribe {
        id: None,
        ch: model::websocket::SubscribeTopic::Depth {
            symbol: "BTMX/USDT",
        },
    })
    .await?;
    */
    ws.send(model::websocket::WsOutMessage::Subscribe {
        id: None,
        ch: model::websocket::SubscribeTopic::Trades {
            symbol: "BTMX/USDT",
        },
    })
    .await?;
    while let Some(msg) = ws.next().await {
        if let Ok(model::websocket::WsInMessage::Ping { .. }) = msg {
            ws.send(model::websocket::WsOutMessage::Pong).await?;
        }
        println!("{:#?}", msg);
    }
    Ok(())
}

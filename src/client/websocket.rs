use failure::Fallible;
use futures::{
    sink::SinkExt,
    stream::Stream,
    task::{Context, Poll},
};
use log::debug;
use pin_project::pin_project;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::from_str;
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{http::Request as HttpRequest, protocol::Message as WSMessage},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::client::BitMaxClient;

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

const WS_ENDPOINT: &'static str = "/stream";

#[pin_project]
pub struct BitMaxWebsocket {
    #[pin]
    stream: WSStream,
}

impl BitMaxClient {
    async fn websocket(&self, auth: bool) -> Fallible<BitMaxWebsocket> {
        let endpoint = self.render_url("wss", WS_ENDPOINT, auth)?;

        let request = HttpRequest::builder()
            .uri(endpoint)
            .header("user-agent", "bitmax-rs");

        let request = if auth {
            self.attach_auth_headers(request, WS_ENDPOINT)?
        } else {
            request
        };

        let (stream, _) = connect_async(request.body(())?).await?;

        Ok(BitMaxWebsocket { stream })
    }

    pub async fn websocket_public(&self) -> Fallible<BitMaxWebsocket> {
        self.websocket(false).await
    }

    pub async fn websocket_all(&self) -> Fallible<BitMaxWebsocket> {
        self.websocket(true).await
    }
}


impl Stream for BitMaxWebsocket {
    type Item = Fallible<BinanceDexWsMessage>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let poll = this.streams.poll_next(cx);
        match poll {
            Poll::Ready(Some((y, token))) => match y {
                StreamYield::Item(item) => {
                    let topic = this.topics.get(&token).unwrap();
                    Poll::Ready({
                        Some(
                            item.map_err(failure::Error::from)
                                .and_then(|msg| parse_message(msg, topic)),
                        )
                    })
                }
                StreamYield::Finished(_) => Poll::Pending,
            },
            Poll::Ready(None) => Poll::Pending,
            Poll::Pending => Poll::Pending,
        }
    }
}

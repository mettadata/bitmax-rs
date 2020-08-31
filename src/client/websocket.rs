use failure::Fallible;
use futures::{
    sink::Sink,
    stream::Stream,
    task::{Context, Poll},
};
use log::debug;
use pin_project::pin_project;
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{http::Request as HttpRequest, protocol::Message as TungsteniteWSMessage},
    MaybeTlsStream, WebSocketStream,
};

use crate::{
    client::BitMaxClient,
    model::websocket::{WsInMessage, WsOutMessage},
};

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

const WS_ENDPOINT: &str = "/stream";

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
    type Item = Fallible<WsInMessage>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let poll = this.stream.poll_next(cx);
        poll.map(|msg| msg.map(|msg| msg.map_err(failure::Error::from).and_then(parse_message)))
    }
}

fn parse_message(msg: TungsteniteWSMessage) -> Fallible<WsInMessage> {
    let msg = match msg {
        TungsteniteWSMessage::Text(msg) => msg,
        TungsteniteWSMessage::Binary(_) => {
            return Err(failure::format_err!("Unexpected binary contents"))
        }
        TungsteniteWSMessage::Pong(..) => {
            return Err(failure::format_err!("Recieved pong in unexpected format"))
        }
        TungsteniteWSMessage::Ping(..) => {
            return Err(failure::format_err!("Recieved ping in unexpected format"))
        }
        TungsteniteWSMessage::Close(..) => {
            return Ok(WsInMessage::Closed);
        }
    };

    debug!("Incoming websocket message {}", msg);

    serde_json::from_str(&msg)
        .map_err(|e| failure::format_err!("could not deserialize {}, error: {:#?}", msg, e))
}

impl<'a> Sink<WsOutMessage<'a>> for BitMaxWebsocket {
    type Error = failure::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.stream.poll_ready(cx).map_err(|e| e.into())
    }

    fn start_send(self: Pin<&mut Self>, msg: WsOutMessage<'a>) -> Result<(), Self::Error> {
        let msg = serde_json::to_string(&msg)?;
        debug!("Sending '{}' through websocket", msg);
        let this = self.project();
        Ok(this.stream.start_send(TungsteniteWSMessage::Text(msg))?)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.stream.poll_flush(cx).map_err(|e| e.into())
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.stream.poll_close(cx).map_err(|e| e.into())
    }
}

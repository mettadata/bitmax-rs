use std::fmt;

use serde::{Serialize, Serializer};

use crate::model::Interval;

#[derive(Debug, Clone)]
pub enum SubscribeTopic<'a> {
    Depth { symbol: &'a str },
    Bbo { symbol: &'a str },
    Trades { symbol: &'a str },
    Bar { symbol: &'a str, interval: Interval },
    RefPx { symbol: &'a str },
}

impl<'a> Serialize for SubscribeTopic<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ch = match self {
            Self::Depth { symbol } => format!("depth:{}", symbol),
            Self::Bbo { symbol } => format!("bbo:{}", symbol),
            Self::Trades { symbol } => format!("trades:{}", symbol),
            Self::Bar { symbol, interval } => format!("bar:{}:{}", interval, symbol),
            Self::RefPx { symbol } => format!("ref-px:{}", symbol),
        };

        serializer.serialize_str(&ch)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum WSRequest {
    PlaceOrder,
    CancelOrder,
    CancelAll,
    DepthSnapshot,
    DepthSnapshotTop100,
    MarketTrades,
    Balance,
    OpenOrder,
    MarginRisk,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "op")]
pub enum WebsocketOp<'a> {
    #[serde(rename = "sub")]
    Subscribe { ch: SubscribeTopic<'a> },
    #[serde(rename = "unsub")]
    Unsubscribe { ch: &'a str },
    #[serde(rename = "req")]
    Request,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebsocketOutMessage<'a> {
    id: Option<&'a str>,
    #[serde(flatten)]
    op: WebsocketOp<'a>,
}

pub enum BitMaxWsMessage {
    
}
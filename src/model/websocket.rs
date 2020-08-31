use serde::{Deserialize, Serialize, Serializer};

use crate::model::{de_f64_str, Fixed9, Interval, PriceQty};

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
pub enum WsRequest {
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
pub enum WsOutMessage<'a> {
    #[serde(rename = "sub")]
    Subscribe {
        ch: SubscribeTopic<'a>,
        id: Option<&'a str>,
    },
    #[serde(rename = "unsub")]
    Unsubscribe { ch: &'a str },
    #[serde(rename = "pong")]
    Pong,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    Auth,
    Unauth,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "m", rename_all = "lowercase")]
pub enum WsInMessage {
    Ping {
        hp: u8,
    },
    Disconnected {
        code: u32,
        reason: String,
        info: String,
    },
    Error {
        code: u32,
        reason: String,
        info: String,
    },
    Connected {
        #[serde(rename = "type")]
        type_: AuthType,
    },
    Closed,
    #[serde(rename = "sub")]
    Subscribed {
        id: Option<String>,
        code: u32,
        ch: String,
    },
    Depth {
        symbol: String,
        data: DepthData,
    },
    Bbo {
        ts: i64,
        symbol: String,
        data: BboData,
    },
    Trades {
        symbol: String,
        data: Vec<Trade>,
    },
    Bar {
        symbol: String,
        data: BarData,
    },
}

#[derive(Clone, Debug, Deserialize)]
pub struct DepthData {
    pub ts: i64,
    pub seqnum: u64,
    pub asks: Vec<PriceQty>,
    pub bids: Vec<PriceQty>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BboData {
    pub bid: PriceQty,
    pub ask: PriceQty,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Trade {
    #[serde(rename = "p")]
    pub price: Fixed9,
    #[serde(rename = "q")]
    pub qty: Fixed9,
    pub ts: i64,
    #[serde(rename = "bm")]
    pub is_buyer_maker: bool,
    pub seqnum: u64,
}
#[derive(Clone, Debug, Deserialize)]
pub struct BarData {
    #[serde(rename = "i")]
    pub interval: Interval,
    pub ts: i64,
    #[serde(rename = "o")]
    pub open: Fixed9,
    #[serde(rename = "c")]
    pub close: Fixed9,
    #[serde(rename = "h")]
    pub high: Fixed9,
    #[serde(rename = "l")]
    pub low: Fixed9,
    #[serde(deserialize_with = "de_f64_str")]
    pub volume: f64,
}

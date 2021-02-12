use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};

use crate::model::{self, AccountType, Fixed9};

pub trait Request: Serialize {
    type Response: DeserializeOwned + std::fmt::Debug;

    const METHOD: Method;
    const NEEDS_ACCOUNT_GROUP: bool;
    const NEEDS_AUTH: bool;
    const API_PATH: &'static str;

    fn account_type(&self) -> Option<model::AccountType> {
        None
    }

    fn render_endpoint(&self) -> String {
        match self.account_type() {
            None => Self::API_PATH.into(),
            Some(AccountType::Cash) => format!("/cash{}", Self::API_PATH),
            Some(AccountType::Margin) => format!("/margin{}", Self::API_PATH),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dummy;

impl<'de> Deserialize<'de> for Dummy {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Dummy {})
    }
}

/// Obtain a list of all assets listed on the exchange
#[derive(Serialize, Clone, Copy, Debug)]
pub struct Assets;

impl Request for Assets {
    type Response = Vec<model::Asset>;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = false;
    const API_PATH: &'static str = "/assets";
}

/// Obtain a list of all products traded on the exchange
#[derive(Serialize, Clone, Copy, Debug)]
pub struct Products;

impl Request for Products {
    type Response = Vec<model::Product>;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = false;
    const API_PATH: &'static str = "/products";
}

/// Get summary statistics of a symbol
#[derive(Serialize, Clone, Copy, Debug)]
pub struct Ticker<'a> {
    pub symbol: &'a str,
}

impl Request for Ticker<'_> {
    type Response = model::Ticker;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = false;
    const API_PATH: &'static str = "/ticker";
}

/// Get summary statistics of several symbols
#[derive(Serialize, Clone, Copy, Debug)]
pub struct Tickers<'a> {
    #[serde(rename = "symbol")]
    pub symbols: &'a [&'a str],
}

impl Request for Tickers<'_> {
    type Response = Vec<model::Ticker>;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = false;
    const API_PATH: &'static str = "/ticker";
}

/// Get summary statistics of all symbols
#[derive(Serialize, Clone, Copy, Debug)]
pub struct AllTickers;

impl Request for AllTickers {
    type Response = Vec<model::Ticker>;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = false;
    const API_PATH: &'static str = "/ticker";
}

/// Get list of all bar intervals supported by the server
#[derive(Serialize, Clone, Copy, Debug)]
pub struct BarhistInfo;

impl Request for BarhistInfo {
    type Response = Vec<model::BarhistInfo>;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = false;
    const API_PATH: &'static str = "/barhist/info";
}

/// Get a list of bars, with each contains the open/close/high/low
/// prices of a symbol for a specific time range
///
///
/// `from`/`to` each specifies the start timestamp of the first/last bar.
/// `to` is always honored. If not provided, this field will be set to the current system time.
/// For `from` and `to`:
///     1) If only from is provided, then the request range is determined by `[from, to]`, inclusive.
///     However, if the range is too wide, the server will increase from so the number of bars in the response won't exceed 500.
///     2) If only `n` is provided, then the server will return the most recent `n` data bars to time to.
///     However, if `n` is greater than 500, only 500 bars will be returned.
///     3) If both `from` and `n` are specified, the server will pick one that returns fewer bars.
#[derive(Serialize, Clone, Debug)]
pub struct Barhist<'a> {
    pub symbol: &'a str,
    pub interval: model::Interval,
    pub from: Option<i64>, // unix timestamp in !MILLISECONDS!
    pub to: Option<i64>,   // unix timestamp in !MILLISECONDS!
    pub n: Option<u32>,    // number of bars to be returned
}

impl Request for Barhist<'_> {
    type Response = Vec<model::Barhist>;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = false;
    const API_PATH: &'static str = "/barhist";
}

#[derive(Serialize, Clone, Debug)]
pub struct OrderDepth<'a> {
    pub symbol: &'a str,
}

impl Request for OrderDepth<'_> {
    type Response = model::OrderDepth;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = false;
    const API_PATH: &'static str = "/depth";
}

#[derive(Serialize, Clone, Debug)]
pub struct Trades<'a> {
    pub symbol: &'a str,
    #[serde(rename = "n")]
    pub number: Option<u8>, // number of trades to return, capped at 100
}

impl Request for Trades<'_> {
    type Response = model::Trades;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = false;
    const API_PATH: &'static str = "/trades";
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct AccountInfo;

impl Request for AccountInfo {
    type Response = model::AccountInfo;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/info";
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance<'a> {
    #[serde(skip)]
    pub account_type: AccountType,
    pub asset: Option<&'a str>,
    pub show_all: bool, // show assets with balance 0
}

impl Request for Balance<'_> {
    type Response = Vec<model::Balance>;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = true;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/balance";

    fn account_type(&self) -> Option<AccountType> {
        Some(self.account_type)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct MarginRisk;

impl Request for MarginRisk {
    type Response = model::MarginRisk;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = true;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/margin/risk";
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SelfTransfer<'a> {
    pub amount: Fixed9,
    pub asset: &'a str,
    pub from_account: AccountType,
    pub to_account: AccountType,
}

impl Request for SelfTransfer<'_> {
    type Response = Dummy;

    const METHOD: Method = Method::POST;
    const NEEDS_ACCOUNT_GROUP: bool = true;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/transfer";
}

#[derive(Serialize, Clone, Debug)]
pub struct DepositAddress<'a> {
    pub asset: &'a str,
    pub blockchain: Option<&'a str>,
}

impl Request for DepositAddress<'_> {
    type Response = model::DepositAddress;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/wallet/deposit/address";
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistory<'a> {
    pub asset: Option<&'a str>,
    pub tx_type: Option<model::TransactionType>,
    pub page: Option<u32>, // page number, starting at 1
    pub page_size: Option<u32>,
}

impl Request for TransactionHistory<'_> {
    type Response = model::TransactionHistory;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = false;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/wallet/transactions";
}

#[derive(Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResponseInstruction {
    #[serde(rename = "ACK")]
    Acknowledged,
    Accept,
    Done,
}

/// Refer to https://bitmax-exchange.github.io/bitmax-pro-api/#place-order
/// in order to ensure that your request is well-formed.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrder<'a> {
    #[serde(skip)]
    pub account_type: AccountType,
    pub symbol: &'a str,
    pub time: i64,
    pub order_qty: Fixed9,
    pub order_type: model::OrderType,
    pub side: model::OrderSide,
    pub id: Option<&'a str>,
    pub order_price: Option<Fixed9>,
    pub stop_price: Option<Fixed9>,
    pub post_only: Option<bool>,
    pub time_in_force: model::TimeInForce,
    pub resp_inst: ResponseInstruction,
}

impl Request for PlaceOrder<'_> {
    type Response = model::PlaceOrderResponse;

    const METHOD: Method = Method::POST;
    const NEEDS_ACCOUNT_GROUP: bool = true;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/order";

    fn account_type(&self) -> Option<AccountType> {
        Some(self.account_type)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrder<'a> {
    #[serde(skip)]
    pub account_type: AccountType,
    pub id: Option<&'a str>,
    pub order_id: &'a str,
    pub symbol: &'a str,
    pub time: i64,
}

impl Request for CancelOrder<'_> {
    type Response = model::CancelOrderResponse;

    const METHOD: Method = Method::DELETE;
    const NEEDS_ACCOUNT_GROUP: bool = true;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/order";

    fn account_type(&self) -> Option<AccountType> {
        Some(self.account_type)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CancelAllOrders<'a> {
    #[serde(skip)]
    pub account_type: AccountType,
    pub symbol: Option<&'a str>,
}

impl Request for CancelAllOrders<'_> {
    type Response = model::CancelAllInfo;

    const METHOD: Method = Method::DELETE;
    const NEEDS_ACCOUNT_GROUP: bool = true;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/order/all";

    fn account_type(&self) -> Option<AccountType> {
        Some(self.account_type)
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatus<'a> {
    #[serde(skip)]
    pub account_type: AccountType,
    pub order_id: &'a str,
}

impl Request for OrderStatus<'_> {
    type Response = model::Order;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = true;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/order/status";

    fn account_type(&self) -> Option<AccountType> {
        Some(self.account_type)
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrdersStatus<'a> {
    #[serde(skip)]
    pub account_type: AccountType,
    pub order_id: &'a [&'a str],
}

impl Request for OrdersStatus<'_> {
    type Response = Vec<model::Order>;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = true;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/order/status";

    fn account_type(&self) -> Option<AccountType> {
        Some(self.account_type)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct OpenOrders {
    #[serde(skip)]
    pub account_type: AccountType,
}

impl Request for OpenOrders {
    type Response = Vec<model::Order>;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = true;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/order/open";

    fn account_type(&self) -> Option<AccountType> {
        Some(self.account_type)
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryCurrent<'a> {
    #[serde(skip)]
    pub account_type: AccountType,
    pub n: Option<u32>,
    pub symbol: Option<&'a str>,
    pub executed_only: bool,
}

impl Request for OrderHistoryCurrent<'_> {
    type Response = Vec<model::Order>;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = true;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/order/hist/current";

    fn account_type(&self) -> Option<AccountType> {
        Some(self.account_type)
    }
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistory<'a> {
    #[serde(rename = "category")]
    pub account_type: AccountType,
    pub symbol: Option<&'a str>,
    pub order_type: Option<model::OrderType>,
    pub side: Option<model::OrderSide>,
    pub status: Option<model::OrderStatus>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

impl Request for OrderHistory<'_> {
    type Response = model::OrderHistoryPage;

    const METHOD: Method = Method::GET;
    const NEEDS_ACCOUNT_GROUP: bool = true;
    const NEEDS_AUTH: bool = true;
    const API_PATH: &'static str = "/order/hist";
}

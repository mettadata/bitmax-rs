use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::model;

#[derive(Debug, Clone, Copy)]
pub enum AccountType {
    Cash,
    Margin,
}

pub trait Request: Serialize {
    type Response: DeserializeOwned + std::fmt::Debug;

    const METHOD: Method;
    const NEEDS_ACCOUNT_GROUP: bool;
    const NEEDS_AUTH: bool;
    const API_PATH: &'static str;

    fn account_type(&self) -> Option<AccountType> {
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

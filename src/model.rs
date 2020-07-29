use serde::{de, Deserialize, Deserializer, Serialize};

mod fixed9;

pub use fixed9::Fixed9;

fn de_f64_str<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    let s: &str = Deserialize::deserialize(deserializer)?;

    s.parse::<f64>().map_err(de::Error::custom)
}

// Price/qty pair
pub type PriceQty = (Fixed9, Fixed9);

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum AssetStatus {
    Normal,
    NoDeposit,
    NoWithdraw,
    NoTransaction,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset_code: String,
    pub asset_name: String,
    #[serde(deserialize_with = "de_f64_str")]
    pub min_withdrawal_amt: f64,
    #[serde(deserialize_with = "de_f64_str")]
    pub withdrawal_fee: f64,
    pub precision_scale: u32,
    pub native_scale: u32,
    pub status: AssetStatus,
}

#[derive(Deserialize, Clone, Debug, Copy)]
pub enum ComissionType {
    Base,
    Quote,
    Received,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub symbol: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub min_notional: Fixed9,
    pub max_notional: Fixed9,
    pub tick_size: Fixed9,
    pub lot_size: Fixed9,
    pub margin_tradable: bool,
    pub commission_type: ComissionType,
    pub commission_reserve_rate: Fixed9,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SymbolType {
    Spot,
    Derivatives,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Ticker {
    pub symbol: String,
    pub open: Fixed9,
    pub close: Fixed9,
    pub high: Fixed9,
    pub low: Fixed9,
    #[serde(deserialize_with = "de_f64_str")]
    pub volume: f64,
    pub ask: PriceQty, // Price and size of the best ask level
    pub bid: PriceQty, // Price and size of the best bid level
    #[serde(rename = "type")]
    pub type_: SymbolType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Interval {
    #[serde(rename = "1")]
    T1m,
    #[serde(rename = "5")]
    T5m,
    #[serde(rename = "15")]
    T15m,
    #[serde(rename = "30")]
    T30m,
    #[serde(rename = "60")]
    T60m,
    #[serde(rename = "120")]
    T120m,
    #[serde(rename = "240")]
    T240m,
    #[serde(rename = "360")]
    T360m,
    #[serde(rename = "720")]
    T720m,
    #[serde(rename = "1d")]
    T1d,
    #[serde(rename = "1w")]
    T1w,
    #[serde(rename = "1m")]
    T1M,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BarhistInfo {
    // note that the one-month bar (T1M) always resets at the month start. The interval value for the one-month bar is only indicative.
    pub name: Interval,
    pub interval_in_millis: u64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum MessageType {
    Bar,
    DepthSnapshot,
    Trades,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Barhist {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "m")]
    pub message_type: MessageType,
    pub data: BarhistData,
}
#[derive(Deserialize, Clone, Debug)]
pub struct BarhistData {
    #[serde(rename = "o")]
    pub open: Fixed9,
    #[serde(rename = "c")]
    pub close: Fixed9,
    #[serde(rename = "h")]
    pub high: Fixed9,
    #[serde(rename = "l")]
    pub low: Fixed9,
    #[serde(rename = "v", deserialize_with = "de_f64_str")]
    pub volume: f64,
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "i")]
    pub interval: Interval,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OrderDepth {
    pub symbol: String,
    #[serde(rename = "m")]
    pub message_type: MessageType,
    pub data: OrderDepthData,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OrderDepthData {
    pub seqnum: u64,
    pub ts: i64,
    pub asks: Vec<PriceQty>,
    pub bids: Vec<PriceQty>,
}
#[derive(Deserialize, Clone, Debug)]
pub struct Trades {
    pub symbol: String,
    #[serde(rename = "m")]
    pub message_type: MessageType,
    pub data: Vec<Trade>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Trade {
    pub seqnum: u64,
    #[serde(rename = "p")]
    pub price: Fixed9,
    #[serde(rename = "q")]
    pub qty: Fixed9,
    pub ts: i64,
    #[serde(rename = "bm")]
    pub is_buyer_maker: bool,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    account_group: u8,
    email: String,
    cash_account: Vec<String>,
    margin_account: Vec<String>,
    futures_account: Vec<String>,
    trade_permission: bool,
    transfer_permission: bool,
    view_permission: bool,
    #[serde(rename = "userUID")]
    user_uid: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    asset: String,
    total_balance: Fixed9,
    available_balance: Fixed9,
    borrowed: Option<Fixed9>, // margin account only
    interest: Option<Fixed9>, // margin account only
}


/// All balances are in USDT
#[derive(Deserialize, Clone, Debug)]
pub struct MarginRisk {
    #[serde(rename = "accountMaxLeverage", deserialize_with = "de_f64_str")]
    pub max_leverage: f64,
    #[serde(rename = "availableBalanceInUSDT")]
    pub available_balance: Fixed9,
    #[serde(rename = "totalBalanceInUSDT")]
    pub total_balance: Fixed9,
    #[serde(rename = "totalBorrowedInUSDT")]
    pub total_borrowed: Fixed9,
    #[serde(rename = "totalInterestInUSDT")]
    pub total_interest: Fixed9,
    #[serde(rename = "netBalanceInUSDT")]
    pub net_balance: Fixed9,
    #[serde(rename = "pointsBalance", deserialize_with = "de_f64_str")]
    pub points_balance: f64,
    #[serde(rename = "currentLeverage", deserialize_with = "de_f64_str")]
    pub current_leverage: f64,
    #[serde(deserialize_with = "de_f64_str")]
    pub cushion: f64,
}

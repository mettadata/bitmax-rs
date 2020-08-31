use std::fmt;

use serde::{
    de::{self, IntoDeserializer},
    Deserialize, Deserializer, Serialize,
};

mod fixed9;
pub mod websocket;

pub use fixed9::Fixed9;

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_deref();
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

fn de_f64_str<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    let s: &str = Deserialize::deserialize(deserializer)?;

    s.parse::<f64>().map_err(de::Error::custom)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    #[serde(alias = "CASH")]
    Cash,
    #[serde(alias = "MARGIN")]
    Margin,
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Cash
    }
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

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::T1m => "1",
                Self::T5m => "5",
                Self::T15m => "15",
                Self::T30m => "30",
                Self::T60m => "60",
                Self::T120m => "120",
                Self::T240m => "240",
                Self::T360m => "360",
                Self::T720m => "720",
                Self::T1d => "1d",
                Self::T1w => "1w",
                Self::T1M => "1m",
            }
        )
    }
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
    PlaceOrder,
    CancelOrder,
    CancelAll,
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
    pub account_group: u8,
    pub email: String,
    pub cash_account: Vec<String>,
    pub margin_account: Vec<String>,
    pub futures_account: Vec<String>,
    pub trade_permission: bool,
    pub transfer_permission: bool,
    pub view_permission: bool,
    #[serde(rename = "userUID")]
    pub user_uid: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    pub total_balance: Fixed9,
    pub available_balance: Fixed9,
    pub borrowed: Option<Fixed9>, // margin account only
    pub interest: Option<Fixed9>, // margin account only
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

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub asset: String,
    pub asset_name: String,
    pub address: Vec<DepositBlockchainAddress>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DepositBlockchainAddress {
    pub address: String,
    pub blockchain: String,
    pub dest_tag: String,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DestAddress {
    pub address: String,
    pub dest_tag: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TransactionStatus {
    Pending,
    Reviewing,
    Confirmed,
    Rejected,
    Canceled,
    Failed,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistoryEntry {
    pub asset: String,
    pub amount: Fixed9,
    pub commission: Fixed9,
    pub dest_address: DestAddress,
    pub network_transaction_id: String,
    pub num_confirmations: u32,
    pub num_confirmed: u32,
    pub request_id: String,
    pub status: TransactionStatus,
    pub time: i64,
    pub transaction_type: TransactionType,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistory {
    pub data: Vec<TransactionHistoryEntry>,
    pub has_next: bool,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    #[serde(alias = "Market")]
    Market,
    #[serde(alias = "Limit")]
    Limit,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderSide {
    #[serde(alias = "Buy")]
    Buy,
    #[serde(alias = "Sell")]
    Sell,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum TimeInForce {
    GTC,
    IOC,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AckOrderInfo {
    pub id: String,
    pub order_id: String,
    pub order_type: OrderType,
    pub symbol: String,
    pub timestamp: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub enum ExecInstruction {
    #[serde(rename = "POST")]
    Post,
    Liquidation,
    #[serde(rename = "NULL_VAL")]
    Null,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OrderStatus {
    New,
    PendingNew,
    PartiallyFilled,
    Filled,
    Rejected,
    Canceled,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub avg_px: Fixed9,
    pub cum_fee: Fixed9,
    pub cum_filled_qty: Fixed9,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub error_code: Option<String>,
    pub fee_asset: String,
    pub last_exec_time: i64,
    pub order_id: String,
    pub order_qty: Fixed9,
    pub order_type: OrderType,
    pub price: Fixed9,
    pub seq_num: u64,
    pub side: OrderSide,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub stop_price: Option<Fixed9>,
    pub symbol: String,
    pub status: OrderStatus,
    pub exec_inst: ExecInstruction,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "status", content = "info", rename_all = "UPPERCASE")]
pub enum PlaceOrderInfo {
    Done(Order),
    Accept(Order),
    #[serde(rename = "Ack")]
    Acknowledged(AckOrderInfo),
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub ac: AccountType,
    pub account_id: String,
    pub action: MessageType,
    #[serde(flatten)]
    pub info: PlaceOrderInfo,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AckCancelInfo {
    pub id: String,
    pub order_id: String,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub order_type: Option<OrderType>,
    pub symbol: String,
    pub timestamp: i64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "status", content = "info")]
pub enum CancelOrderInfo {
    #[serde(rename = "Ack")]
    Acknowledged(AckCancelInfo),
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub account_id: String,
    pub ac: AccountType,
    pub action: MessageType,
    #[serde(flatten)]
    pub info: CancelOrderInfo,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AckCancelAllInfo {
    #[serde(deserialize_with = "empty_string_as_none")]
    pub symbol: Option<String>,
    pub timestamp: i64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "status", content = "info")]
pub enum CancelAllInfo {
    #[serde(rename = "Ack")]
    Acknowledged(AckCancelAllInfo),
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllResponse {
    pub account_id: String,
    pub ac: AccountType,
    pub action: MessageType,
    #[serde(flatten)]
    pub info: CancelAllInfo,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoryOrder {
    pub ac: AccountType,
    pub account_id: String,
    pub avg_px: Fixed9,
    pub cum_fee: Fixed9,
    pub cum_qty: Fixed9,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub error_code: Option<String>,
    pub fee_asset: String,
    pub last_exec_time: i64,
    pub order_id: String,
    pub order_qty: Fixed9,
    pub order_type: OrderType,
    pub price: Fixed9,
    pub seq_num: u64,
    pub sending_time: i64,
    pub side: OrderSide,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub stop_price: Option<Fixed9>,
    pub symbol: String,
    pub status: OrderStatus,
    pub exec_inst: ExecInstruction,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryPage {
    pub data: Vec<HistoryOrder>,
    pub has_next: bool,
    pub limit: u32,
    pub page: u32,
    pub page_size: u32,
}

//! # Trade
//!
//! Datatypes for Bitpanda API trades

use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

/// A trade on the Bitpanda exchange
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Trade {
    pub amount_asset: Decimal,
    pub amount_fiat: Decimal,
    pub datetime: DateTime<FixedOffset>,
    pub fiat_to_eur_rate: Decimal,
    pub id_asset: String,
    pub id_fiat: String,
    pub id_wallet: String,
    pub id: String,
    pub price: Decimal,
    /// The Swap trade related to this
    pub related_swap_trade: Option<Box<Trade>>,
    pub status: TradeStatus,
    pub symbol: String,
    pub r#type: TradeType,
}

/// Defines the trade status
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
pub enum TradeStatus {
    Pending,
    Processing,
    Finished,
    Canceled,
}

/// Defines the trade type
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
pub enum TradeType {
    Buy,
    Sell,
}

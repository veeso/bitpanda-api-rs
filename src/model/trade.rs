//! # Trade
//!
//! Datatypes for Bitpanda API trades

use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

use crate::ApiError;

/// A trade on the Bitpanda exchange
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Trade {
    pub amount_asset: Decimal,
    pub amount_fiat: Decimal,
    pub datetime: DateTime<FixedOffset>,
    pub fiat_to_eur_rate: Decimal,
    pub fiat_wallet_id: Option<String>,
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

impl FromStr for TradeStatus {
    type Err = ApiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "pending" => Ok(Self::Pending),
            "processing" => Ok(Self::Processing),
            "finished" => Ok(Self::Finished),
            "canceled" => Ok(Self::Canceled),
            _ => Err(ApiError::UnexpectedValue(value.to_string())),
        }
    }
}

/// Defines the trade type
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
pub enum TradeType {
    Buy,
    Sell,
}

impl FromStr for TradeType {
    type Err = ApiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err(ApiError::UnexpectedValue(value.to_string())),
        }
    }
}
